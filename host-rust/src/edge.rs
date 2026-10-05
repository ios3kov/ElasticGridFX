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
pub(crate) fn update_ui(params:&ae::Parameters<Params>)->Result<(),ae::Error>{
    let value=params.get(Params::EdgeMode)?.as_popup()?.value();
    let mut control=(*params.get(Params::EdgeSelector)?).clone();
    control.as_popup_mut()?.set_value(display(value)?);
    control.update_param_ui()
}
#[cfg(test)] mod tests{
    #[test] fn old_saved_modes_roundtrip_and_none_is_first(){
        for saved in 1..=4{
            let ui=super::display(saved).unwrap();
            assert_eq!(super::stored(ui),Ok(saved));
            assert_eq!(super::DISPLAY[ui as usize-1],super::STORED[saved as usize-1]);
        }
        assert_eq!(super::stored(1),Ok(4));assert!(super::display(0).is_err());
    }
}
