"""Source wiring guards only; Rust/FFI and real AE tests are separate gates."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
BINDING = (ROOT / 'host-rust/src/binding_probe.rs').read_text()
HOST = (ROOT / 'host-rust/src/lib.rs').read_text()

class FirstApplicationContracts(unittest.TestCase):
    def test_exception_is_limited_to_pending_automatic_frame(self):
        self.assertIn('kind == 0.0 && frame_context && cfg!(fstr_auto_binding)', BINDING)
        self.assertIn('&& pending_frame_identity(params, checkout)?', BINDING)
        self.assertIn('resolve_comp_space_kind(kind,frame_context,cfg!(fstr_auto_binding),initial_identity)?', BINDING)
        self.assertIn('0.0 if automatic && frame_context=>Err(ae::Error::BadCallbackParameter)', BINDING)

    def test_identity_proof_checks_out_smartfx_dependencies(self):
        proof=BINDING.split('fn pending_frame_identity(',1)[1].split('pub fn add_params(',1)[0]
        for token in ['checked_float(params,id)',
                      'checked_popup(params,Params::PlaneMode)?', 'params.checkout(Params::GridState)?',
                      'i32::from(grid.columns)','i32::from(grid.rows)','Params::WaveAmplitude',
                      'Params::StretchEasing','Params::MinSpacing']:
            self.assertIn(token,proof)
        self.assertNotIn('Params::Columns',proof)
        self.assertNotIn('Params::Rows',proof)
        self.assertNotIn('.resized(',proof)
        self.assertNotIn('unwrap_or',proof)
        self.assertNotIn('set_value(',proof)
        self.assertNotIn('aegp::',proof)

    def test_exact_grid_only_and_no_projection_changes(self):
        proof=BINDING.split('fn initial_identity_values(',1)[1].split('fn pending_frame_identity(',1)[0]
        self.assertIn('topology != (4,4)',proof)
        self.assertIn('grid == &initial',proof)
        self.assertIn('a.to_bits()==b.to_bits()',proof)
        self.assertNotIn('epsilon',proof.lower())
        self.assertNotIn('.abs()',proof)
        self.assertIn('setup_float(f,(0.0,7.0),(0.0,7.0),0.0,0,false)',BINDING)
        self.assertIn('install_or_upgrade',BINDING)

    def test_no_live_parameters_in_smart_render(self):
        smart=HOST.split('ae::Command::SmartRender { extra } => {',1)[1].split('#[cfg(target_os = "macos")]',1)[0]
        self.assertIn('pre_render_data::<SmartRenderSnapshot>()',smart)
        self.assertNotIn('pending_frame_identity',smart)
        self.assertNotIn('params.get(',smart)
        self.assertNotIn('params.checkout(',smart)

    def test_host_fixture_never_suppresses_initial_error_or_repairs_binding(self):
        source=(ROOT/'tests/ae_first_application.jsx').read_text()
        for forbidden in ['beginSuppressDialogs(', 'executeCommand(', '.purge(', 'setExpression(',
                          'DO_NOT_SAVE_CHANGES', 'scheduleTask(', 'app.quit(']:
            self.assertNotIn(forbidden,source)
        self.assertIn('elasticGridHasTestProjectOwnership',source)
        self.assertIn('root.getFiles().length !== 0',source)
        self.assertIn("record('before-add')",source)
        self.assertIn("capture('first-neutral')",source)
        self.assertIn("return 'CAPTURED_NOT_FULL_ACCEPTANCE'",source)

if __name__ == '__main__':
    unittest.main()
