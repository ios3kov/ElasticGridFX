"""Portable contract tests for 0.9.4 aerender benchmark tooling. No AE execution."""
import json
from pathlib import Path
import sys
import tempfile
import subprocess
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import aerender_benchmark as ab


class BenchmarkTool(unittest.TestCase):
    def test_mfr_command_includes_required_cpu_percentage(self):
        fixture={'project_path':self.project,'rqindex':1,'output_pattern':'frame_[#####].png'}
        result=subprocess.CompletedProcess([],0,'ElasticGridBuildID=EGFX-'+'a'*24,
            ' 1.0 real 0.5 user 0.1 sys\n 12345 maximum resident set size\n')
        with patch.object(ab.subprocess,'run',return_value=result) as run, patch.object(ab,'output_manifest',return_value=([{'size':1}],'digest')):
            ab.run_once(1,'sample',self.root,Path('/test/aerender'),fixture,'EGFX-'+'a'*24,'on',60)
        self.assertEqual(run.call_args.args[0][-3:],['-mfr','ON','100'])

    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='egfx-perf-')
        self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name).resolve()
        self.project=self.root/'EGFX_PERF.aep'; self.project.write_bytes(b'test-aep')
        self.fixture=self.root/'fixture.json'
        data=dict(schema=2,fixture_id='egfx-perf-v2',project='EGFX_PERF.aep',
                  project_sha256=ab.file_digest(self.project),composition='EGFX_PERF',
                  rqindex=1,width=1920,height=1080,fps=30.0,frame_start=0,frame_end=29,
                  bit_depth=32,quality='Final Bicubic',output_format='PNG sequence',
                  output_pattern='frame_[#####].png',geometry='grid',
                  color_management='none-linearize-off')
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

    def test_legacy_fixture_schema_remains_readable(self):
        data=json.loads(self.fixture.read_text())
        data.pop('geometry');data.pop('color_management');data['schema']=1
        self.fixture.write_text(json.dumps(data))
        loaded=ab.load_fixture(self.root,self.fixture)
        self.assertEqual(loaded['geometry'],'grid')
        self.assertEqual(loaded['color_management'],'legacy-unspecified')

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

    def test_summary_requires_five_samples_and_stable_outputs(self):
        sample=lambda n,d='same':dict(wall_seconds=float(n),metrics={'peak_rss_bytes':100+n},output_digest=d)
        with self.assertRaises(ValueError):ab.summarize([sample(1)]*4,30)
        result=ab.summarize([sample(n) for n in range(1,6)],30)
        self.assertEqual(result['samples'],5);self.assertEqual(result['wall_median_seconds'],3.0)
        self.assertEqual(result['output_digest'],'same')
        self.assertAlmostEqual(result['ms_per_frame_median'],100.0)
        with self.assertRaises(ValueError):ab.summarize([sample(n,str(n)) for n in range(1,6)],30)

    def test_environment_summary_accepts_valid_version_with_nonzero_exit(self):
        good=subprocess.CompletedProcess([],15,'aerender version 25.6x101\n','')
        with patch.object(ab.subprocess,'run',return_value=good):
            env=ab.environment_summary(Path('/test/aerender'))
        self.assertEqual(env['aerender_version'],'aerender version 25.6x101')
        self.assertEqual(env['aerender_version_query_exit_code'],15)

        bad=subprocess.CompletedProcess([],0,'aerender version 25.6x101\naerender SYNTAX ERROR: bad flag\n','')
        with patch.object(ab.subprocess,'run',return_value=bad):
            with self.assertRaisesRegex(ValueError,'version query failed'):
                ab.environment_summary(Path('/test/aerender'))

    def test_benchmark_refuses_nonmac_before_execution(self):
        with patch.object(ab.platform,'system',return_value='Linux'), patch.object(ab.subprocess,'run') as run:
            with self.assertRaises(ValueError):
                ab.benchmark(self.root,self.fixture,self.root/'aerender',self.root/'plugin',
                             self.root/'p.zip',self.root/'m.json',2,5,'inherit',60,'PERF094-RQ-001')
            run.assert_not_called()


if __name__=='__main__':unittest.main()
