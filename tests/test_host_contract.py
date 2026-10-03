"""Source-contract guards, not After Effects runtime tests."""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[1]
SOURCE = (ROOT / 'host-rust/src/lib.rs').read_text()
SETUP = SOURCE.split('    fn params_setup(', 1)[1].split('    fn handle_command(', 1)[0]
LEGACY = ['Columns', 'Rows', 'GridState', 'TensionRadius', 'Falloff',
          'ElasticityStrength', 'MinSpacing', 'StretchEasing', 'EasingDistance',
          'WaveEnabled', 'WaveAmplitude', 'WaveFrequency', 'WavePhase', 'WaveSpeed',
          'WaveAxis', 'EdgeMode', 'Quality']
STAGE9_APPEND = ['PlaneMode', 'PlaneTopLeft', 'PlaneTopRight',
                 'PlaneBottomRight', 'PlaneBottomLeft', 'ResetPlane']
NATIVE_PLANE_APPEND = ['ResearchPlaneTL', 'ResearchPlaneTR', 'ResearchPlaneBR',
                       'ResearchPlaneBL', 'ResearchPlaneKind']


def block(name):
    start = SETUP.index('Params::' + name + ',')
    end = SETUP.index(')?;', start)
    return SETUP[start:end]


class HostContract(unittest.TestCase):
    def test_parameter_ids_keep_legacy_prefix_and_approved_append(self):
        variants = SOURCE.split('pub(crate) enum Params {', 1)[1].split('}', 1)[0]
        self.assertEqual(re.findall(r'\b(\w+)\s*,', variants),
                         LEGACY + STAGE9_APPEND + NATIVE_PLANE_APPEND +
                         ['ResetGridPositions', 'WaveGroupStart', 'WaveGroupEnd', 'AutomaticSpacing', 'ControlLayout', 'VersionRow'])

        # Disk IDs derive from unchanged enum Debug names, not UI registration order.
        direct = re.findall(r'params\.add\w*\(Params::(\w+),', SETUP)
        self.assertEqual(direct, ['PlaneMode', 'ResetPlane'] + LEGACY[:3] +
                         ['ResetGridPositions'] + LEGACY[3:10] + ['WaveGroupStart'] +
                         LEGACY[10:] + ['AutomaticSpacing', 'ControlLayout', 'VersionRow'])
        self.assertIn('Params::WaveGroupStart, Params::WaveGroupEnd, "Wave Animation", true', SETUP)
        self.assertEqual(direct[-1], 'VersionRow')
        for name in STAGE9_APPEND[1:-1]:
            self.assertIn(f'(Params::{name},', SETUP)

    def test_popup_ordinals_describe_current_core_behavior(self):
        self.assertIn('["Layer Plane", "Four Corners"]', block('PlaneMode'))
        self.assertIn('ae::ParamFlag::SUPERVISE', block('PlaneMode'))
        self.assertIn('["Smooth", "Soft", "Even", "Old Smooth"]', block('Falloff'))
        self.assertIn('f.set_default(1)', block('Falloff'))
        # Ordinal 4 intentionally keeps the old Smoothstep fallback, not Cosine.
        bridge = (ROOT / 'src/bridge/elasticgrid_ffi.cpp').read_text()
        mapping = bridge.split('falloff_from_i32', 1)[1].split('wave_axis_from_i32', 1)[0]
        for text in ('case 2: return eg::FalloffProfile::Gaussian',
                     'case 3: return eg::FalloffProfile::Linear',
                     'default: return eg::FalloffProfile::Smoothstep'):
            self.assertIn(text, mapping)

    def test_hidden_arbitrary_layout_has_required_native_ui_flag(self):
        self.assertIn('NO_ECW_UI', block('ControlLayout'))
        self.assertIn('INVISIBLE', block('ControlLayout'))
        self.assertIn('TOPIC', block('VersionRow'))

    def test_hidden_shape_controls_preserve_saved_streams_and_old_fallbacks(self):
        for name in ('Falloff', 'ElasticityStrength', 'StretchEasing', 'EasingDistance'):
            self.assertIn('ae::ParamUIFlags::INVISIBLE', block(name))
        self.assertIn('USE_VALUE_FOR_OLD_PROJECTS', block('Falloff'))
        self.assertIn('f.set_value(2)', block('Falloff'))
        self.assertIn('USE_VALUE_FOR_OLD_PROJECTS', block('StretchEasing'))
        self.assertIn('f.set_value(0.0)', block('StretchEasing'))
        self.assertIn('(0.0, 100.0), (0.0, 100.0), 100.0', block('StretchEasing'))
        for name in ('ElasticityStrength', 'StretchEasing', 'EasingDistance'):
            self.assertNotIn('CANNOT_TIME_VARY', block(name))

    def test_ui_limits_match_renderer(self):
        for name in ('Columns', 'Rows'):
            self.assertIn('f.set_valid_max(MAX_GUIDES as i32)', block(name))
        self.assertIn('(0.0, 20.0)', block('TensionRadius'))
        self.assertNotIn('(0.0, 32.0)', block('TensionRadius'))
        self.assertIn('(0.0, 10.0)', block('WaveFrequency'))
        self.assertNotIn('(0.0, 20.0)', block('WaveFrequency'))
        self.assertIn('(-360.0, 360.0)', block('WavePhase'))

    def test_grid_counts_are_static_but_still_editable_and_supervised(self):
        for name in ('Columns', 'Rows'):
            with self.subTest(parameter=name):
                definition = block(name)
                self.assertIn('ae::ParamFlag::SUPERVISE | ae::ParamFlag::CANNOT_TIME_VARY', definition)
                self.assertIn('ae::ParamUIFlags::empty()', definition)
                self.assertIn('f.set_valid_min(1)', definition)
                self.assertIn('f.set_valid_max(MAX_GUIDES as i32)', definition)
                self.assertIn('f.set_default(4)', definition)
                self.assertNotIn('CANNOT_INTERP', definition)
                self.assertNotIn('CONTROL_ONLY', definition)

    def test_static_choices_and_animated_grid_waves_keep_approved_policy(self):
        static = ('Columns', 'Rows', 'PlaneMode', 'Falloff', 'EdgeMode', 'Quality', 'AutomaticSpacing', 'ControlLayout', 'VersionRow')
        for name in static:
            self.assertIn('CANNOT_TIME_VARY', block(name))
        self.assertEqual(SETUP.count('ae::ParamFlag::CANNOT_TIME_VARY'), len(static))
        self.assertIn('USE_VALUE_FOR_OLD_PROJECTS', block('AutomaticSpacing'))
        self.assertIn('f.set_default(true); f.set_value(false)', block('AutomaticSpacing'))
        for name in ('GridState', 'WaveAmplitude', 'WaveFrequency', 'WavePhase', 'WaveSpeed'):
            self.assertNotIn('CANNOT_TIME_VARY', block(name))
        self.assertNotIn('CANNOT_INTERP', block('GridState'))
        self.assertIn('ae::ParamUIFlags::TOPIC', block('GridState'))
        self.assertNotIn('ae::ParamUIFlags::CONTROL', block('GridState'))

    def test_manual_density_change_requests_redraw_without_grid_sync(self):
        handler = SOURCE.split('ae::Command::UserChangedParam { param_index } => {', 1)[1].split(
            'ae::Command::UpdateParamsUi', 1)[0]
        self.assertIn('params.index(Params::Columns) == Some(param_index)', handler)
        self.assertIn('params.index(Params::Rows) == Some(param_index)', handler)
        self.assertNotIn('sync_grid_topology', handler)
        self.assertIn('out_data.set_out_flag(ae::OutFlags::ForceRerender, true)', handler)
        # Never change registration-only behavior flags during rendering/UI.
        self.assertNotIn('CANNOT_TIME_VARY', handler)

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

    def test_plane_ui_mode_does_not_change_values(self):
        plane = (ROOT / 'host-rust/src/plane.rs').read_text()
        update = plane.split('pub(crate) fn update_ui', 1)[1].split('#[derive', 1)[0]
        self.assertIn('Params::PlaneMode', update)
        self.assertIn('ui_layer_is_3d(input)', update)
        self.assertIn('ae::ParamUIFlags::DISABLED,mode_disabled', update)
        self.assertIn('ae::ParamUIFlags::DISABLED,corners_disabled', update)
        self.assertIn('definition.update_param_ui()', update)
        self.assertNotIn('set_value(', update)
        self.assertIn('ae::Command::UpdateParamsUi => plane::update_ui(&in_data,params)?', SOURCE)

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

    def test_grid_animation_control_visible_without_diagnostic_text(self):
        ui = (ROOT / 'host-rust/src/ui.rs').read_text()
        self.assertNotIn('EGFX-{}', ui)
        grid = SETUP.split('params.add_customized(Params::GridState,',1)[1].split('})?;',1)[0]
        self.assertIn('ae::ParamUIFlags::TOPIC',grid)
        self.assertNotIn('ae::ParamUIFlags::CONTROL',grid)
        self.assertNotIn('ae::ParamUIFlags::NO_ECW_UI',grid)
        self.assertNotIn('ae::ParamUIFlags::INVISIBLE',grid)
        control = ui.split('fn draw_effect_control(',1)[1].split('pub fn draw(',1)[0]
        self.assertNotIn('draw_string(',control)
        self.assertIn('grid_row::draw(event)', control)
        identity=(ROOT/'tools/build_identity.py').read_text()
        self.assertIn('ElasticGridBuildID=',identity)

    def test_branding_preserves_effect_match_name(self):
        build=(ROOT/'host-rust/build.rs').read_text()
        self.assertIn('Property::Name("FSTR Stretch")',build)
        self.assertIn('Property::Category("FSTR Effects")',build)
        self.assertIn('Property::AE_Effect_Match_Name("com.elasticgrid.fx.warp")',build)


if __name__ == '__main__':
    unittest.main()
