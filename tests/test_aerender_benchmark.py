"""Portable contract tests for Stage 8 aerender benchmark tooling. No AE execution."""
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import aerender_benchmark as ab


class BenchmarkTool(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='egfx-perf-')
        self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name).resolve()
        self.project=self.root/'EGFX_PERF.aep'; self.project.write_bytes(b'test-aep')
        self.fixture=self.root/'fixture.json'
        data=dict(schema=1,fixture_id='egfx-perf-v1',project='EGFX_PERF.aep',
                  project_sha256=ab.file_digest(self.project),composition='EGFX_PERF',
                  rqindex=1,width=1920,height=1080,fps=30.0,frame_start=0,frame_end=29,
                  bit_depth=32,quality='Final Bicubic',output_format='PNG sequence',
                  output_pattern='frame_[#####].png')
        self.fixture.write_text(json.dumps(data))

    def test_fixture_is_pinned_and_inside_workspace(self):
        f=ab.load_fixture(self.root,self.fixture)
        self.assertEqual(f['project_path'],self.project)
        self.project.write_bytes(b'changed')
        with self.assertRaises(ValueError): ab.load_fixture(self.root,self.fixture)

    def test_fixture_rejects_nonfinal_or_unsafe_output(self):
        data=json.loads(self.fixture.read_text())
        for key,value in [('quality','Preview'),('output_pattern','../x_[###].png')]:
            changed=dict(data);changed[key]=value;self.fixture.write_text(json.dumps(changed))
            with self.assertRaises(ValueError):ab.load_fixture(self.root,self.fixture)
        self.fixture.write_text(json.dumps(data))

    def test_time_metrics_and_runtime_build_marker(self):
        err='  1.25 real  1.00 user  0.20 sys\n  123456 maximum resident set size\n'
        self.assertEqual(ab.parse_time_metrics(err)['peak_rss_bytes'],123456)
        self.assertEqual(ab.runtime_build_id('', 'x ElasticGridBuildID=EGFX-'+'a'*24,
                                             'EGFX-'+'a'*24),'EGFX-'+'a'*24)
        with self.assertRaises(ValueError):
            ab.runtime_build_id('','', 'EGFX-'+'a'*24)

    def test_output_manifest_is_fresh_regular_data(self):
        out=self.root/'out';out.mkdir()
        (out/'a.png').write_bytes(b'a');(out/'b.png').write_bytes(b'b')
        files,digest=ab.output_manifest(out)
        self.assertEqual(len(files),2);self.assertEqual(len(digest),64)
        (out/'link').symlink_to(out/'a.png')
        with self.assertRaises(ValueError):ab.output_manifest(out)

    def test_summary_requires_five_measured_samples(self):
        sample=lambda n:dict(wall_seconds=float(n),metrics={'peak_rss_bytes':100+n},output_digest=str(n))
        with self.assertRaises(ValueError):ab.summarize([sample(1)]*4)
        result=ab.summarize([sample(n) for n in range(1,6)])
        self.assertEqual(result['samples'],5);self.assertEqual(result['wall_median_seconds'],3.0)

    def test_benchmark_refuses_nonmac_before_execution(self):
        with patch.object(ab.platform,'system',return_value='Linux'), patch.object(ab.subprocess,'run') as run:
            with self.assertRaises(ValueError):
                ab.benchmark(self.root,self.fixture,self.root/'aerender',self.root/'plugin',
                             self.root/'p.zip',self.root/'m.json',2,5,'inherit',60)
            run.assert_not_called()


if __name__=='__main__':unittest.main()
