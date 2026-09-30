//! Main-thread lifecycle bridge for the default native-plane feature.
//! Legacy module/cfg names retain reproducible opt-in research baselines.
use super::*;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Default)]
pub(crate) struct Probe {
    id: Option<ae::aegp::PluginId>,
    records: Vec<String>,
    journal_entries: u8,
    pending: Arc<AtomicBool>,
    wake:Option<IdleWake>,
}

// Adobe explicitly permits this cached function on worker threads, but not
// suite acquisition there. Keep the suite acquired for the global lifetime.
struct IdleWake {
    call:unsafe extern "C" fn()->ae::sys::A_Err,
    release:unsafe extern "C" fn(*const std::ffi::c_char,i32)->ae::sys::SPErr,
}
impl IdleWake {
    fn new(input:&ae::InData)->Result<Self,ae::Error>{
        let basic=input.pica_basic_suite_ptr();
        if basic.is_null() || !main_thread() {return Err(ae::Error::BadCallbackParameter);}
        let basic=unsafe {&*basic};
        let acquire=basic.AcquireSuite.ok_or(ae::Error::MissingSuite)?;
        let release=basic.ReleaseSuite.ok_or(ae::Error::MissingSuite)?;
        let mut ptr:*const c_void=std::ptr::null();
        let code=unsafe {acquire(ae::sys::kAEGPUtilitySuite.as_ptr().cast(),ae::sys::kAEGPUtilitySuiteVersion6 as i32,&mut ptr)};
        if code!=0 {return Err(ae::Error::MissingSuite);}
        let call=if ptr.is_null() {None} else {unsafe {(*ptr.cast::<ae::sys::AEGP_UtilitySuite6>()).AEGP_CauseIdleRoutinesToBeCalled}};
        if let Some(call)=call {Ok(Self{call,release})} else {
            unsafe {release(ae::sys::kAEGPUtilitySuite.as_ptr().cast(),ae::sys::kAEGPUtilitySuiteVersion6 as i32)};
            Err(ae::Error::MissingSuite)
        }
    }
    fn request(&self){
        let code=unsafe {(self.call)()};
        if code!=0 {journal("wake-error",&format!("Idle wake failed: {code}"));}
    }
}
impl Drop for IdleWake {
    fn drop(&mut self){unsafe {(self.release)(ae::sys::kAEGPUtilitySuite.as_ptr().cast(),ae::sys::kAEGPUtilitySuiteVersion6 as i32);}}
}

