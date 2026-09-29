//! Opt-in, read-only lifecycle experiment. Never shipped in the default build.
//! Enable with RUSTFLAGS='--cfg fstr_lifecycle_probe' (recorded in build identity).
use super::*;

#[derive(Default)]
pub(crate) struct Probe {
    id: Option<ae::aegp::PluginId>,
    records: Vec<String>,
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
        else { match self.inspect(cmd, input) {
            Ok(n) => format!("main: streams={n}"),
            Err(e) => format!("main: {e:?}"),
        }};
        if self.records.len() == 4 { self.records.remove(0); }
        self.records.push(format!("{label} {result}"));
    }

    fn inspect(&mut self, cmd: &ae::Command, input: &ae::InData) -> Result<i32, ae::Error> {
        if matches!(cmd, ae::Command::GlobalSetup) {
            self.id = Some(ae::aegp::suites::Utility::new()?
                .register_with_aegp("com.elasticgrid.fx.warp")?);
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
