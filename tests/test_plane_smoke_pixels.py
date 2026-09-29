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
        projected = sp.Image(128,96,array('f',[0.0])*(128*96*4))
        for x,y,u,v in pp.perspective_coordinates():
            if 0<=u<127 and 0<=v<95:
                ix=int(u); iy=int(v); t=u-ix
                for channel in range(4):
                    projected.pixels[(y*128+x)*4+channel]=(1-t)*self.base.pixels[(iy*128+ix)*4+channel]+t*self.base.pixels[(iy*128+ix+1)*4+channel]
        for depth in pp.DEPTHS:
            p = f'd{depth}-'
            frames[p+'original'] = self.base
            frames[p+'identity'] = self.base
            frames[p+'skew-identity'] = projected
            frames[p+'legacy-wave'] = a
            frames[p+'plane-wave'] = a
            frames[p+'skew-wave'] = b
            frames[p+'invalid'] = self.base
            frames[p+'half-identity'] = self.base
            frames[p+'half-original'] = self.base
        f = self.shifted(self.base, 45)
        g = self.shifted(self.base, 53)
        h = self.shifted(self.base, 61)
        i = self.shifted(self.base, 69)
        frames.update({
            'roundtrip-before': b, 'roundtrip-after': b,
            '3d-base': b, '3d-position': c, '3d-scale': d,
            '3d-layer-rotate': e, '3d-camera-move': f, '3d-parent': g,
            '3d-camera-switch': h, '3d-no-camera': i,
        })
        return frames

    def validate(self, frames):
        with patch.object(pp, 'read_png', side_effect=lambda p: frames[p.stem]):
            return pp.validate_plane_frames(self.root)

    def test_positive_matrix_passes(self):
        result = self.validate(self.frames())
        self.assertEqual(result['status'], 'PASS')
        self.assertEqual(result['frames'], len(pp.FRAMES))
        self.assertEqual(len(result['checks']), 29)

    def test_zero_wave_stationary_image_and_mask_are_rejected(self):
        self.assertEqual(pp.check_zero_wave_projection(self.base,self.base)['status'],'FAIL')
        masked=sp.Image(128,96,array('f',self.base.pixels))
        for x,y,u,v in pp.perspective_coordinates():
            if not (0<=u<=127 and 0<=v<=95):
                masked.pixels[(y*128+x)*4+3]=0
        self.assertEqual(pp.check_zero_wave_projection(self.base,masked)['status'],'FAIL')

    def test_inverse_oracle_maps_known_quad_corners(self):
        points={(x,y):(u,v) for x,y,u,v in pp.perspective_coordinates()}
        for dest,source in [((10,5),(0,0)),((120,0),(127,0)),((127,85),(127,95)),((0,95),(0,95))]:
            for actual,expected in zip(points[dest],source):
                self.assertAlmostEqual(actual,expected,places=8)

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

    def test_frozen_3d_or_camera_state_fails(self):
        for name, source in (
            ('3d-position','3d-base'), ('3d-scale','3d-position'),
            ('3d-layer-rotate','3d-scale'), ('3d-camera-move','3d-layer-rotate'),
            ('3d-parent','3d-camera-move'), ('3d-camera-switch','3d-parent'),
            ('3d-no-camera','3d-camera-switch'),
        ):
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