// One deferred observation per process. No PF effect handle crosses callbacks.
struct Deferred {
    id: ae::aegp::PluginId, pending: Arc<AtomicBool>, consumed: bool,
    basic: *const ae::sys::SPBasicSuite,
    observations:u8,
}
impl Deferred {
    fn observe(&mut self) {
        if (self.consumed && !cfg!(fstr_auto_binding)) || !main_thread() || !self.pending.swap(false, Ordering::AcqRel) { return; }
        self.consumed = true;
        // Non-AEGP hook wrapper does not establish the crate's thread-local
        // suite context. EffectMain's guard has already been dropped at idle.
        // Retain only the host-global suite pointer, never PF_InData/effect_ref.
        if self.basic.is_null() { journal("idle", "idle: missing suite context"); return; }
        let _context = ae::PicaBasicSuite::from_sp_basic_suite_raw(self.basic);
        let result = self.inspect();
        if self.observations<16 {
            journal(&format!("idle-{}",self.observations), &format!("idle main: {result:?}"));
            self.observations+=1;
        }
    }
    #[cfg(not(fstr_auto_binding))]
    fn inspect(&self) -> Result<i32, ae::Error> {
        let layers = ae::aegp::suites::Layer::new()?;
        let layer = layers.active_layer()?.ok_or(ae::Error::BadCallbackParameter)?;
        // Research fixture only; never inspect arbitrary active user effects.
        if layers.layer_name(layer, self.id)?.0 != "__EGFX_TEST_TEXT" {
            return Err(ae::Error::BadCallbackParameter);
        }
        let comp = layers.layer_parent_comp(layer)?;
        let item = ae::aegp::suites::Comp::new()?.item_from_comp(comp)?;
        let name = ae::aegp::suites::Item::new()?.item_name(item, self.id)?;
        if name != "__EGFX_TEXT_b7c354d8976942c1ad5d9088d9222f30" {
            return Err(ae::Error::BadCallbackParameter);
        }
        let effects = ae::aegp::suites::Effect::new()?;
        if effects.layer_num_effects(layer)? != 2 { return Err(ae::Error::BadCallbackParameter); }
        let effect = effects.layer_effect_by_index(layer, self.id, 1)?;
        let result = (|| {
            let key = effects.installed_key_from_layer_effect(effect)?;
            if effects.effect_match_name(key)? != "com.elasticgrid.fx.warp" {
                return Err(ae::Error::BadCallbackParameter);
            }
            #[cfg(fstr_binding_probe)] {
                let binding = binding_probe::run(self.id,effect,layer,self.basic);
                journal("binding", &format!("binding main: {binding:?}"));
                if binding.is_err() { return Err(ae::Error::BadCallbackParameter); }
            }
            ae::aegp::suites::Stream::new()?.effect_num_param_streams(effect)
        })();
        let disposed = effects.dispose_effect(effect);
        match (result, disposed) { (Ok(n), Ok(())) => Ok(n), (Err(e), _) | (_, Err(e)) => Err(e) }
    }

    // No active selection dependency and no effect handles retained across idle.
    // Only exact match-name/schema streams are writable; conflicts remain intact.
    #[cfg(fstr_auto_binding)]
    fn inspect(&self) -> Result<i32,ae::Error> {
        let projects=ae::aegp::suites::Project::new()?;
        let items=ae::aegp::suites::Item::new()?;
        let comps=ae::aegp::suites::Comp::new()?;
        let layers=ae::aegp::suites::Layer::new()?;
        let effects=ae::aegp::suites::Effect::new()?;
        let mut installed=0;
        let mut visited=0;
        let mut failed=false;
        for index in 0..projects.num_projects()? {
            let project=projects.project_by_index(index)?;
            let mut next=Some(items.first_proj_item(&project)?);
            while let Some(item)=next {
                visited+=1;
                if visited>50000 {return Err(ae::Error::BadCallbackParameter);}
                next=items.next_proj_item(&project,item)?;
                if items.item_type(item)?!=ae::aegp::ItemType::Comp {continue;}
                let Some(comp)=comps.comp_from_item(item)? else {continue;};
                for li in 0..layers.comp_num_layers(comp)? {
                    let layer=layers.comp_layer_by_index(comp,li)?;
                    if layers.layer_flags(layer)?.contains(ae::aegp::LayerFlags::LOCKED) {continue;}
                    for ei in 0..effects.layer_num_effects(layer)? {
                        visited+=1;
                        if visited>50000 {return Err(ae::Error::BadCallbackParameter);}
                        let effect=effects.layer_effect_by_index(layer,self.id,ei)?;
                        let result=(|| ->Result<(),ae::Error>{
                            let key=effects.installed_key_from_layer_effect(effect)?;
                            if effects.effect_match_name(key)?!="com.elasticgrid.fx.warp" {return Ok(());}
                            match binding_probe::bind(self.id,effect,layer,self.basic) {
                                Ok(binding_transaction::Outcome::Installed)=>installed+=1,
                                Ok(binding_transaction::Outcome::AlreadyInstalled)=>{},
                                Err(_)=>failed=true,
                            }
                            Ok(())
                        })();
                        let disposed=effects.dispose_effect(effect);
                        result?;disposed?;
                    }
                }
            }
        }
        if failed {Err(ae::Error::BadCallbackParameter)} else {Ok(installed)}
    }
}

fn journal(index: &str, record: &str) {
    #[cfg(target_os = "macos")] {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;
        let path = std::env::temp_dir().join(format!("fstr-lifecycle-{}-{index}.txt", std::process::id()));
        if let Ok(mut f) = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(path) {
            let _ = writeln!(f, "{}\n{}", build_identity::ABOUT, record);
        }
    }
}

#[cfg(target_os = "macos")]
fn main_thread() -> bool {
    unsafe extern "C" { fn pthread_main_np() -> i32; }
    unsafe { pthread_main_np() != 0 }
}
#[cfg(not(target_os = "macos"))]
fn main_thread() -> bool { false }

