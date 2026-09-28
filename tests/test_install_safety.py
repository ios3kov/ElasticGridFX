"""Real filesystem tests in owned temporary fixtures; signatures/AE are mocked."""
import contextlib
import json
import os
from pathlib import Path
import plistlib
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import build_identity as bi
import install_candidate as ic


class InstallerSafety(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='egfx-install-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name).resolve()
        self.bundle = self.root / 'source/ElasticGrid.plugin'
        self.scope = self.root / 'test-scope'
        self.scope.mkdir()
        self.scan = self.root / 'other-root'
        self.scan.mkdir()
        self.package = self.root / 'package.zip'
        self.manifest = self.root / 'manifest.json'
        meta = bi.identity(dict(version='0.9.0', commit='a'*40, source_state='clean',
                                source_sha256='b'*64), 'aarch64-apple-darwin', 'release', {}, {})
        bi.dump(self.bundle / 'Contents/Resources/BuildIdentity.json', meta)
        binary = self.bundle / 'Contents/MacOS/ElasticGrid'
        binary.parent.mkdir(parents=True)
        binary.write_bytes(b'fixture ElasticGridBuildID=' + meta['build_id'].encode())
        binary.chmod(0o755)
        (self.bundle / 'Contents/Info.plist').write_bytes(plistlib.dumps({'CFBundleIdentifier':'com.elasticgrid.fx'}))
        bi.seal(self.bundle, self.package, self.manifest)
        self.destination = self.scope / self.bundle.name
        self.signature_calls = []

    def apply(self, **kwargs):
        return ic.install(self.bundle, self.package, self.manifest, self.scope, [self.scan],
                          verify_signature=kwargs.pop('verify_signature', self.signature_calls.append),
                          check_hosts=kwargs.pop('check_hosts', lambda: None), **kwargs)

    def test_inspection_writes_nothing(self):
        before = sorted(str(p) for p in self.root.rglob('*'))
        record = ic.inspect(self.bundle, self.package, self.manifest, [self.scan, self.scope])
        self.assertEqual(record['conflicts'], [])
        self.assertEqual(before, sorted(str(p) for p in self.root.rglob('*')))

    def test_create_only_install_preserves_exact_payload(self):
        result = self.apply()
        self.assertEqual(result['status'], 'INSTALLED_FOR_TEST')
        self.assertEqual(result['runtime_verification'], 'NOT RUN')
        bi.verify(self.destination, self.package, self.manifest)
        self.assertEqual(len(self.signature_calls), 3)
        self.assertEqual(len(list(self.scope.glob('ElasticGridFX-install-*.json'))), 1)

    def test_same_candidate_is_idempotent(self):
        self.apply()
        before = bi.payload_files(self.destination)
        self.assertEqual(self.apply()['status'], 'ALREADY_PRESENT')
        self.assertEqual(bi.payload_files(self.destination), before)

    def test_old_or_tampered_build_is_never_replaced(self):
        self.apply()
        sentinel = self.destination / 'Contents/keep.txt'
        sentinel.write_text('existing work')
        with self.assertRaises(ValueError):
            self.apply()
        self.assertEqual(sentinel.read_text(), 'existing work')

    def test_nested_fstr_or_renamed_bundle_is_detected(self):
        import shutil
        other = self.scan / 'FSTR FX/Renamed.plugin'
        shutil.copytree(self.bundle, other)
        with self.assertRaises(ValueError):
            self.apply()
        self.assertTrue(other.exists())
        self.assertFalse(self.destination.exists())

    def test_missing_signature_or_corrupt_package_is_rejected(self):
        def reject(_):
            raise ValueError('signature invalid')
        with self.assertRaises(ValueError):
            self.apply(verify_signature=reject)
        self.assertFalse(self.destination.exists())
        self.package.write_bytes(b'corrupted')
        with self.assertRaises(ValueError):
            self.apply()
        self.assertFalse(self.destination.exists())

    def test_stopped_host_requirement_is_not_bypassed(self):
        def running():
            raise ValueError('host running')
        with self.assertRaises(ValueError):
            self.apply(check_hosts=running)
        self.assertFalse(self.destination.exists())
        with patch.object(ic.subprocess, 'check_output', return_value='123 /Applications/Adobe After Effects.app/Contents/MacOS/After Effects\n'):
            with self.assertRaises(ValueError):
                ic.adobe_hosts_stopped()

    def test_empty_existing_destination_is_preserved(self):
        self.destination.mkdir()
        with self.assertRaises(ValueError):
            self.apply()
        self.assertEqual(list(self.destination.iterdir()), [])

    def test_symlink_scope_destination_and_source_refused(self):
        link = self.root / 'scope-link'
        link.symlink_to(self.scope)
        with self.assertRaises(ValueError):
            ic.install(self.bundle, self.package, self.manifest, link, [],
                       verify_signature=lambda _:None, check_hosts=lambda:None)
        self.destination.symlink_to(self.bundle)
        with self.assertRaises(ValueError):
            self.apply()
        self.assertTrue(self.destination.is_symlink())

    def test_symlink_lock_is_not_followed(self):
        sentinel = self.root / 'sentinel'
        sentinel.write_text('preserve')
        (self.scope / '.elasticgrid-install.lock').symlink_to(sentinel)
        with self.assertRaises((OSError, ValueError)):
            self.apply()
        self.assertEqual(sentinel.read_text(), 'preserve')

    def test_new_conflict_during_copy_is_not_overwritten(self):
        calls = 0
        def check():
            nonlocal calls
            calls += 1
            if calls == 2:
                self.destination.mkdir()
                (self.destination / 'keep').write_text('concurrent install')
        with self.assertRaises(ValueError):
            self.apply(check_hosts=check)
        self.assertEqual((self.destination / 'keep').read_text(), 'concurrent install')
        receipts = list(self.scope.glob('ElasticGridFX-install-*.json'))
        self.assertEqual(json.loads(receipts[0].read_text())['status'], 'FAIL')

    def test_scan_permission_error_fails_closed(self):
        with patch.object(Path, 'iterdir', side_effect=PermissionError('not readable')):
            with self.assertRaises(PermissionError):
                ic.discover([self.scan])

    def test_inspection_cli_and_help_never_invoke_sudo(self):
        # Use the actual shell entry point in a fake repo. Old --help ignored its
        # argument and ran sudo rm; a fake sudo records the attempted mutation.
        tools = self.root / 'entry/tools'
        tools.mkdir(parents=True)
        command = tools / 'install_macos.command'
        command.write_bytes((ROOT / 'tools/install_macos.command').read_bytes())
        (tools / 'install_candidate.py').write_bytes((ROOT / 'tools/install_candidate.py').read_bytes())
        (tools / 'build_identity.py').write_bytes((ROOT / 'tools/build_identity.py').read_bytes())
        (tools.parent / 'dist/mac/ElasticGrid.plugin').mkdir(parents=True)
        fakebin = self.root / 'bin'; fakebin.mkdir()
        sudo = fakebin / 'sudo'; sudo.write_text('#!/bin/sh\necho UNSAFE_MUTATION_ATTEMPT >&2\nexit 73\n'); sudo.chmod(0o755)
        result = subprocess.run(['bash', str(command), '--help'], capture_output=True, text=True,
                                env={**os.environ, 'PATH':str(fakebin)+os.pathsep+os.environ['PATH']}, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn('UNSAFE_MUTATION_ATTEMPT', result.stderr)


if __name__ == '__main__':
    unittest.main()
