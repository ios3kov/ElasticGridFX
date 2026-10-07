//! Nondefault native hit-priority probe. Only documented DISABLED UI flags
//! change; persistent Point type/ID/value, keyframes and renderers are untouched.
use super::*;
use std::cell::Cell;

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
struct Scope{owner:i32,window:i32,time:i32,scale:u32,mode:i32}
#[derive(Clone,Copy,Default)]
struct Claim{scope:Option<Scope>}
impl Claim{
    fn hover(&mut self,scope:Option<Scope>,corner:bool,held:bool){
        // A held button cannot acquire a new gesture or steal another owner.
        if held {if self.scope!=scope {self.scope=None;}return;}
        self.scope=scope.filter(|_|corner);
    }
    fn matches(self,scope:Option<Scope>)->bool{scope.is_some()&&self.scope==scope}
}
thread_local!{static CLAIM:Cell<Claim>=const{Cell::new(Claim{scope:None})};}
fn scope(input:&ae::InData,params:&ae::Parameters<Params>,event:&ae::EventExtra,
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
pub(crate) fn clear(){CLAIM.set(Claim::default());}

#[cfg(test)]mod tests{
    use super::*;
    fn key()->Scope{Scope{owner:9,window:0,time:25,scale:25,mode:2}}
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