impl Probe {
    pub fn observe(&mut self, cmd: &ae::Command, input: &ae::InData) {
        let label = match cmd {
            ae::Command::GlobalSetup => "global",
            ae::Command::SequenceSetup => "setup",
            ae::Command::SequenceResetup => "resetup",
            _ => return,
        };
        // Host creation can arrive on a worker. Scheduling is just an atomic
        // notification; only the later main-thread idle callback uses AEGP.
        // Do not enqueue worker resetup: MFR uses it for render-side copies.
        if matches!(cmd,ae::Command::SequenceSetup) {
            self.pending.store(true,Ordering::Release);
            if let Some(wake)=&self.wake {wake.request();}
        }
        // Never acquire AEGP suites off the main thread, including resetup.
        let result = if !main_thread() { "worker: no AEGP calls".to_owned() }
        else if matches!(cmd, ae::Command::SequenceSetup) {
            self.pending.store(true, Ordering::Release);
            "main: deferred; no AEGP calls".to_owned()
        } else {
            #[cfg(fstr_auto_binding)]
            if matches!(cmd,ae::Command::SequenceResetup) {
                self.pending.store(true,Ordering::Release);
                if let Some(wake)=&self.wake {wake.request();}
            }
            match self.inspect(cmd, input) {
            Ok(n) => format!("main: streams={n}"),
            Err(e) => format!("main: {e:?}"),
        }};
        let record = format!("{label} {result}");
        // Research only, bounded to four non-overwriting, private temp files.
        // No project names, contents, handles or coordinates are recorded.
        #[cfg(target_os = "macos")]
        if self.journal_entries < 16 {
            journal(&self.journal_entries.to_string(), &record);
            self.journal_entries += 1;
        }
        if self.records.len() == 4 { self.records.remove(0); }
        self.records.push(record);
    }

    fn inspect(&mut self, cmd: &ae::Command, input: &ae::InData) -> Result<i32, ae::Error> {
        if matches!(cmd, ae::Command::GlobalSetup) {
            self.wake=Some(IdleWake::new(input)?);
            self.id = Some(ae::aegp::suites::Utility::new()?
                .register_with_aegp("com.elasticgrid.fx.warp")?);
            let id = self.id.ok_or(ae::Error::BadCallbackParameter)?;
            ae::aegp::suites::RegisterNonAegp::new()?.register_idle_hook(id,
                Box::new(|state: &mut Deferred, _| { state.observe(); Ok(()) }),
                Deferred { id, pending: self.pending.clone(), consumed: false,
                    basic: input.pica_basic_suite_ptr(),observations:0 })?;
            return Ok(0);
        }
        let id = self.id.ok_or(ae::Error::BadCallbackParameter)?;
        let suite = ae::aegp::suites::Effect::new()?;
        let effect = ae::aegp::suites::PFInterface::new()?
            .new_effect_for_effect(input.effect_ref(), id)?;
        let result = ae::aegp::suites::Stream::new()
            .and_then(|s| s.effect_num_param_streams(effect));
        let disposed = suite.dispose_effect(effect);
        match (result, disposed) { (Ok(n), Ok(())) => Ok(n), (Err(e), _) | (_, Err(e)) => Err(e) }
    }

    #[cfg(not(feature="native-plane"))]
    pub fn report(&self) -> String { self.records.join("\r") }
}

#[cfg(test)] mod wake_tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    static CALLS:AtomicUsize=AtomicUsize::new(0);
    static RELEASES:AtomicUsize=AtomicUsize::new(0);
    unsafe extern "C" fn call()->ae::sys::A_Err {CALLS.fetch_add(1,Ordering::SeqCst);0}
    unsafe extern "C" fn release(_: *const std::ffi::c_char,_:i32)->ae::sys::SPErr {RELEASES.fetch_add(1,Ordering::SeqCst);0}
    #[test] fn cached_wake_is_worker_callable_and_suite_released_once(){
        let wake=IdleWake {call,release};
        std::thread::scope(|s| {s.spawn(||wake.request()).join().unwrap();});
        assert_eq!(CALLS.load(Ordering::SeqCst),1);
        assert_eq!(RELEASES.load(Ordering::SeqCst),0);
        drop(wake);assert_eq!(RELEASES.load(Ordering::SeqCst),1);
    }
}
