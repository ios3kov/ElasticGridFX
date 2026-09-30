"""Control-flow regression tests for bounded target-AE smoke phase records."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import ae_smoke_runner as smoke


class SmokeRunner(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='egfx-smoke-runner-')
        self.addCleanup(self.tmp.cleanup)
        self.folder = Path(self.tmp.name).resolve()
        (self.folder / 'arm.jsx').write_text('// test')
        self.metadata = {'run_id': 'a' * 32}

    def test_numeric_ae_exit_is_not_mislabeled_as_transport_failure(self):
        completed = subprocess.CompletedProcess([], 0, '91\n', '')
        with patch.object(smoke.subprocess, 'run', return_value=completed):
            self.assertEqual(smoke._run_jsx(self.folder, 'arm.jsx', 'com.adobe.AfterEffects.application'), 91)
        self.assertEqual((self.folder / 'arm.jsx.transport.log').read_text(), '91\n\n')

    def test_nonzero_osascript_exit_is_transport_failure(self):
        completed = subprocess.CompletedProcess([], 1, '', 'not authorized')
        with patch.object(smoke.subprocess, 'run', return_value=completed):
            with self.assertRaisesRegex(ValueError, 'AE transport failed'):
                smoke._run_jsx(self.folder, 'arm.jsx', 'com.adobe.AfterEffects.application')

    def test_arm_reports_sanitized_guard_record(self):
        (self.folder / 'arm.json').write_text(
            '{"run_id":"' + 'a' * 32 + '","status":"FAIL","stage":"guard",'
            '"ae_version":"25.6.0","guard":"DIRTY_UNAVAILABLE","project_revision":"2"}'
        )
        with patch.object(smoke, '_transport_context', return_value=('com.adobe.AfterEffects.application', 42)), \
             patch.object(smoke, '_run_jsx', return_value=91):
            with self.assertRaises(smoke.AEPhaseError) as raised:
                smoke.arm(self.folder, self.metadata, self.folder, self.folder, self.folder, self.folder)
        self.assertEqual(str(raised.exception), 'AE arm guard refused: DIRTY_UNAVAILABLE')
        self.assertEqual(raised.exception.record, {
            'run_id': 'a' * 32, 'status': 'FAIL', 'stage': 'guard',
            'ae_version': '25.6.0', 'guard': 'DIRTY_UNAVAILABLE', 'project_revision': '2',
        })

    def test_arm_rejects_unrecognized_guard_record(self):
        (self.folder / 'arm.json').write_text(
            '{"run_id":"' + 'a' * 32 + '","status":"FAIL","stage":"guard",'
            '"ae_version":"25.6.0","guard":"/user/project.aep","project_revision":"1"}'
        )
        with patch.object(smoke, '_transport_context', return_value=('com.adobe.AfterEffects.application', 42)), \
             patch.object(smoke, '_run_jsx', return_value=91):
            with self.assertRaisesRegex(ValueError, 'invalid AE project guard record'):
                smoke.arm(self.folder, self.metadata, self.folder, self.folder, self.folder, self.folder)

    def test_arm_rejects_zero_exit_without_ownership_proof(self):
        (self.folder / 'arm.json').write_text(
            '{"run_id":"' + 'a' * 32 + '","status":"ARMED","stage":"armed",'
            '"ae_version":"25.6.0","guard":"DIRTY_UNAVAILABLE","project_revision":"2"}'
        )
        with patch.object(smoke, '_transport_context', return_value=('com.adobe.AfterEffects.application', 42)), \
             patch.object(smoke, '_run_jsx', return_value=0):
            with self.assertRaisesRegex(smoke.AEPhaseError, 'did not prove test-project ownership'):
                smoke.arm(self.folder, self.metadata, self.folder, self.folder, self.folder, self.folder)

    def test_disarm_rejects_zero_exit_without_fresh_project_proof(self):
        (self.folder / 'disarm.json').write_text(
            '{"run_id":"' + 'a' * 32 + '","status":"CLEAN","stage":"clean",'
            '"fresh_guard":"DIRTY_UNAVAILABLE","fresh_project_revision":"2"}'
        )
        metadata = dict(self.metadata, target_pid=42)
        with patch.object(smoke, '_transport_context', return_value=('com.adobe.AfterEffects.application', 42)), \
             patch.object(smoke, '_run_jsx', return_value=0):
            with self.assertRaisesRegex(smoke.AEPhaseError, 'did not prove a fresh test project'):
                smoke.disarm(self.folder, metadata, self.folder, self.folder, self.folder, self.folder)


if __name__ == '__main__':
    unittest.main()
