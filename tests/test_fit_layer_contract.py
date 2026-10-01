"""Source integration checks, NOT live After Effects/keyframe tests."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
SOURCE = (ROOT / 'host-rust/src/lib.rs').read_text()
FIT_PATH = ROOT / 'host-rust/src/fit_layer.rs'


class FitLayerContract(unittest.TestCase):
    def test_fit_uses_boundary_helper_not_last_pixel_index(self):
        handler = SOURCE.split('if params.index(Params::ResetPlane) == Some(param_index) {', 1)[1].split(
            'if params.index(Params::Columns)', 1)[0]
        self.assertIn('fit_layer::apply(&in_data, params)?', handler)
        self.assertNotIn('saturating_sub(1)', handler)
        self.assertNotIn('GridState', handler)
        self.assertNotIn('set_value', handler)

    def test_ui_fit_reads_full_source_dimensions_only(self):
        self.assertTrue(FIT_PATH.exists())
        source = FIT_PATH.read_text()
        self.assertIn('points(input.width(), input.height())?', source)
        self.assertNotIn('pre_effect_source_origin', source)
        self.assertNotIn('downsample_', source)
        self.assertNotIn('checkout', source)
        self.assertNotIn('GridState', source)
        self.assertNotIn('Params::Columns', source)
        self.assertNotIn('Params::Rows', source)
        self.assertIn('if changed[i]', source)
        self.assertEqual(source.count('set_value_changed()'), 1)

    def test_initial_percent_points_and_corner_order_stay_unchanged(self):
        for name, pair in [('PlaneTopLeft', '(0.0, 0.0)'), ('PlaneTopRight', '(100.0, 0.0)'),
                           ('PlaneBottomRight', '(100.0, 100.0)'), ('PlaneBottomLeft', '(0.0, 100.0)')]:
            self.assertIn(f'(Params::{name}, ', SOURCE)
            line = next(s for s in SOURCE.splitlines() if f'(Params::{name}, ' in s)
            self.assertIn(pair, line)
        plane = (ROOT / 'host-rust/src/plane.rs').read_text()
        self.assertIn('Params::PlaneTopLeft, Params::PlaneTopRight,\n    Params::PlaneBottomRight, Params::PlaneBottomLeft', plane)

    def test_render_and_ui_keep_explicit_coordinate_domains(self):
        # Prevent the tempting global removal of size-1 from raster sampling.
        cpp = (ROOT / 'src/bridge/plane_ffi.cpp').read_text()
        self.assertIn('static_cast<double>(f->canvas_width)-1', cpp)
        self.assertIn('static_cast<double>(f->canvas_height)-1', cpp)
        plane = (ROOT / 'host-rust/src/plane.rs').read_text()
        self.assertIn('if frame_context {in_data.pre_effect_source_origin()}', plane)
        self.assertNotIn('fit_layer::', plane)
        render = SOURCE.split('ae::Command::Render {', 1)[1]
        self.assertNotIn('fit_layer::', render)


if __name__ == '__main__':
    unittest.main()
