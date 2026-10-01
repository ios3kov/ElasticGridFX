"""Portable safety/contract tests for the 0.9.4 baseline orchestrator. No AE execution."""
from pathlib import Path
import tempfile
import sys
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import performance_baseline_094 as pb


class Baseline094(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='egfx-094-baseline-')
        self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name).resolve()

    def test_core_matrix_covers_required_grid_depth_mfr_and_four_corners(self):
        cases=pb.matrix_conditions('core')
        self.assertTrue(any(c['test_case_id']=='PERF094-RQ-001' and c['mfr']=='on' for c in cases))
        self.assertTrue(any(c['test_case_id']=='PERF094-RQ-001' and c['mfr']=='off' for c in cases))
        self.assertTrue(any(c['test_case_id']=='PERF094-RQ-002' and c['width']==3840 for c in cases))
        depths={c['bit_depth'] for c in cases if c['geometry']=='grid' and c['width']==3840}
        self.assertEqual(depths,{8,16,32})
        self.assertTrue(any(c['geometry']=='four_corners' and c['mfr']=='on' for c in cases))
        self.assertTrue(any(c['geometry']=='four_corners' and c['mfr']=='off' for c in cases))
        self.assertEqual(len({c['condition_id'] for c in cases}),len(cases))

    def test_smoke_is_explicit_subset_not_stage_completion(self):
        smoke=pb.matrix_conditions('smoke')
        core=pb.matrix_conditions('core')
        self.assertLess(len(smoke),len(core))
        self.assertEqual({c['geometry'] for c in smoke},{'grid','four_corners'})
        with self.assertRaises(ValueError):
            pb.matrix_conditions('invented')

    def test_nonmac_blocks_before_candidate_or_host_work(self):
        with patch.object(pb.platform,'system',return_value='Linux'),              patch.object(pb,'candidate_files') as candidate,              patch.object(pb,'running_target') as host:
            with self.assertRaises(ValueError):
                pb.run_baseline(self.root,self.root,'smoke',0,5,60)
            candidate.assert_not_called();host.assert_not_called()

    def test_candidate_identity_requires_exact_frozen_baseline(self):
        package=self.root/pb.PACKAGE_NAME;package.write_bytes(b'package')
        manifest=self.root/pb.MANIFEST_NAME
        base=dict(schema=1,version=pb.BASELINE_VERSION,commit=pb.BASELINE_COMMIT,
                  source_state='clean',source_sha256='a'*64,artifact_type='AE native effect (.plugin)',
                  target='aarch64-apple-darwin',profile='release',toolchain={},
                  settings_sha256='b'*64)
        # Build ID validation is tested in build_identity itself; patch only the
        # validator here so this unit case focuses on the orchestrator's extra pins.
        manifest.write_text('{"build":{}}')
        with patch.object(pb.bi,'validate_identity',return_value=dict(base,build_id='wrong')):
            with self.assertRaisesRegex(ValueError,'Build ID'):
                pb.candidate_files(self.root)

    def test_successful_mock_matrix_still_reports_stage_blocked(self):
        installed=self.root/'Installed.plugin';installed.mkdir()
        package=self.root/pb.PACKAGE_NAME;package.write_bytes(b'p')
        manifest=self.root/pb.MANIFEST_NAME;manifest.write_text('{}')
        ae_app=self.root/'AE.app';ae_app.mkdir()
        exe=self.root/'After Effects';exe.write_bytes(b'x')
        aerender=self.root/'aerender';aerender.write_bytes(b'x')
        host=dict(pid=123,app=ae_app,executable=exe)
        condition=dict(condition_id='grid-1080p-32-animated-mfr-on',
                       test_case_id='PERF094-RQ-001',width=1920,height=1080,
                       bit_depth=32,mode='animated',geometry='grid',mfr='on')

        def fake_prepare(root,*args):
            folder=root/'fixture-a';folder.mkdir()
            (folder/'fixture.json').write_text('{}')
            return folder,{'status':'NOT RUN'}

        def fake_execute(folder,meta,*args):
            return dict(meta,status='PREPARED')

        def fake_benchmark(folder,*args):
            report=folder/'aerender-benchmark.json';report.write_text('{"ok":true}')
            data=dict(summary=dict(samples=5,frame_count=60,wall_median_seconds=1.0,
                                   wall_p95_seconds=1.1,ms_per_frame_median=16.6,
                                   ms_per_frame_p95=18.3,peak_rss_max_bytes=100,
                                   output_digest='d',encoded_output_repeatability='PASS'))
            return data,report

        with patch.object(pb.platform,'system',return_value='Darwin'),              patch.object(pb.platform,'machine',return_value='arm64'),              patch.object(pb,'candidate_files',return_value=(package,manifest,{})),              patch.object(pb,'running_target',return_value=host),              patch.object(pb,'discover_aerender',return_value=aerender),              patch.object(pb,'live_baseline_identity',return_value=({'observed_build_id':pb.BASELINE_BUILD_ID,'observed_image_uuid':'U'},installed)),              patch.object(pb.ab,'verify_candidate'),              patch.object(pb,'require_same_host'),              patch.object(pb,'matrix_conditions',return_value=[condition]),              patch.object(pb.pf,'prepare',side_effect=fake_prepare),              patch.object(pb.pf,'execute',side_effect=fake_execute),              patch.object(pb.ab,'benchmark',side_effect=fake_benchmark):
            result,path=pb.run_baseline(self.root,self.root,'smoke',0,5,60)
        self.assertEqual(result['status'],'AERENDER_BASELINE_MEASURED')
        self.assertEqual(result['stage_status'],'BLOCKED')
        self.assertFalse(result['performance_claim_allowed'])
        self.assertIn('PERF094-RAM-001/002/003 real RAM Preview baseline',result['stage_blockers'])
        self.assertTrue(path.is_file())


if __name__=='__main__':
    unittest.main()
