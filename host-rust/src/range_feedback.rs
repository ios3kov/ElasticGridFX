//! Viewer-only scalar marker state; never saved in a project or used by render.
use super::*;
const TAG:ae::sys::A_intptr_t=0x45475231;
type Slots=[ae::sys::A_intptr_t;4];
#[derive(Clone,Copy,Debug,PartialEq)]
pub(crate) struct Anchor {pub column:bool,pub source:f32}
fn fingerprint(grid:&GridArb)->ae::sys::A_intptr_t {
    let mut hash=0xcbf29ce484222325u64;
    for value in [grid.columns as u32,grid.rows as u32]
        .into_iter().chain(grid.column_lines.iter().chain(&grid.row_lines).map(|v|v.to_bits()))
        .chain(grid.column_pins.iter().chain(&grid.row_pins).map(|&v|u32::from(v))) {
        hash^=u64::from(value);hash=hash.wrapping_mul(0x100000001b3);
    }
    hash as ae::sys::A_intptr_t
}
pub(crate) fn record(grid:&GridArb,anchor:Anchor)->Slots {
    if !anchor.source.is_finite() || anchor.source<=0.0 || anchor.source>=1.0{return [0;4];}
    [TAG,if anchor.column {1}else{2},anchor.source.to_bits() as _,fingerprint(grid)]
}
fn decode(grid:&GridArb,slots:Slots)->Option<Anchor> {
    if slots[0]!=TAG || ![1,2].contains(&slots[1]) || slots[3]!=fingerprint(grid){return None;}
    let source=f32::from_bits(u32::try_from(slots[2]).ok()?);
    (source.is_finite() && source>0.0 && source<1.0).then_some(Anchor{column:slots[1]==1,source})
}
fn fallback(grid:&GridArb)->Anchor {
    let mut anchor=Anchor{column:true,source:0.5};let mut largest=0.0;
    for (column,lines) in [(true,&grid.column_lines),(false,&grid.row_lines)] {
        if lines.len()<3 {continue;}
        for (i,&v) in lines.iter().enumerate().skip(1).take(lines.len()-2) {
            let source=i as f32/(lines.len()-1) as f32;
            let delta=(v-source).abs();
            if delta.is_finite() && delta>largest {largest=delta;anchor=Anchor{column,source};}
        }
    }
    anchor
}
fn context(event:&ae::EventExtra)->Option<*mut ae::sys::PF_Context> {
    if !ui::known_window(ui::event_window_code(event)){return None;}
    let handle=event.as_ref().contextH;
    if handle.is_null(){return None;}
    // SAFETY: this handle is borrowed exclusively for its owning callback;
    // no caller stores the returned pointer beyond this function's immediate use.
    let p=unsafe{*handle};
    if p.is_null() || unsafe{(*p).magic}!=0x05ea771e{return None;}
    Some(p)
}
pub(crate) fn write(event:&ae::EventExtra,slots:Slots) {
    if let Some(p)=context(event) {
        // SAFETY: SDK25.6 explicitly reserves these four scalar slots for plugin
        // data. No host-owned drawing/transform/reserved field is changed.
        unsafe{(*p).plugin_state=slots;}
    }
}
pub(crate) fn clear(event:&ae::EventExtra){write(event,[0;4]);}
pub(crate) fn anchor(event:&ae::EventExtra,grid:&GridArb)->Anchor {
    let marker=context(event).and_then(|p| {
        // SAFETY: owning recognized callback only; scalar copy, no retained handle.
        decode(grid,unsafe{(*p).plugin_state})
    });marker.unwrap_or_else(||fallback(grid))
}
pub(crate) fn bounds(anchor:Anchor,radius:f32,side:usize)->Option<(f32,f32)> {
    if !(3..=MAX_GUIDES+2).contains(&side) || !radius.is_finite() || !anchor.source.is_finite() ||
        !(0.0..=1.0).contains(&anchor.source){return None;}
    let width=radius.clamp(0.0,20.0)/(side-1) as f32;
    Some(((anchor.source-width).max(0.0),(anchor.source+width).min(1.0)))
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn scalar_marker_cannot_follow_a_different_saved_shape() {
        let g=GridArb::default();let a=Anchor{column:false,source:0.35};let slots=record(&g,a);
        assert_eq!(decode(&g,slots),Some(a));let mut edited=g.clone();edited.row_lines[1]+=0.01;
        assert_eq!(decode(&edited,slots),None);assert_eq!(decode(&g,[0;4]),None);
        for invalid in [f32::NAN,f32::INFINITY,0.0,1.0] {
            assert_eq!(decode(&g,record(&g,Anchor{column:true,source:invalid})),None);
        }
    }
    #[test] fn marker_range_is_bounded_and_neutral_reference_is_explicit() {
        let g=GridArb::default();assert_eq!(fallback(&g),Anchor{column:true,source:0.5});
        for side in 3..=MAX_GUIDES+2 {for radius in [0.0,1.0,3.0,20.0] {
            let (lo,hi)=bounds(Anchor{column:true,source:0.35},radius,side).unwrap();
            assert!(lo>=0.0 && hi<=1.0 && lo<=0.35 && hi>=0.35);
        }}assert_eq!(bounds(fallback(&g),f32::NAN,6),None);
    }
    #[test] fn boundary_sampling_uses_the_real_fractional_core() {
        let lines=[0.0,0.1,0.7,1.0];
        assert_eq!(control_grid::position_at(&lines,0.0,0.0,0.25).unwrap().to_bits(),0.0f32.to_bits());
        assert_eq!(control_grid::position_at(&lines,1.0,0.0,0.25).unwrap().to_bits(),1.0f32.to_bits());
        assert!((control_grid::position_at(&lines,0.5,0.0,0.25).unwrap()-0.4).abs()<1e-6);
        for easing in [0.0,0.7,1.0] {
            let mut last=0.0;for i in 0..=100 {
                let v=control_grid::position_at(&lines,i as f32/100.0,easing,0.25).unwrap();
                assert!(v.is_finite() && v>=last);last=v;
            }
        }
        assert!(control_grid::position_at(&lines,f32::NAN,0.0,0.25).is_err());
        assert!(control_grid::position_at(&[0.0,0.7,0.2,1.0],0.5,0.0,0.25).is_err());
    }
}
