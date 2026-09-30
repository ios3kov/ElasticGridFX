"""Presigning metadata tests with synthetic bundles; NOT codesign/AE checks."""
import json
from pathlib import Path
import plistlib
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import build_identity as bi
import bundle_version as bv


class BundleVersionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.bundle = Path(self.tmp.name) / 'FSTR Stretch.plugin'
        self.meta = bi.identity(dict(version='0.9.3', commit='a'*40, source_state='clean',
                                    source_sha256='b'*64), 'aarch64-apple-darwin', 'release', {}, {})
        self.identity = self.bundle / 'Contents/Resources/BuildIdentity.json'
        self.identity.parent.mkdir(parents=True)
        self.identity.write_text(json.dumps(self.meta))
        self.binary = self.bundle / 'Contents/MacOS/ElasticGrid'
        self.binary.parent.mkdir()
        self.binary.write_bytes(b'fixture ElasticGridBuildID=' + self.meta['build_id'].encode())
        self.plist = self.bundle / 'Contents/Info.plist'
        self.original = dict(CFBundleShortVersionString='0.9.0', CFBundleVersion='9',
                             CFBundleExecutable='ElasticGrid', CFBundleIdentifier='com.elasticgrid.fx')
        self.plist.write_bytes(plistlib.dumps(self.original))

    def test_stamp_matches_validated_identity_and_preserves_other_fields(self):
        old_binary, old_identity = self.binary.read_bytes(), self.identity.read_bytes()
        self.assertEqual(bv.stamp(self.bundle), '0.9.3')
        self.assertEqual(bv.verify(self.bundle), '0.9.3')
        info = plistlib.loads(self.plist.read_bytes())
        for key in ('CFBundleExecutable', 'CFBundleIdentifier'):
            self.assertEqual(info[key], self.original[key])
        self.assertEqual(self.binary.read_bytes(), old_binary)
        self.assertEqual(self.identity.read_bytes(), old_identity)
        once = self.plist.read_bytes()
        bv.stamp(self.bundle)
        self.assertEqual(self.plist.read_bytes(), once)

    def test_detects_stale_versions_without_rewriting_signed_candidate(self):
        old = self.plist.read_bytes()
        with self.assertRaises(ValueError):
            bv.verify(self.bundle)
        (self.bundle / 'Contents/_CodeSignature').mkdir()
        with self.assertRaises(ValueError):
            bv.stamp(self.bundle)
        self.assertEqual(self.plist.read_bytes(), old)

    def test_rejects_mismatched_binary_and_symlinked_plist(self):
        self.binary.write_bytes(b'wrong build')
        with self.assertRaises(ValueError):
            bv.stamp(self.bundle)
        self.binary.write_bytes(b'ElasticGridBuildID=' + self.meta['build_id'].encode())
        original = self.plist.read_bytes()
        target = Path(self.tmp.name) / 'outside.plist'
        target.write_bytes(original)
        self.plist.unlink()
        self.plist.symlink_to(target)
        with self.assertRaises(ValueError):
            bv.stamp(self.bundle)
        self.assertEqual(target.read_bytes(), original)

    def test_build_stamps_before_signing_and_native_verifier_checks_versions(self):
        source = (ROOT / 'tools/build_macos_sdkless.command').read_text()
        self.assertNotIn('CFBundleShortVersionString string 0.9.0', source)
        self.assertNotIn('CFBundleVersion string 9', source)
        self.assertLess(source.index('build_identity.py" stamp'), source.index('bundle_version.py" stamp'))
        self.assertLess(source.index('bundle_version.py" stamp'), source.index('codesign --force'))
        verifier = (ROOT / 'tools/verify_bundle_macos.command').read_text()
        self.assertIn('bundle_version.py" verify', verifier)


if __name__ == '__main__':
    unittest.main()
