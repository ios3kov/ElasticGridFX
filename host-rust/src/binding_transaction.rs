//! Host-independent transaction contract for native-plane dependencies.
//! Adapter must reacquire/validate the exact effect and dedicated hidden streams.
//! Never pass Grid Positions or public Four Corners streams to this interface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub expression: String,
    pub enabled: bool,
    pub keys: usize,
}

pub trait Host {
    // Validate stable target identity, ownership, stream schema and editability.
    // Must fail if deleted, replaced, locked, or no longer the requested effect.
    // Every read/setter must enforce the same identity, not use UI selection.
    fn validate_target(&mut self) -> Result<(), String>;
    fn read(&mut self, index: usize) -> Result<Snapshot, String>;
    fn begin_undo(&mut self) -> Result<(), String>;
    fn end_undo(&mut self) -> Result<(), String>;
    fn expression(&mut self, index: usize, value: &str) -> Result<(), String>;
    fn enable(&mut self, index: usize, value: bool) -> Result<(), String>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome { Installed, AlreadyInstalled }
#[derive(Debug, PartialEq, Eq)]
pub struct Failure {
    pub cause: String,
    pub rollback_failed: bool,
    pub undo_close_failed: bool,
}
impl From<String> for Failure {
    fn from(cause: String) -> Self { Self {cause, rollback_failed:false, undo_close_failed:false} }
}

fn read_all(host: &mut impl Host, count:usize) -> Result<Vec<Snapshot>, String> {
    (0..count).map(|i| host.read(i)).collect()
}

#[cfg(any(test,not(fstr_auto_binding)))]
pub fn install(host: &mut impl Host, expected: &[String]) -> Result<Outcome, Failure> {
    install_or_upgrade(host, expected, &[])
}

// Only a complete, enabled, unkeyed exact previous version may be upgraded.
// Use the same readback, reentrancy and rollback contract as initial binding.
pub fn install_or_upgrade(host: &mut impl Host, expected: &[String], previous: &[String]) -> Result<Outcome, Failure> {
    if expected.is_empty() || expected.len()>16 || expected.iter().any(|s| s.is_empty() || s.contains('\0')) {
        return Err("Invalid binding expressions".to_owned().into());
    }
    host.validate_target()?;
    let before = read_all(host,expected.len())?;
    if before.iter().zip(expected).all(|(s,e)| s.keys==0 && s.enabled && s.expression==*e) {
        return Ok(Outcome::AlreadyInstalled);
    }
    // A partial, disabled, keyed, foreign or newer binding is not ours to repair.
    let owned_previous = previous.len()==expected.len() && before.iter().zip(previous)
        .all(|(s,e)| s.keys==0 && s.enabled && s.expression==*e);
    if !owned_previous && before.iter().any(|s| s.keys!=0 || s.enabled || !s.expression.is_empty()) {
        return Err("Binding conflict; no changes made".to_owned().into());
    }
    host.begin_undo()?;
    let mut touched = 0;
    let operation = (|| -> Result<(), String> {
        host.validate_target()?;
        if read_all(host,expected.len())? != before { return Err("Target changed before writes".to_owned()); }
        for (i, expression) in expected.iter().enumerate() {
            // Include a setter that fails after modifying host state in rollback.
            touched = i+1;
            host.expression(i, expression)?;
            host.enable(i, true)?;
        }
        let after = read_all(host,expected.len())?;
        if !after.iter().zip(expected).all(|(s,e)| s.keys==0 && s.enabled && s.expression==*e) {
            return Err("Binding readback mismatch".to_owned());
        }
        Ok(())
    })();
    let mut rollback_failed = false;
    if operation.is_err() && touched>0 {
        // Never write to a replacement effect if the original was deleted.
        if host.validate_target().is_err() { rollback_failed=true; }
        else {
            for i in (0..touched).rev() {
                // Reentrant host activity must not make recovery overwrite a
                // newly keyed property or a foreign expression.
                let owned = host.read(i).is_ok_and(|s| s.keys==before[i].keys &&
                    (s.expression==before[i].expression || s.expression==expected[i]));
                if !owned { rollback_failed=true; continue; }
                if host.enable(i, false).is_err() { rollback_failed=true; }
                if host.expression(i, &before[i].expression).is_err() { rollback_failed=true; }
                if host.enable(i, before[i].enabled).is_err() { rollback_failed=true; }
            }
            if read_all(host,expected.len()).as_ref() != Ok(&before) { rollback_failed=true; }
        }
    }
    // Attempt closing exactly once, even after failed writes or rollback.
    let close = host.end_undo();
    match (operation, close) {
        (Ok(()), Ok(())) => Ok(Outcome::Installed),
        (result, close) => Err(Failure {
            cause: result.err().unwrap_or_else(|| "Undo close failed after binding".to_owned()),
            rollback_failed, undo_close_failed: close.is_err(),
        }),
    }
}

#[cfg(test)] mod tests {
    use super::*;
    #[derive(Default)] struct Mock {
        states: Vec<Snapshot>, writes: usize, fail_write: Option<usize>,
        fail_reads_after_write: bool, begin_fail: bool, end_fail: bool,
        begins: usize, ends: usize, validations: usize, vanish_at: Option<usize>,
        mutate_at_begin: bool, rollback_fail: bool, foreign_on_failure: bool,
    }
    fn blank() -> Snapshot { Snapshot {expression:String::new(),enabled:false,keys:0} }
    fn fixture() -> Mock { Mock {states:vec![blank();4],..Mock::default()} }
    fn expressions() -> [String;4] { std::array::from_fn(|i| format!("owned-v1-corner-{i}")) }
    impl Mock {
        fn wrote(&mut self) -> Result<(),String> {
            self.writes+=1;
            if self.fail_write==Some(self.writes) || (self.rollback_fail && self.writes>2) {
                if self.foreign_on_failure { self.states[0].expression="new user expression".into(); }
                Err("injected setter failure".into())
            } else { Ok(()) }
        }
    }
    impl Host for Mock {
        fn validate_target(&mut self)->Result<(),String>{
            self.validations+=1;
            if self.vanish_at==Some(self.validations) {Err("deleted".into())} else {Ok(())}
        }
        fn read(&mut self,i:usize)->Result<Snapshot,String>{
            if self.fail_reads_after_write && self.writes>0 {Err("read failed".into())}
            else {Ok(self.states[i].clone())}
        }
        fn begin_undo(&mut self)->Result<(),String>{
            self.begins+=1;
            if self.begin_fail {return Err("begin failed".into());}
            if self.mutate_at_begin {self.states[0].expression="foreign".into();}
            Ok(())
        }
        fn end_undo(&mut self)->Result<(),String>{self.ends+=1;if self.end_fail {Err("end failed".into())} else {Ok(())}}
        fn expression(&mut self,i:usize,v:&str)->Result<(),String>{self.states[i].expression=v.into();self.wrote()}
        fn enable(&mut self,i:usize,v:bool)->Result<(),String>{self.states[i].enabled=v;self.wrote()}
    }
    #[test] fn install_and_repeat_are_idempotent(){
        let mut h=fixture();assert_eq!(install(&mut h,&expressions()),Ok(Outcome::Installed));
        assert_eq!((h.writes,h.begins,h.ends),(8,1,1));
        assert_eq!(install(&mut h,&expressions()),Ok(Outcome::AlreadyInstalled));
        assert_eq!((h.writes,h.begins,h.ends),(8,1,1));
    }
    #[test] fn exact_previous_binding_upgrades_and_rolls_back_on_each_failure(){
        let old=expressions();let mut new=old.clone();new[3]="owned-v2-kind".into();
        let before:Vec<_>=old.iter().map(|e|Snapshot{expression:e.clone(),enabled:true,keys:0}).collect();
        let mut h=Mock{states:before.clone(),..Mock::default()};
        assert_eq!(install_or_upgrade(&mut h,&new,&old),Ok(Outcome::Installed));
        assert_eq!(install_or_upgrade(&mut h,&new,&old),Ok(Outcome::AlreadyInstalled));
        for fail in 1..=8 {
            let mut h=Mock{states:before.clone(),fail_write:Some(fail),..Mock::default()};
            assert!(!install_or_upgrade(&mut h,&new,&old).unwrap_err().rollback_failed);
            assert_eq!(h.states,before);
        }
        for index in 0..4 {
            let mut h=Mock{states:before.clone(),..Mock::default()};h.states[index].keys=1;
            assert!(install_or_upgrade(&mut h,&new,&old).is_err());assert_eq!(h.writes,0);
        }
    }
    #[test] fn five_stream_marker_transaction_rolls_back_all_partial_writes(){
        let expected:Vec<String>=(0..5).map(|i|format!("owned-{i}")).collect();
        for fail in 1..=10 {
            let mut h=Mock {states:vec![blank();5],fail_write:Some(fail),..Mock::default()};
            let error=install(&mut h,&expected).unwrap_err();
            assert!(!error.rollback_failed && !error.undo_close_failed);
            assert_eq!(h.states,vec![blank();5]);
        }
        let mut h=Mock {states:vec![blank();5],..Mock::default()};
        assert_eq!(install(&mut h,&expected),Ok(Outcome::Installed));
        assert_eq!(install(&mut h,&expected),Ok(Outcome::AlreadyInstalled));
        assert_eq!(h.writes,10);
        assert!(install(&mut h,&[]).is_err());
    }
    #[test] fn every_partial_setter_failure_rolls_back_even_if_setter_mutated(){
        for fail in 1..=8 {
            let mut h=fixture();h.fail_write=Some(fail);
            let error=install(&mut h,&expressions()).unwrap_err();
            assert!(!error.rollback_failed && !error.undo_close_failed);
            assert_eq!(h.states,vec![blank();4]);assert_eq!((h.begins,h.ends),(1,1));
        }
    }
    #[test] fn foreign_disabled_partial_and_keyed_bindings_are_preserved(){
        for state in [Snapshot{expression:"foreign".into(),..blank()},
            Snapshot{expression:expressions()[0].clone(),..blank()},
            Snapshot{enabled:true,..blank()},Snapshot{keys:1,..blank()}] {
            let mut h=fixture();h.states[2]=state;let original=h.states.clone();
            assert!(install(&mut h,&expressions()).is_err());
            assert_eq!(h.states,original);assert_eq!((h.writes,h.begins,h.ends),(0,0,0));
        }
    }
    #[test] fn invalid_or_deleted_target_never_starts_writes(){
        for vanish in [1,2] {
            let mut h=fixture();h.vanish_at=Some(vanish);
            assert!(install(&mut h,&expressions()).is_err());assert_eq!(h.writes,0);
            assert_eq!(h.ends,usize::from(vanish==2));
        }
        let mut h=fixture();let mut e=expressions();e[0].clear();
        assert!(install(&mut h,&e).is_err());assert_eq!(h.validations,0);
    }
    #[test] fn change_at_undo_entry_is_not_overwritten(){
        let mut h=fixture();h.mutate_at_begin=true;
        assert!(install(&mut h,&expressions()).is_err());
        assert_eq!(h.states[0].expression,"foreign");assert_eq!((h.writes,h.ends),(0,1));
    }
    #[test] fn undo_failures_are_not_success(){
        let mut h=fixture();h.begin_fail=true;
        assert!(install(&mut h,&expressions()).is_err());assert_eq!((h.writes,h.ends),(0,0));
        let mut h=fixture();h.end_fail=true;
        assert!(install(&mut h,&expressions()).unwrap_err().undo_close_failed);assert_eq!(h.ends,1);
    }
    #[test] fn rollback_failure_and_unverifiable_readback_are_explicit(){
        let mut h=fixture();h.fail_write=Some(2);h.rollback_fail=true;
        assert!(install(&mut h,&expressions()).unwrap_err().rollback_failed);assert_eq!(h.ends,1);
        let mut h=fixture();h.fail_reads_after_write=true;
        assert!(install(&mut h,&expressions()).unwrap_err().rollback_failed);assert_eq!(h.ends,1);
    }
    #[test] fn deleted_target_during_recovery_is_not_written_again(){
        let mut h=fixture();h.fail_write=Some(2);h.vanish_at=Some(3);
        assert!(install(&mut h,&expressions()).unwrap_err().rollback_failed);
        assert_eq!((h.writes,h.ends),(2,1));
    }
    #[test] fn recovery_preserves_reentrant_foreign_expression(){
        let mut h=fixture();h.fail_write=Some(2);h.foreign_on_failure=true;
        assert!(install(&mut h,&expressions()).unwrap_err().rollback_failed);
        assert_eq!(h.states[0].expression,"new user expression");
        assert_eq!((h.writes,h.ends),(2,1));
    }
}
