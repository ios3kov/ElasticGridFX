//! Scoped custom corner ownership. Only documented DISABLED UI flags
//! change; persistent Point type/ID/value, keyframes and renderers are untouched.
use super::*;
use std::cell::Cell;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub(super) struct Scope{owner:i32,window:i32,time:i32,scale:u32,mode:i32}
#[derive(Clone,Copy,Default)]
struct Claim{scope:Option<Scope>}
impl Claim{
    fn hover(&mut self,scope:Option<Scope>,corner:bool,held:bool){
        // A held button cannot acquire a new gesture or steal another owner.
        if held {if self.scope!=scope {self.scope=None;}return;}
        self.scope=scope.filter(|_|corner);
    }
    fn matches(self,scope:Option<Scope>)->bool{scope.is_some()&&self.scope==scope}
    fn finish(&mut self,scope:Option<Scope>,corner_release:bool,success:bool)->bool{
        let retain=corner_release&&success&&self.matches(scope);
        if !retain {self.scope=None;}
        retain
    }
}
thread_local!{static CLAIM:Cell<Claim>=const{Cell::new(Claim{scope:None})};}
pub(super) fn scope(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    id:Option<ae::aegp::PluginId>)->Option<Scope>{
    let window=ui::event_window_code(event);
    if window!=ae::sys::PF_Window_COMP&&window!=ae::sys::PF_Window_LAYER{return None;}
    let mode=params.get(Params::PlaneMode).ok()?.as_popup().ok()?.value();
    if mode!=2&&mode!=4{return None;}
    Some(Scope{owner:corner_loupe::owner(input,id).ok()?,window,
        time:input.current_time(),scale:input.time_scale(),mode})
}
pub(crate) fn hover(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    corner:bool,id:Option<ae::aegp::PluginId>){
    let mut claim=CLAIM.get();claim.hover(scope(input,params,event,id),corner,corner_loupe::native_button_down());CLAIM.set(claim);
}
pub(crate) fn claimed(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    id:Option<ae::aegp::PluginId>)->bool{
    CLAIM.get().matches(scope(input,params,event,id))
}
// UPDATE_PARAMS_UI has no viewer context. Retain only this same effect/time/mode
// claim so a parameter-panel refresh cannot re-enable native picking mid-drag.
pub(crate) fn claimed_ui(input:&ae::InData,params:&ae::Parameters<Params>,
    id:Option<ae::aegp::PluginId>)->bool {
    let Some(claim)=CLAIM.get().scope else{return false;};
    let Ok(owner)=corner_loupe::owner(input,id) else{return false;};
    let Ok(param)=params.get(Params::PlaneMode) else{return false;};
    let Ok(mode)=param.as_popup() else{return false;};
    claim.owner==owner&&claim.time==input.current_time()&&claim.scale==input.time_scale()
        &&claim.mode==mode.value()
}
pub(crate) fn finish(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
    id:Option<ae::aegp::PluginId>,corner_release:bool,success:bool)->bool {
    let mut claim=CLAIM.get();
    let retain=claim.finish(scope(input,params,event,id),corner_release,success);
    CLAIM.set(claim);retain
}
pub(crate) fn clear(){CLAIM.set(Claim::default());}

#[cfg(all(test,feature="owned-corner-drag"))]
pub(super) fn test_scope(owner:i32,time:i32,mode:i32)->Scope {
    Scope{owner,window:ae::sys::PF_Window_COMP,time,scale:25,mode}
}

#[cfg(test)]mod tests{
    use super::*;
    fn key()->Scope{Scope{owner:9,window:0,time:25,scale:25,mode:2}}
    #[test]fn repeated_press_without_cursor_motion_keeps_only_same_corner_hover(){
        let mut claim=Claim::default();claim.hover(Some(key()),true,false);
        for _ in 0..3 {assert!(claim.finish(Some(key()),true,true));
            // No new AdjustCursor between release and next press.
            assert!(claim.matches(Some(key())));}
        claim.hover(Some(key()),false,false);assert!(!claim.matches(Some(key())));
        for (scope,corner,success) in [(Some(key()),false,true),(Some(key()),true,false),
            (Some(Scope{owner:99,..key()}),true,true),(None,true,true)]{
            claim.hover(Some(key()),true,false);assert!(!claim.finish(scope,corner,success));
            assert!(claim.scope.is_none());}
    }
    #[test]fn hover_does_not_acquire_a_gesture_from_an_already_held_button(){
        let mut claim=Claim::default();claim.hover(Some(key()),true,true);
        assert!(!claim.matches(Some(key())));
        claim.hover(Some(key()),true,false);assert!(claim.matches(Some(key())));
        claim.hover(Some(key()),false,true);assert!(claim.matches(Some(key())));
        claim.hover(Some(key()),false,false);assert!(!claim.matches(Some(key())));
    }
    #[test]fn other_owner_window_time_scale_mode_and_unknown_context_cannot_inherit_claim(){
        for other in [Scope{owner:10,..key()},Scope{window:1,..key()},Scope{time:26,..key()},
            Scope{scale:30,..key()},Scope{mode:4,..key()}]{
            let mut claim=Claim::default();claim.hover(Some(key()),true,false);
            assert!(!claim.matches(Some(other)));
            claim.hover(Some(other),true,true);assert!(!claim.matches(Some(other)));
        }
        let mut claim=Claim::default();claim.hover(Some(key()),true,false);
        claim.hover(None,true,false);assert!(!claim.matches(None));
    }
}
