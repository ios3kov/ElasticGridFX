"""Wiring regressions; these do not replace native AE key-stream checks."""
from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[1]
HOST = (ROOT/'host-rust/src/lib.rs').read_text()
UI = (ROOT/'host-rust/src/ui.rs').read_text()
BINDING = (ROOT/'host-rust/src/binding_probe.rs').read_text()

class DensityContract(unittest.TestCase):
    def test_counts_never_rewrite_grid_keys(self):
        self.assertNotIn('sync_grid_topology', HOST)
        self.assertNotIn('.resized(', HOST)
        handler = HOST.split('if params.index(Params::Columns) == Some(param_index)', 1)[1].split('ae::Command::UpdateParamsUi', 1)[0]
        for forbidden in ('GridState', 'set_value', 'set_value_changed', 'checkout', 'aegp::'):
            self.assertNotIn(forbidden, handler)
        self.assertIn('ForceRerender', handler)
    def test_render_snapshot_does_not_read_display_counts(self):
        for start, end in [('fn smart_render_snapshot(', '#[repr(C)]'),
                           ('pub(crate) fn grid_snapshot(', 'pub(crate) fn elastic_params(')]:
            text = HOST.split(start, 1)[1].split(end, 1)[0]
            for forbidden in ('Params::Columns', 'Params::Rows', 'topology(', '.resized('):
                self.assertNotIn(forbidden, text)
            self.assertIn('guide_density::render_grid', text)
    def test_pending_identity_is_independent_of_display_counts(self):
        text = BINDING.split('fn pending_frame_identity(', 1)[1].split('pub fn add_params(', 1)[0]
        self.assertNotIn('Params::Columns', text)
        self.assertNotIn('Params::Rows', text)
        self.assertIn('params.checkout(Params::GridState)', text)
    def test_all_viewer_paths_share_readonly_projection(self):
        text = UI.split('fn displayed_grid(', 1)[1].split('thread_local!', 1)[0]
        self.assertIn('guide_density::view_grid', text)
        self.assertNotIn('set_value', text)
        # Drawing, picking and cursor all use the same displayed data.
        self.assertEqual(UI.count('displayed_grid('), 4)
        self.assertIn('let base=evaluated_control_base(in_data,params,&plane)?;', UI)
        self.assertIn('guide_density::drag_control(&mut grid,&base,request,&elastic)', UI)
    def test_drag_cannot_switch_target_when_counts_change(self):
        self.assertIn('event.continue_refcon(2)', UI)
        self.assertIn('event.continue_refcon(3)', UI)
        self.assertIn('guide_density::drag_control', UI)
        self.assertIn('if changed {', UI)
    def test_old_wire_stays_readable_and_counts_are_not_keyed(self):
        self.assertIn('const GRID_WIRE_VERSION: u16 = 3;', HOST)
        self.assertIn('const GRID_DETAIL_WIRE_VERSION: u16 = 4;', HOST)
        self.assertIn('if detailed {8} else {6}', HOST)
        self.assertEqual(HOST.count('ae::ParamFlag::CANNOT_TIME_VARY'), 2)

if __name__ == '__main__': unittest.main()
