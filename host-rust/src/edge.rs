//! Presentation order independent of the existing saved edge ordinals.
use super::*;
pub(crate) const STORED:[&str;4]=["Clamp","Wrap","Mirror","None"];
pub(crate) const DISPLAY:[&str;4]=["None","Clamp","Wrap","Mirror"];
pub(crate) fn stored(display:i32)->Result<i32,ae::Error>{
    match display{1=>Ok(4),2..=4=>Ok(display-1),_=>Err(ae::Error::BadCallbackParameter)}
}
pub(crate) fn display(stored:i32)->Result<i32,ae::Error>{
    match stored{1..=3=>Ok(stored+1),4=>Ok(1),_=>Err(ae::Error::BadCallbackParameter)}
}
pub(crate) fn locked(mode:i32)->Result<bool,ae::Error>{
    Ok(matches!(plane::Mode::from_value(mode)?,plane::Mode::Layer|plane::Mode::Perspective))
}
// Preserve the serialized choice; only the evaluated render/UI value is fixed.
pub(crate) fn effective(mode:i32,saved:i32)->Result<i32,ae::Error>{
    display(saved)?;
    Ok(if locked(mode)? {1} else {saved})
}
pub(crate) fn update_ui(params:&ae::Parameters<Params>)->Result<(),ae::Error>{
    let mode=params.get(Params::PlaneMode)?.as_popup()?.value();
    let value=params.get(Params::EdgeMode)?.as_popup()?.value();
    let mut control=(*params.get(Params::EdgeSelector)?).clone();
    control.set_ui_flag(ae::ParamUIFlags::DISABLED,locked(mode)?);
    control.as_popup_mut()?.set_value(display(effective(mode,value)?)?);
    control.update_param_ui()
}
#[cfg(test)] mod tests{
    #[test] fn fixed_modes_clamp_without_overwriting_saved_choice(){
        for saved in 1..=4{
            for mode in [3,4]{assert_eq!(super::effective(mode,saved),Ok(1));}
            for mode in [1,2]{assert_eq!(super::effective(mode,saved),Ok(saved));}
        }
        assert!(super::effective(0,1).is_err());
        assert!(super::effective(3,0).is_err());
    }
    #[test] fn old_saved_modes_roundtrip_and_none_is_first(){
        for saved in 1..=4{
            let ui=super::display(saved).unwrap();
            assert_eq!(super::stored(ui),Ok(saved));
            assert_eq!(super::DISPLAY[ui as usize-1],super::STORED[saved as usize-1]);
        }
        assert_eq!(super::stored(1),Ok(4));assert!(super::display(0).is_err());
    }
}
