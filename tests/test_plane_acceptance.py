"""Control-flow tests for the Stage 9 AE acceptance runner; never launches AE."""
from pathlib import Path
import json
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
import ae_plane_acceptance as pa


class PlaneAcceptance(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='egfx-plane-runner-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()


    def test_load_candidate_requires_exact_known_package_sha(self):
        manifest_path = self.root/'manifest.json'
        package_path = self.root/'candidate.zip'
        package_path.write_bytes(b'candidate')
        build = {'commit':pa.EXPECTED_COMMIT,'build_id':pa.EXPECTED_BUILD}
        manifest_path.write_text(json.dumps({
            'build':build,
            'package_sha256':pa.EXPECTED_PACKAGE_SHA,
        }))
        with patch.object(pa.bi, 'validate_identity', side_effect=lambda value:value), \
             patch.object(pa.bi, 'digest', return_value=pa.EXPECTED_PACKAGE_SHA):
            self.assertEqual(pa.load_candidate(manifest_path, package_path)['build'], build)

        manifest_path.write_text(json.dumps({
            'build':build,
            'package_sha256':'0'*64,
        }))
        with patch.object(pa.bi, 'validate_identity', side_effect=lambda value:value), \
             patch.object(pa.bi, 'digest', return_value=pa.EXPECTED_PACKAGE_SHA):
            with self.assertRaises(ValueError):
                pa.load_candidate(manifest_path, package_path)

        manifest_path.write_text(json.dumps({
            'build':build,
            'package_sha256':pa.EXPECTED_PACKAGE_SHA,
        }))
        with patch.object(pa.bi, 'validate_identity', side_effect=lambda value:value), \
             patch.object(pa.bi, 'digest', return_value='f'*64):
            with self.assertRaises(ValueError):
                pa.load_candidate(manifest_path, package_path)

    def test_prepare_is_unique_and_not_run(self):
        a, meta_a = pa.prepare(self.root/'runs', {'build_id':pa.EXPECTED_BUILD})
        b, _ = pa.prepare(self.root/'runs', {'build_id':pa.EXPECTED_BUILD})
        self.assertNotEqual(a, b)
        self.assertEqual(meta_a['status'], 'NOT RUN')
        self.assertFalse(meta_a['actual_ae_execution'])
        self.assertTrue((a/'run.jsx').is_file())
        self.assertTrue((a/'cleanup.jsx').is_file())
        self.assertTrue((a/'pattern.png').is_file())
        self.assertIn('elasticGridPlaneSmoke(', (a/'run.jsx').read_text())
        self.assertIn('elasticGridPlaneSmokeCleanup(', (a/'cleanup.jsx').read_text())

    def test_phase_record_rejects_stale_or_wrong_status(self):
        folder = self.root/'phase'; folder.mkdir()
        (folder/'plane-smoke.json').write_text(json.dumps(
            {'run_id':'a'*32,'status':'FAIL','stage':'guard'}))
        with self.assertRaises(ValueError):
            pa.phase_record(folder, 'plane-smoke.json', 'a'*32, 'CAPTURED')
        with self.assertRaises(ValueError):
            pa.phase_record(folder, 'plane-smoke.json', 'b'*32, 'FAIL')

    @patch.object(pa, 'write_report', side_effect=lambda run, result, fixture: run/'report.zip')
    @patch.object(pa, 'validate_plane_frames', return_value={'status':'PASS','checks':{}})
    @patch.object(pa.live_identity, 'diagnose')
    @patch.object(pa.ae_smoke_runner, '_run_jsx')
    @patch.object(pa.ae_smoke_runner, '_transport_context')
    @patch.object(pa, 'load_candidate')
    @patch.object(pa.platform, 'machine', return_value='arm64')
    @patch.object(pa.platform, 'system', return_value='Darwin')
    def test_success_requires_identity_pixels_and_cleanup(
        self, _system, _machine, load_candidate, transport, run_jsx, diagnose,
        validate, write_report
    ):
        build = {'commit':pa.EXPECTED_COMMIT,'build_id':pa.EXPECTED_BUILD}
        load_candidate.return_value = {'build':build}
        transport.return_value = ('com.adobe.AfterEffects', 77)
        run_jsx.side_effect = [0, 0]
        diagnose.return_value = {
            'status':'PASS','loaded_image_status':'PASS','observed_build_id':pa.EXPECTED_BUILD,
            'ae':{'pid':77,'path':'/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app'}
        }
        fixture = self.root/'fixture'; fixture.mkdir()
        meta = {'run_id':'a'*32}
        with patch.object(pa, 'prepare', return_value=(fixture, meta)), \
             patch.object(pa, 'phase_record', side_effect=[
                 {'run_id':'a'*32,'status':'CAPTURED','stage':'complete','frames':list(pa.FRAMES)},
                 {'run_id':'a'*32,'status':'CLEAN','stage':'complete'},
             ]):
            result, _ = pa.run_acceptance(
                self.root/'reports',
                Path('/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app'),
                self.root/'ElasticGrid.plugin',
                self.root/'manifest.json',
                self.root/'plugin.zip',
            )
        self.assertEqual(result['functional_status'], 'PASS')
        self.assertEqual(result['cleanup_status'], 'CLEAN')
        self.assertEqual(result['pixel_status'], 'PASS')
        self.assertEqual(run_jsx.call_count, 2)
        self.assertGreaterEqual(transport.call_count, 3)
        write_report.assert_called_once()

    @patch.object(pa, 'write_report', side_effect=lambda run, result, fixture: run/'report.zip')
    @patch.object(pa, 'validate_plane_frames', return_value={'status':'PASS','checks':{}})
    @patch.object(pa.live_identity, 'diagnose')
    @patch.object(pa.ae_smoke_runner, '_run_jsx')
    @patch.object(pa.ae_smoke_runner, '_transport_context')
    @patch.object(pa, 'load_candidate')
    @patch.object(pa.platform, 'machine', return_value='arm64')
    @patch.object(pa.platform, 'system', return_value='Darwin')
    def test_unproven_identity_never_passes_even_if_pixels_would(
        self, _system, _machine, load_candidate, transport, run_jsx, diagnose,
        validate, write_report
    ):
        build = {'commit':pa.EXPECTED_COMMIT,'build_id':pa.EXPECTED_BUILD}
        load_candidate.return_value = {'build':build}
        transport.return_value = ('com.adobe.AfterEffects', 77)
        run_jsx.side_effect = [0, 0]
        diagnose.return_value = {
            'status':'BLOCKED','loaded_image_status':'NOT RUN','observed_build_id':None,
            'ae':{'pid':77,'path':'/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app'}
        }
        fixture = self.root/'fixture2'; fixture.mkdir()
        meta = {'run_id':'b'*32}
        with patch.object(pa, 'prepare', return_value=(fixture, meta)), \
             patch.object(pa, 'phase_record', side_effect=[
                 {'run_id':'b'*32,'status':'CAPTURED','stage':'complete','frames':list(pa.FRAMES)},
                 {'run_id':'b'*32,'status':'CLEAN','stage':'complete'},
             ]):
            result, _ = pa.run_acceptance(
                self.root/'reports2',
                Path('/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app'),
                self.root/'ElasticGrid.plugin',
                self.root/'manifest.json',
                self.root/'plugin.zip',
            )
        self.assertNotEqual(result['functional_status'], 'PASS')
        self.assertEqual(result['identity_status'], 'NOT RUN')
        validate.assert_not_called()
        self.assertEqual(result['cleanup_status'], 'CLEAN')


if __name__ == '__main__':
    unittest.main()
