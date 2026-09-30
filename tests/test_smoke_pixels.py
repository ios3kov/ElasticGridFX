"""Known image/control-flow fixtures; these do not execute the effect in AE."""
from array import array
import json
from pathlib import Path
import struct
import sys
import tempfile
import unittest
from unittest.mock import patch
import zlib

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import smoke_pixels as sp
import ae_smoke_runner as runner


class SmokePixels(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix='egfx-pixels-')
        cls.addClassCleanup(cls.tmp.cleanup)
        cls.root = Path(cls.tmp.name).resolve()
        fixture = cls.root/'pattern.png'
        sp.pattern(fixture)
        cls.base = sp.read_png(fixture)
        cls.changed = sp.Image(319,241,array('f',cls.base.pixels[4*5:]+cls.base.pixels[:4*5]))
        cls.other = sp.Image(319,241,array('f',cls.base.pixels[4*13:]+cls.base.pixels[:4*13]))

    def validate(self, frames):
        with patch.object(sp,'read_png',side_effect=lambda p:self.base if p.stem=='pattern' else frames[p.stem]):
            return sp.validate_frames(self.root)

    def frames(self):
        moved=sp.Image(319,241,array('f',self.other.pixels))
        for i in range(319*241):
            if sp.corner_coverage(i%319+0.5,i//319+0.5)==-1:
                moved.pixels[i*4+3]=0
        return dict(bypass=self.base,identity=self.base,static_a=self.changed,static_b=self.changed,
                    animated_a=self.changed,animated_b=self.other,reset=self.base,
                    chain_before_corner=self.changed,chain_corner_identity=self.changed,
                    chain_corner_moved=moved)

    def test_positive_fixture_passes_direct_and_corner_pin_checks(self):
        result=self.validate(self.frames())
        self.assertEqual(result['status'],'PASS')
        self.assertEqual(len(result['checks']),7)

    def test_passthrough_fails_despite_all_valid_pngs(self):
        frames=self.frames()
        for name in sp.FRAMES:
            if name!='chain_corner_moved': frames[name]=self.base
        result=self.validate(frames)
        self.assertEqual(result['status'],'FAIL')
        self.assertEqual(result['checks']['identity_static_a']['status'],'FAIL')

    def test_frozen_animation_fails(self):
        frames=self.frames(); frames['animated_b']=frames['animated_a']
        self.assertEqual(self.validate(frames)['status'],'FAIL')

    def test_export_scale_requires_exact_bypass_calibration(self):
        frames=self.frames()
        frames={n:sp.Image(im.width,im.height,array('f',(v*0.1 if i%4!=3 else v for i,v in enumerate(im.pixels)))) for n,im in frames.items()}
        self.assertEqual(self.validate(frames)['export_rgb_scale'],0.1)
        frames['bypass'].pixels[0]+=0.01
        with self.assertRaises(ValueError): self.validate(frames)

    def test_corner_interior_hole_and_opaque_exterior_fail(self):
        for index,value in ((120*319+160,0),(0,1)):
            frames=self.frames(); frames['chain_corner_moved'].pixels[index*4+3]=value
            with self.assertRaises(ValueError): self.validate(frames)

    def test_identity_corruption_and_failed_reset_fail(self):
        for name in ('identity','reset'):
            frames=self.frames(); frames[name]=self.changed
            self.assertEqual(self.validate(frames)['status'],'FAIL')

    def test_nonstatic_zero_speed_fails(self):
        frames=self.frames(); frames['static_b']=self.other
        self.assertEqual(self.validate(frames)['status'],'FAIL')

    def test_corner_pin_identity_black_frame_is_rejected(self):
        frames=self.frames()
        frames['chain_corner_identity']=sp.Image(319,241,array('f',[0,0,0,1])*(319*241))
        with self.assertRaises(ValueError):
            self.validate(frames)

    def test_corner_pin_identity_must_not_change_pixels(self):
        frames=self.frames(); frames['chain_corner_identity']=self.other
        result=self.validate(frames)
        self.assertEqual(result['status'],'FAIL')
        self.assertEqual(result['checks']['chain_before_corner_chain_corner_identity']['status'],'FAIL')

    def test_flat_colored_or_transparent_frame_is_rejected(self):
        for rgba in ([0,0.5,1,1],[0.2,0.5,0.9,0]):
            frames=self.frames(); frames['static_a']=sp.Image(319,241,array('f',rgba)*(319*241))
            with self.assertRaises(ValueError): self.validate(frames)

    def test_decoder_filters_depth_and_crc(self):
        for depth in (8,16):
            for filter_kind in range(5):
                bpp=4*(depth//8)
                rows=[bytes((17*x+31*y)%256 for x in range(3*bpp)) for y in range(2)]
                encoded=bytearray(); previous=bytes(3*bpp)
                for row in rows:
                    encoded.append(filter_kind)
                    for i,value in enumerate(row):
                        a=row[i-bpp] if i>=bpp else 0; b=previous[i]; c=previous[i-bpp] if i>=bpp else 0
                        p=a+b-c; distances=[abs(p-a),abs(p-b),abs(p-c)]
                        paeth=[a,b,c][distances.index(min(distances))]
                        predictor=[0,a,b,(a+b)//2,paeth][filter_kind]
                        encoded.append((value-predictor)&255)
                    previous=row
                data=sp.SIGNATURE+sp.chunk(b'IHDR',struct.pack('>IIBBBBB',3,2,depth,6,0,0,0))+sp.chunk(b'IDAT',zlib.compress(encoded))+sp.chunk(b'IEND',b'')
                file=self.root/'decoder.png'; file.write_bytes(data)
                image=sp.read_png(file)
                raw=b''.join(rows); expected=list(raw) if depth==8 else struct.unpack('>'+'H'*(len(raw)//2),raw)
                for actual,want in zip(image.pixels,expected): self.assertAlmostEqual(actual,want/(255 if depth==8 else 65535),places=6)
                file.write_bytes(data[:-5]+b'xxxxx')
                with self.assertRaises(ValueError): sp.read_png(file)

    def test_truncated_and_oversized_deflate_are_rejected(self):
        file=self.root/'bad.png'
        for payload in (b'',b'\x00'*10000):
            file.write_bytes(sp.SIGNATURE+sp.chunk(b'IHDR',struct.pack('>IIBBBBB',1,1,8,6,0,0,0))+sp.chunk(b'IDAT',zlib.compress(payload))+sp.chunk(b'IEND',b''))
            with self.assertRaises(ValueError): sp.read_png(file)

    def test_unsupported_interlace_fails_not_pass(self):
        file=self.root/'unsupported.png'
        file.write_bytes(sp.SIGNATURE+sp.chunk(b'IHDR',struct.pack('>IIBBBBB',1,1,8,6,0,0,1))+sp.chunk(b'IDAT',zlib.compress(bytes(5)))+sp.chunk(b'IEND',b''))
        with self.assertRaises(ValueError): sp.read_png(file)

    def test_preparation_unique_and_stale_capture_rejected(self):
        first,meta=runner.prepare(self.root/'runs',{})
        second,_=runner.prepare(self.root/'runs',{})
        self.assertNotEqual(first,second)
        self.assertEqual(meta['status'],'NOT RUN')
        self.assertIsNone(meta['loaded_build_id'])
        (first/'capture.json').write_text(json.dumps(dict(run_id='old',status='CAPTURED',project_bpc=32,ae_version='fixture')))
        with self.assertRaises(ValueError): runner.inspect_capture(first,meta)

    def test_failed_capture_cannot_be_promoted(self):
        folder,meta=runner.prepare(self.root/'failed-runs',{})
        (folder/'capture.json').write_text(json.dumps(dict(run_id=meta['run_id'],status='FAIL',project_bpc=32,ae_version='fixture')))
        with self.assertRaises(ValueError): runner.inspect_capture(folder,meta)


if __name__ == '__main__': unittest.main()
