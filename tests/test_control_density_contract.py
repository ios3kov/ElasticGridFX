"""Source-wiring guards, not a substitute for the target AE acceptance test."""
from pathlib import Path
import unittest
ROOT = Path(__file__).resolve().parents[1]
HOST = (ROOT / 'host-rust/src/lib.rs').read_text()
UI = (ROOT / 'host-rust/src/ui.rs').read_text()
BINDING = (ROOT / 'host-rust/src/binding_probe.rs').read_text()

class DensityContract(unittest.TestCase):
    def test_count_handler_never_writes_animated_state(self):
        handler = HOST.split('if params.index(Params::Columns) == Some(param_index)', 1)[1].split('ae::Command::UpdateParamsUi', 1)[0]
        for word in ('sync_grid_topology', 'get_mut(', 'set_value(', 'set_value_changed(', 'keyframe', 'aegp::'):
            self.assertNotIn(word, handler)
        self.assertIn('ForceRerender', handler)

    def test_render_snapshot_ignores_density(self):
        smart = HOST.split('fn smart_render_snapshot(', 1)[1].split('#[repr(C)]', 1)[0]
        normal = HOST.split('pub(crate) fn grid_snapshot(', 1)[1].split('pub(crate) fn elastic_params', 1)[0]
        proof = BINDING.split('fn pending_frame_identity(', 1)[1].split('pub fn add_params', 1)[0]
        for section in (smart, normal, proof):
            for word in ('Params::Columns', 'Params::Rows', '.resized(', 'set_value('):
                self.assertNotIn(word, section)
        for section in (smart, normal):
            self.assertIn('retained_grid(', section)

    def test_count_change_cannot_reset_an_axis(self):
        self.assertNotIn('fn sync_grid_topology(', HOST)
        self.assertNotIn('fn resized(', HOST)

    def test_draw_pick_drag_share_same_layout(self):
        self.assertEqual(UI.count('control_grid::read(in_data,params)?'), 4)
        self.assertIn('control_grid::drag_live(lines,pins,refs[index],target,&elastic,&render,axis==DRAG_COLUMNS)', UI)
        drag = UI.split('fn drag_inner(', 1)[1].split('pub fn adjust_cursor(', 1)[0]
        self.assertIn('let render = evaluated_params(params,*in_data,&grid)?;', drag)
        self.assertNotIn('control_grid::drag(', drag)
        self.assertIn('if grid != before {', UI)
        self.assertIn('event.continue_refcon(2) != displayed.grid.columns', UI)
        self.assertIn('event.continue_refcon(3) != displayed.grid.rows', UI)

if __name__ == '__main__': unittest.main()
