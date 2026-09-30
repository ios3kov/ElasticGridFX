//! Opt-in, read-only lifecycle experiment. Never shipped in the default build.
//! Enable with RUSTFLAGS='--cfg fstr_lifecycle_probe' (recorded in build identity).
use super::*;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Default)]
pub(crate) struct Probe {
    id: Option<ae::aegp::PluginId>,
    records: Vec<String>,
    journal_entries: u8,
    pending: Arc<AtomicBool>,
}

// One deferred observation per process. No PF effect handle crosses callbacks.
struct Deferred {
    id: ae::aegp::PluginId, pending: Arc<AtomicBool>, consumed: bool,
    basic: *const ae::sys::SPBasicSuite,
}
impl Deferred {
    fn observe(&mut self) {
        if self.consumed || !main_thread() || !self.pending.swap(false, Ordering::AcqRel) { return; }
        self.consumed = true;
        // Non-AEGP hook wrapper does not establish the crate's thread-local
        // suite context. EffectMain's guard has already been dropped at idle.
        // Retain only the host-global suite pointer, never PF_InData/effect_ref.
        if self.basic.is_null() { journal("idle", "idle: missing suite context"); return; }
        let _context = ae::PicaBasicSuite::from_sp_basic_suite_raw(self.basic);
        let result = self.inspect();
        journal("idle", &format!("idle main: {result:?}"));
    }
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
            ae::aegp::suites::Stream::new()?.effect_num_param_streams(effect)
        })();
        let disposed = effects.dispose_effect(effect);
        match (result, disposed) { (Ok(n), Ok(())) => Ok(n), (Err(e), _) | (_, Err(e)) => Err(e) }
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
        // Never acquire AEGP suites off the main thread, including resetup.
        let result = if !main_thread() { "worker: no AEGP calls".to_owned() }
        else if matches!(cmd, ae::Command::SequenceSetup) {
            self.pending.store(true, Ordering::Release);
            "main: deferred; no AEGP calls".to_owned()
        } else { match self.inspect(cmd, input) {
            Ok(n) => format!("main: streams={n}"),
            Err(e) => format!("main: {e:?}"),
        }};
        let record = format!("{label} {result}");
        // Research only, bounded to four non-overwriting, private temp files.
        // No project names, contents, handles or coordinates are recorded.
        #[cfg(target_os = "macos")]
        if self.journal_entries < 4 {
            journal(&self.journal_entries.to_string(), &record);
            self.journal_entries += 1;
        }
        if self.records.len() == 4 { self.records.remove(0); }
        self.records.push(record);
    }

    fn inspect(&mut self, cmd: &ae::Command, input: &ae::InData) -> Result<i32, ae::Error> {
        if matches!(cmd, ae::Command::GlobalSetup) {
            self.id = Some(ae::aegp::suites::Utility::new()?
                .register_with_aegp("com.elasticgrid.fx.warp")?);
            let id = self.id.ok_or(ae::Error::BadCallbackParameter)?;
            ae::aegp::suites::RegisterNonAegp::new()?.register_idle_hook(id,
                Box::new(|state: &mut Deferred, _| { state.observe(); Ok(()) }),
                Deferred { id, pending: self.pending.clone(), consumed: false,
                    basic: input.pica_basic_suite_ptr() })?;
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

    pub fn report(&self) -> String { self.records.join("\r") }
}
