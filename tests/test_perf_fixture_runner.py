"""Portable tests for synthetic performance fixture preparation. No AE execution."""
import json
from pathlib import Path
import sys,tempfile,unittest
from unittest.mock import patch
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import perf_fixture_runner as pf
import aerender_benchmark as ab

class Fixture(unittest.TestCase):
 def setUp(self):
  self.tmp=tempfile.TemporaryDirectory(prefix='egfx-perf-fixture-');self.addCleanup(self.tmp.cleanup)
  self.root=Path(self.tmp.name).resolve()
 def test_prepare_is_unique_and_not_run(self):
  a,ma=pf.prepare(self.root,1920,1080,32,'animated');b,mb=pf.prepare(self.root,1920,1080,32,'animated')
  self.assertNotEqual(a,b);self.assertEqual(ma['status'],'NOT RUN');self.assertTrue((a/'pattern.png').is_file())
 def test_prepare_rejects_unsupported_configuration(self):
  for args in [(100,100,32,'animated'),(1920,1080,24,'animated'),(1920,1080,32,'bad')]:
   with self.assertRaises(ValueError):pf.prepare(self.root,*args)
 def test_inspect_builds_exact_benchmark_schema(self):
  folder,meta=pf.prepare(self.root,1920,1080,32,'static')
  (folder/'EGFX_PERF.aep').write_bytes(b'aep')
  capture=dict(run_id=meta['run_id'],status='PREPARED',stage='prepared',saved=True,ae_version='x',
               width=1920,height=1080,fps=30,duration=2,bit_depth=32,mode='static',composition='EGFX_PERF',
               rqindex=1,render_template='Best Settings',output_template='PNG Sequence',
               output_format='PNG Sequence',output_pattern='frame_[#####].png',
               output_precision=16,output_depth='Trillions of Colors+',output_channels='RGB + Alpha',
               output_color='Straight (Unmatted)',working_space='',linearize=False)
  (folder/'capture.json').write_text(json.dumps(capture))
  fixture=pf.inspect(folder,meta)
  self.assertEqual(fixture['quality'],'Final Bicubic');self.assertEqual(fixture['frame_end'],59)
  self.assertEqual(fixture['schema'],2);self.assertEqual(fixture['output_bit_depth'],16)
  self.assertEqual(fixture['output_channels'],'RGBA')
  self.assertEqual(ab.load_fixture(folder,folder/'fixture.json')['project_path'],folder/'EGFX_PERF.aep')
  for field,value in [('output_precision',8),('output_depth','Millions of Colors'),('output_channels','RGB'),
                      ('output_color','Premultiplied (Matted)'),('working_space','sRGB'),('linearize',True)]:
   invalid=dict(capture);invalid[field]=value;(folder/'capture.json').write_text(json.dumps(invalid))
   with self.subTest(field=field),self.assertRaises(ValueError):pf.inspect(folder,meta)
 def test_capture_mismatch_refuses(self):
  folder,meta=pf.prepare(self.root,1920,1080,32,'animated')
  (folder/'EGFX_PERF.aep').write_bytes(b'aep')
  (folder/'capture.json').write_text(json.dumps(dict(run_id=meta['run_id'],status='FAIL')))
  with self.assertRaises(ValueError):pf.inspect(folder,meta)
 def test_execute_refuses_nonmac_before_transport(self):
  folder,meta=pf.prepare(self.root,1920,1080,32,'animated')
  with patch.object(pf.platform,'system',return_value='Linux'),patch.object(pf.subprocess,'run') as run:
   with self.assertRaises(ValueError):pf.execute(folder,meta,self.root/'AE.app',self.root/'plugin',self.root/'p.zip',self.root/'m.json')
   run.assert_not_called()

if __name__=='__main__':unittest.main()
