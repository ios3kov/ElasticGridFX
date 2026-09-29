"""Source-contract guards, not After Effects runtime tests."""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
SOURCE = (ROOT / 'host-rust/src/lib.rs').read_text()
SETUP = SOURCE.split('    fn params_setup(', 1)[1].split('    fn handle_command(', 1)[0]
EXPECTED = ['Columns', 'Rows', 'GridState', 'TensionRadius', 'Falloff',
            'ElasticityStrength', 'MinSpacing', 'StretchEasing', 'EasingDistance',
            'WaveEnabled', 'WaveAmplitude', 'WaveFrequency', 'WavePhase', 'WaveSpeed',
            'WaveAxis', 'EdgeMode', 'Quality']


def block(name):
    start = SETUP.index('Params::' + name + ',')
    end = SETUP.index(')?;', start)
    return SETUP[start:end]


class HostContract(unittest.TestCase):
    def test_parameter_ids_order_and_count_stay_frozen(self):
        variants = SOURCE.split('pub(crate) enum Params {', 1)[1].split('}', 1)[0]
        self.assertEqual(re.findall(r'\b(\w+)\s*,', variants), EXPECTED)
        self.assertEqual(re.findall(r'params\.add\w*\(Params::(\w+),', SETUP), EXPECTED)

    def test_popup_ordinals_describe_current_core_behavior(self):
        self.assertIn('["Smoothstep", "Gaussian", "Linear", "Smoothstep (Legacy)"]', block('Falloff'))
        self.assertIn('f.set_default(2)', block('Falloff'))
        # Ordinal 4 intentionally keeps the old Smoothstep fallback, not Cosine.
        bridge = (ROOT / 'src/bridge/elasticgrid_ffi.cpp').read_text()
        mapping = bridge.split('falloff_from_i32', 1)[1].split('wave_axis_from_i32', 1)[0]
        for text in ('case 2: return eg::FalloffProfile::Gaussian',
                     'case 3: return eg::FalloffProfile::Linear',
                     'default: return eg::FalloffProfile::Smoothstep'):
            self.assertIn(text, mapping)

    def test_ui_limits_match_renderer(self):
        for name in ('Columns', 'Rows'):
            self.assertIn('f.set_valid_max(MAX_GUIDES as i32)', block(name))
        self.assertIn('(0.0, 20.0)', block('TensionRadius'))
        self.assertNotIn('(0.0, 32.0)', block('TensionRadius'))
        self.assertIn('(0.0, 10.0)', block('WaveFrequency'))
        self.assertNotIn('(0.0, 20.0)', block('WaveFrequency'))
        self.assertIn('(-360.0, 360.0)', block('WavePhase'))

    def test_unused_wave_slot_remains_serialized_but_hidden(self):
        legacy = block('WaveEnabled')
        self.assertIn('ae::CheckBoxDef::setup', legacy)
        self.assertIn('ae::ParamUIFlags::INVISIBLE', legacy)
        # Preserve existing amplitude-controlled rendering and cached snapshots.
        self.assertEqual(SOURCE.count('wave_enabled: 1,'), 2)
        self.assertIn('wave_is_time_varying(true, amplitude, speed)', SOURCE)

    def test_wire_version_remains_three(self):
        self.assertIn('const GRID_WIRE_VERSION: u16 = 3;', SOURCE)
        self.assertIn('const GRID_REFCON: u64 = 0x4547_4658_4752_4944;', SOURCE)

    def test_smartfx_uses_sparse_logical_canvas_and_handles_empty_input(self):
        smart = SOURCE.split('ae::Command::SmartRender { extra } => {', 1)[1].split(
            '#[cfg(target_os = "macos")]', 1
        )[0]
        self.assertIn('eg_render_frame_sparse(', SOURCE)
        self.assertIn('render_sparse(input.as_ref(), &mut output, &p)?;', smart)
        self.assertIn('let input = cb.checkout_layer_pixels(0)?;', smart)
        self.assertNotIn('let Some(input) = cb.checkout_layer_pixels(0)? else', smart)
        self.assertIn('let checkin = cb.checkin_layer_pixels(0);', smart)
        pre = SOURCE.split('ae::Command::SmartPreRender { mut extra } => {', 1)[1].split(
            'ae::Command::SmartRender { extra } => {', 1
        )[0]
        self.assertIn('extra.set_max_result_rect(canvas_rect);', pre)
        self.assertNotIn('max_rect.union(&input_max)', pre)

    def test_custom_control_has_readable_short_build_id(self):
        ui = (ROOT / 'host-rust/src/ui.rs').read_text()
        self.assertIn('EGFX-{}', ui)
        self.assertIn('raw_id[..short_len]', ui)
        self.assertIn('param.set_ui_height(32)', SETUP)


if __name__ == '__main__':
    unittest.main()
