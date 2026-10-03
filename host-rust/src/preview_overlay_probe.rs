//! Test-only public PF_CustomEFlag_PREVIEW callback observation, no drawing.
use after_effects as ae;
use std::{fs::{DirBuilder,File,OpenOptions},io::{self,Write},path::Path,
    sync::{Mutex,OnceLock},time::{SystemTime,UNIX_EPOCH}};
#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt,OpenOptionsExt};

const LIMIT:u32=4096;
static LOG:OnceLock<Option<Mutex<Log>>>=OnceLock::new();
struct Log {file:File,count:u32,failed:bool}
impl Log {
    fn create(folder:&Path)->io::Result<Self> {
        let mut d=DirBuilder::new();
        #[cfg(unix)] d.mode(0o700);
        d.create(folder)?; // Exclusive: refuse stale or symlinked output.
        let mut o=OpenOptions::new();o.write(true).create_new(true);
        #[cfg(unix)] o.mode(0o600);
        let mut file=o.open(folder.join("events.csv"))?;
        writeln!(file,"# schema=2,build_id={},limit={LIMIT},scope=callback observation only",
            super::build_identity::BUILD_ID)?;
        writeln!(file,"sequence,utc_ms,event,window,time_numerator,time_scale,mouse_h,mouse_v,hit_axis,hit_index,failed")?;
        Ok(Self{file,count:0,failed:false})
    }
    fn append(&mut self,event:i32,window:i32,time:i32,scale:u32,mouse:Option<(i32,i32)>,trace:(isize,isize,i32)) {
        if self.failed||self.count>=LIMIT{return;}
        self.count+=1;
        let ms=SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
        let (h,v)=mouse.unwrap_or((i32::MIN,i32::MIN));
        let (axis,index,failed)=trace;
        self.failed=writeln!(self.file,"{},{ms},{event},{window},{time},{scale},{h},{v},{},{},{failed}",self.count,axis,index)
            .and_then(|_|self.file.flush()).is_err();
    }
}
fn recognized(window:i32)->bool {
    [ae::sys::PF_Window_COMP,ae::sys::PF_Window_LAYER,
        ae::sys::PF_Window_EFFECT].contains(&window)
}
pub(crate) fn observe(input:&ae::InData,event:&ae::EventExtra)->bool {
    let raw=event.as_ref();
    // SDK25.6 AE_EffectUI.h supplies PF_ContextH. Read it only in its owning
    // callback; no pointer/context survives the callback or reaches a log.
    let window=super::ui::event_window_code(event);
    let logger=LOG.get_or_init(|| {
        let folder=std::env::temp_dir().join(format!("egfx-preview-probe-{}",std::process::id()));
        Log::create(&folder).ok().map(Mutex::new)
    });
    if let Some(log)=logger {
        if let Ok(mut log)=log.lock() {
            let mouse=if recognized(window) && [ae::sys::PF_Event_DO_CLICK,ae::sys::PF_Event_DRAG,ae::sys::PF_Event_ADJUST_CURSOR].contains(&raw.e_type) {
                let p=event.screen_point();Some((p.h,p.v))
            } else {None};
            log.append(raw.e_type,window,input.current_time(),input.time_scale(),mouse,(-2,-2,-1));
        }
    }
    !recognized(window)
}
// Called only inside owning recognized viewer callbacks. No context is retained.
pub(crate) fn interaction(input:&ae::InData,event:&ae::EventExtra,hit:(isize,isize),failed:bool) {
    if let Some(Some(logger))=LOG.get() {if let Ok(mut log)=logger.lock() {
        let p=event.screen_point();
        log.append(100+event.as_ref().e_type,-1,input.current_time(),input.time_scale(),
            Some((p.h,p.v)),(hit.0,hit.1,i32::from(failed)));
    }}
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn preview_and_null_context_never_enter_normal_ui() {
        assert!(!recognized(ae::sys::PF_Window_PREVIEW));
        assert!(!recognized(ae::sys::PF_Window_NONE));
        assert!(!recognized(99));
        for w in 0..3 {assert!(recognized(w));}
        assert_eq!(ae::CustomEventFlags::PREVIEW.bits(),8);
    }
    #[test] fn exclusive_bounded_log_refuses_stale_paths() {
        let p=std::env::temp_dir().join(format!("egfx-preview-unit-{}-{}",std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));
        let mut log=Log::create(&p).unwrap();
        assert!(Log::create(&p).is_err());
        for _ in 0..LIMIT+2 {log.append(4,3,6,30,None,(-2,-2,-1));}
        drop(log);
        let s=std::fs::read_to_string(p.join("events.csv")).unwrap();
        assert_eq!(s.lines().count(),LIMIT as usize+2);
        assert!(s.lines().nth(2).unwrap().ends_with(",-2147483648,-2147483648,-2,-2,-1"));
        assert!(s.lines().last().unwrap().starts_with("4096,"));
        std::fs::remove_file(p.join("events.csv")).unwrap();std::fs::remove_dir(p).unwrap();
    }
}
