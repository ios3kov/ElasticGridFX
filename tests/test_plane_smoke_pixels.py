"""Known image fixtures for the Stage 9 plane comparator; not real AE QA."""
from array import array
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
import smoke_pixels as sp
import plane_smoke_pixels as pp


class PlanePixels(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix='egfx-plane-pixels-')
        cls.addClassCleanup(cls.tmp.cleanup)
        cls.root = Path(cls.tmp.name)
        fixture = cls.root/'pattern.png'
        sp.pattern(fixture, 128, 96)
        cls.base = sp.read_png(fixture)

    @staticmethod
    def shifted(image, pixels):
        amount = 4*pixels
        return sp.Image(image.width, image.height, array('f', image.pixels[amount:]+image.pixels[:amount]))

    def frames(self):
        a = self.shifted(self.base, 5)
        b = self.shifted(self.base, 13)
        c = self.shifted(self.base, 21)
        d = self.shifted(self.base, 29)
        e = self.shifted(self.base, 37)
        frames = {}
        for depth in pp.DEPTHS:
            p = f'd{depth}-'
            frames[p+'original'] = self.base
            frames[p+'identity'] = self.base
            frames[p+'legacy-wave'] = a
            frames[p+'plane-wave'] = a
            frames[p+'skew-wave'] = b
            frames[p+'invalid'] = self.base
            frames[p+'half-identity'] = self.base
            frames[p+'half-original'] = self.base
        frames.update({
            'roundtrip-before': b, 'roundtrip-after': b,
            '3d-base': b, '3d-layer-rotate': c,
            '3d-camera-move': d, '3d-parent': e,
        })
        return frames

    def validate(self, frames):
        with patch.object(pp, 'read_png', side_effect=lambda p: frames[p.stem]):
            return pp.validate_plane_frames(self.root)

    def test_positive_matrix_passes(self):
        result = self.validate(self.frames())
        self.assertEqual(result['status'], 'PASS')
        self.assertEqual(result['frames'], len(pp.FRAMES))
        self.assertEqual(len(result['checks']), 22)

    def test_plane_passthrough_fails(self):
        frames = self.frames()
        for depth in pp.DEPTHS:
            frames[f'd{depth}-plane-wave'] = self.base
            frames[f'd{depth}-legacy-wave'] = self.base
        result = self.validate(frames)
        self.assertEqual(result['status'], 'FAIL')
        self.assertEqual(result['checks']['d8-identity_plane-wave']['status'], 'FAIL')

    def test_roundtrip_change_fails(self):
        frames = self.frames()
        frames['roundtrip-after'] = self.shifted(self.base, 44)
        self.assertEqual(self.validate(frames)['checks']['roundtrip']['status'], 'FAIL')

    def test_frozen_camera_or_parent_fails(self):
        for name, source in (('3d-camera-move','3d-layer-rotate'), ('3d-parent','3d-camera-move')):
            frames = self.frames()
            frames[name] = frames[source]
            self.assertEqual(self.validate(frames)['status'], 'FAIL')

    def test_flat_frame_is_refused(self):
        frames = self.frames()
        frames['3d-base'] = sp.Image(128, 96, array('f', [0.2,0.2,0.2,1.0])*(128*96))
        with self.assertRaises(ValueError):
            self.validate(frames)


if __name__ == '__main__':
    unittest.main()
