//! Nondefault native validation of documented UI event render deferral.
//! No render-worker override, saved data, quality setter or extra frame request.
use super::{ae,corner_loupe,ui};
use std::cell::Cell;

#[derive(Clone,Copy,PartialEq,Eq)]
struct Key {owner:i32,window:i32,time:i32,scale:u32}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Action {Keep,Suspend,Resume}
#[derive(Clone,Copy,Default)]
struct State {held:Option<Key>}
impl State {
    fn observe(&mut self,key:Key,active:bool)->Action {
        if active {self.held=Some(key);Action::Suspend}
        else if self.held.take()==Some(key) {Action::Resume}
        else {Action::Keep}
    }
}
thread_local! {static STATE:Cell<State>=const{Cell::new(State{held:None})};}
fn flags(current:ae::EventOutFlags,action:Action)->ae::EventOutFlags {
    match action {
        Action::Keep=>current,
        Action::Suspend=>(current&!ae::EventOutFlags::ALWAYS_UPDATE)|ae::EventOutFlags::NEVER_UPDATE,
        Action::Resume=>(current&!ae::EventOutFlags::NEVER_UPDATE)|
            ae::EventOutFlags::ALWAYS_UPDATE|ae::EventOutFlags::UPDATE_NOW,
    }
}
pub(crate) fn event(input:&ae::InData,event:&mut ae::EventExtra,id:Option<ae::aegp::PluginId>) {
    // Closed/changed contexts do not retain host pointers or a suppression flag.
    if matches!(event.event(),ae::Event::CloseContext|ae::Event::NewContext|ae::Event::Deactivate) {
        STATE.set(State::default());return;
    }
    if !matches!(event.window_type(),ae::WindowType::Comp|ae::WindowType::Layer) ||
       !matches!(event.event(),ae::Event::Click(_)|ae::Event::Drag(_)|ae::Event::Draw(_)|ae::Event::AdjustCursor(_)) {
        return;
    }
    let Some((owner,active))=corner_loupe::policy_context(input,event,id) else {
        STATE.set(State::default());return;
    };
    let key=Key{owner,window:ui::event_window_code(event),time:input.current_time(),scale:input.time_scale()};
    let mut state=STATE.get();let action=state.observe(key,active);STATE.set(state);
    if action==Action::Keep {return;}
    if action==Action::Resume {
        // Consume the release transition before invalidating: the subsequent
        // DRAW must not enqueue another render. Fail open if invalidation fails.
        if ae::pf::suites::App::new().and_then(|app|app.invalidate_rect(event.context_handle(),None)).is_err() {
            event.set_event_out_flags(ae::EventOutFlags::from_bits_retain(event.as_ref().evt_out_flags)|
                ae::EventOutFlags::ALWAYS_UPDATE);return;
        }
    }
    // SDK25.6 AE_EffectUI.h:505-510. Only UI event response flags change.
    let current=ae::EventOutFlags::from_bits_retain(event.as_ref().evt_out_flags);
    event.set_event_out_flags(flags(current,action));
}

#[cfg(test)]
mod tests {
    use super::*;
    fn key()->Key {Key{owner:4,window:1,time:9,scale:25}}
    #[test]fn release_is_once_and_repress_is_independent() {
        let mut state=State::default();let key=key();
        assert_eq!(state.observe(key,false),Action::Keep);
        assert_eq!(state.observe(key,true),Action::Suspend);
        for _ in 0..1000 {assert_eq!(state.observe(key,true),Action::Suspend);}
        assert_eq!(state.observe(key,false),Action::Resume);
        for _ in 0..1000 {assert_eq!(state.observe(key,false),Action::Keep);}
        assert_eq!(state.observe(key,true),Action::Suspend);
        assert_eq!(state.observe(key,false),Action::Resume);
    }
    #[test]fn changed_scope_never_resumes_another_context() {
        for other in [Key{owner:5,..key()},Key{window:2,..key()},Key{time:10,..key()},Key{scale:30,..key()}] {
            let mut state=State::default();state.observe(key(),true);
            assert_eq!(state.observe(other,false),Action::Keep);
            assert_eq!(state.observe(key(),false),Action::Keep);
            state.observe(other,true);
            assert_eq!(state.observe(other,false),Action::Resume);
        }
    }
    #[test]fn update_flags_are_exclusive_and_unrelated_flags_retained() {
        let original=ae::EventOutFlags::HANDLED_EVENT|ae::EventOutFlags::ALWAYS_UPDATE|ae::EventOutFlags::UPDATE_NOW;
        let original_bits=original.bits();
        let suspended=flags(original,Action::Suspend);
        assert!(suspended.contains(ae::EventOutFlags::NEVER_UPDATE|ae::EventOutFlags::HANDLED_EVENT|ae::EventOutFlags::UPDATE_NOW));
        assert!(!suspended.contains(ae::EventOutFlags::ALWAYS_UPDATE));
        let resumed=flags(suspended,Action::Resume);
        assert!(!resumed.contains(ae::EventOutFlags::NEVER_UPDATE));
        assert_eq!(resumed.bits(),original_bits);
        assert_eq!(flags(ae::EventOutFlags::from_bits_retain(original_bits),Action::Keep).bits(),original_bits);
    }
}
