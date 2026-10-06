//! Synchronous AE-scheduled row strips; no host calls from worker callbacks.
use super::{ae,dispatch_render,render_result,Frame,Image,RenderKind,Report,State};

const ROWS:i32=16;
const TASKS:i32=4;
const MIN_PIXELS:i64=256*1024;

#[allow(clippy::manual_is_multiple_of)] // keep Rust1.85 MSRV
fn extent(image:&Image,bytes:i32)->Option<(usize,usize)> {
    if image.width<=0 || image.height<=0 || image.pixels.is_null() ||
        image.row_bytes<i64::from(image.width).checked_mul(4*i64::from(bytes))? as isize ||
        image.row_bytes%bytes as isize!=0 || image.pixels as usize%bytes as usize!=0 {
        return None;
    }
    let length=(image.height as usize-1).checked_mul(image.row_bytes as usize)?
        .checked_add(image.width as usize*4*bytes as usize)?;
    if length>isize::MAX as usize {return None;}
    Some((image.pixels as usize,(image.pixels as usize).checked_add(length)?))
}

pub(super) fn eligible(src:&Image,dst:&Image,depth:i32,frame:&Frame,state:&State)->bool {
    let bytes=match depth {8=>1,16=>2,32=>4,_=>return false};
    if i64::from(dst.width)*i64::from(dst.height)<MIN_PIXELS {return false;}
    let Some((d0,d1))=extent(dst,bytes) else {return false;};
    if !(src.width==0 && src.height==0) {
        let Some((s0,s1))=extent(src,bytes) else {return false;};
        if s0<d1 && d0<s1 {return false;}
    }
    if frame.output_y.checked_add(dst.height-1).is_none() {return false;}
    let c=frame.corners;
    // Preserve the existing fast separable axis cache. Perspective is projected
    // even when its target happens to be rectangular.
    state.render_kind==RenderKind::Perspective ||
        !(c[1]==c[3] && c[2]==c[4] && c[5]==c[7] && c[6]==c[0])
}

pub(super) struct Job<'a> {
    src:Image,dst:Image,depth:i32,frame:Frame,sampling:(i32,i32),state:&'a State,
}

// SAFETY: constructed only for synchronous scheduling with immutable source,
// axes and State; destination partitions contain distinct rows, each scheduled
// once. All callbacks join before run returns and worlds/axes are released.
// This does not make arbitrary Image/Frame pointers thread-safe.
unsafe impl Sync for Job<'_> {}

impl<'a> Job<'a> {
    // SAFETY: valid separate source/output allocations and axes outlive run;
    // positive output dimensions/stride with no address/output-origin overflow.
    // The scheduler executes each strip once and joins before returning.
    pub(super) unsafe fn new(src:Image,dst:Image,depth:i32,mut frame:Frame,
        sampling:(i32,i32),state:&'a State)->Self {
        // The input effect_ref/abort bridge is valid on the render caller only.
        frame.abort_fn=None;frame.abort_refcon=std::ptr::null_mut();
        Self {src,dst,depth,frame,sampling,state}
    }

    pub(super) fn run(&self,mut poll:impl FnMut()->Result<(),ae::Error>,
        mut schedule:impl FnMut(i32,&(dyn Fn(i32)->Result<(),ae::Error>+Sync))->Result<(),ae::Error>)
        ->Result<(),ae::Error> {
        let mut first=0;
        while first<self.dst.height {
            poll()?;
            let remaining=self.dst.height-first;
            let count=((remaining-1)/ROWS+1).min(TASKS);
            let callback=|i| {
                // The wrapper's C callback does not catch panics. Contain ours
                // here before control can cross that ABI boundary.
                contain(||self.strip(first,i,count))
            };
            schedule(count,&callback)?;
            poll()?;
            first+=remaining.min(ROWS*TASKS);
        }
        Ok(())
    }

    fn strip(&self,first:i32,i:i32,count:i32)->Result<(),ae::Error> {
        if !(0..count).contains(&i) {return Err(ae::Error::BadCallbackParameter);}
        let row=first.checked_add(i.checked_mul(ROWS).ok_or(ae::Error::BadCallbackParameter)?)
            .ok_or(ae::Error::BadCallbackParameter)?;
        let height=ROWS.min(self.dst.height-row);
        if row<0 || height<=0 {return Err(ae::Error::BadCallbackParameter);}
        let offset=(row as usize).checked_mul(self.dst.row_bytes as usize)
            .filter(|v|*v<=isize::MAX as usize).ok_or(ae::Error::BadCallbackParameter)?;
        // SAFETY: eligible validates dimensions/positive stride/address bounds;
        // source and destination are separate checked-out worlds. This strip's
        // rows remain in the full output allocation and do not overlap peers.
        let pixels=unsafe {self.dst.pixels.cast::<u8>().add(offset)}.cast();
        let dst=Image {pixels,height,..self.dst};
        let frame=Frame {output_y:self.frame.output_y.checked_add(row)
            .ok_or(ae::Error::BadCallbackParameter)?,..self.frame};
        render_result(dispatch_render(&self.src,&dst,self.depth,&frame,
            &mut Report::default(),self.sampling,self.state))
    }
}

fn contain(callback:impl FnOnce()->Result<(),ae::Error>)->Result<(),ae::Error> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback))
        .unwrap_or(Err(ae::Error::InternalStructDamaged))
}

#[cfg(test)]
#[path="plane_row_tiles_tests.rs"]
mod tests;
