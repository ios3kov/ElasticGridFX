"""Real filesystem/ZIP branding checks; no AE or signature claims from mocks."""
import json
from pathlib import Path
import plistlib
import shutil
import sys
import tempfile
import unittest
import zipfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import build_identity as bi
import install_candidate as ic
import package_fstr_stretch as branded


class BrandedPackage(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='egfx branding ')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.bundle = self.root / 'input/ElasticGrid.plugin'
        self.archive = self.root / 'input.zip'
        self.manifest = self.root / 'input.json'
        self.output = self.root / 'delivery'
        self.meta = bi.identity(dict(version='0.9.1', commit='a' * 40, source_state='clean',
                                    source_sha256='b' * 64), 'aarch64-apple-darwin', 'release', {}, {})
        bi.dump(self.bundle / 'Contents/Resources/BuildIdentity.json', self.meta)
        binary = self.bundle / 'Contents/MacOS/ElasticGrid'
        binary.parent.mkdir(parents=True)
        binary.write_bytes(b'test fixture ElasticGridBuildID=' + self.meta['build_id'].encode())
        binary.chmod(0o755)
        (self.bundle / 'Contents/Info.plist').write_bytes(plistlib.dumps({
            'CFBundleIdentifier': 'com.elasticgrid.fx', 'CFBundleExecutable': 'ElasticGrid'}))
        bi.seal(self.bundle, self.archive, self.manifest)

    def package(self):
        return branded.package(self.bundle, self.archive, self.manifest, self.output)

    def test_new_name_preserves_every_byte_permission_and_build_identity(self):
        before = bi.payload_files(self.bundle)
        original = self.archive.read_bytes(), self.manifest.read_bytes()
        result = self.package()
        target = self.output / branded.BUNDLE_NAME
        self.assertEqual(bi.payload_files(target), before)
        self.assertEqual(result['build'], self.meta)
        self.assertEqual(result['runtime_verification'], 'NOT RUN')
        self.assertEqual(result['signature_verification'], 'NOT RUN')
        self.assertTrue(result['payload_unchanged'])
        self.assertEqual(original, (self.archive.read_bytes(), self.manifest.read_bytes()))
        self.assertNotEqual(result['package_sha256'], result['input_package_sha256'])
        bi.verify(target, self.output / branded.PACKAGE_NAME, self.output / branded.MANIFEST_NAME)
        with zipfile.ZipFile(self.output / branded.PACKAGE_NAME) as z:
            self.assertEqual(set(z.namelist()), {branded.BUNDLE_NAME + '/' + n for n in before})
        self.assertFalse((self.output / 'ElasticGrid.plugin').exists())

    def test_corrupted_input_archive_is_rejected_before_output(self):
        self.archive.write_bytes(b'wrong archive')
        with self.assertRaises(ValueError):
            self.package()
        self.assertFalse(self.output.exists())

    def test_changed_input_payload_is_rejected(self):
        (self.bundle / 'Contents/MacOS/ElasticGrid').write_bytes(b'wrong binary')
        with self.assertRaises(ValueError):
            self.package()
        self.assertFalse(self.output.exists())

    def test_existing_output_and_source_are_never_overwritten(self):
        self.output.mkdir()
        (self.output / 'keep.txt').write_text('keep')
        with self.assertRaises(FileExistsError):
            self.package()
        self.assertEqual((self.output / 'keep.txt').read_text(), 'keep')
        with self.assertRaises(ValueError):
            branded.package(self.bundle, self.archive, self.manifest, self.bundle / 'nested')
        self.assertFalse((self.bundle / 'nested').exists())

    def test_symlinked_output_parent_and_payload_are_rejected(self):
        link = self.root / 'linked'
        link.symlink_to(self.root, target_is_directory=True)
        with self.assertRaises(ValueError):
            branded.package(self.bundle, self.archive, self.manifest, link / 'delivery')
        (self.bundle / 'Contents/link').symlink_to(self.archive)
        with self.assertRaises(ValueError):
            self.package()

    def test_unexpected_executable_or_identifier_is_rejected(self):
        p = self.bundle / 'Contents/Info.plist'
        p.write_bytes(plistlib.dumps({'CFBundleIdentifier': 'foreign.plugin',
                                     'CFBundleExecutable': 'ElasticGrid'}))
        bi.seal(self.bundle, self.archive, self.manifest)
        with self.assertRaises(ValueError):
            self.package()

    def test_both_names_are_discovered_in_distinct_install_roots(self):
        self.package()
        self.assertEqual(ic.discover([self.bundle.parent, self.output]),
                         sorted([self.bundle, self.output / branded.BUNDLE_NAME]))

    def test_branded_name_without_metadata_remains_a_conflict(self):
        path = self.root / branded.BUNDLE_NAME
        path.mkdir()
        self.assertIn(path, ic.discover([self.root]))

    def test_branded_create_only_install_and_repeat_preserve_bytes(self):
        self.package()
        scope = self.root / 'user MediaCore'; scope.mkdir()
        def apply():
            return ic.install(self.output / branded.BUNDLE_NAME, self.output / branded.PACKAGE_NAME,
                              self.output / branded.MANIFEST_NAME, scope, [],
                              verify_signature=lambda _: None, check_hosts=lambda: None)
        self.assertEqual(apply()['status'], 'INSTALLED_FOR_TEST')
        self.assertEqual(apply()['status'], 'ALREADY_PRESENT')
        self.assertEqual(bi.payload_files(scope / branded.BUNDLE_NAME), bi.payload_files(self.bundle))

    def test_legacy_copy_blocks_branded_install_without_modification(self):
        self.package()
        scope = self.root / 'user MediaCore'; scope.mkdir()
        other = self.root / 'system MediaCore'
        shutil.copytree(self.bundle, other / self.bundle.name)
        before = bi.payload_files(other / self.bundle.name)
        with self.assertRaises(ValueError):
            ic.install(self.output / branded.BUNDLE_NAME, self.output / branded.PACKAGE_NAME,
                       self.output / branded.MANIFEST_NAME, scope, [other],
                       verify_signature=lambda _: None, check_hosts=lambda: None)
        self.assertEqual(before, bi.payload_files(other / self.bundle.name))
        self.assertFalse((scope / branded.BUNDLE_NAME).exists())

    def test_build_gates_verify_branded_and_extracted_signature(self):
        script = (ROOT / 'tools/build_macos_sdkless.command').read_text()
        self.assertIn('package_fstr_stretch.py', script)
        self.assertIn('"$DIST/delivery/FSTR Stretch.plugin"', script)
        self.assertIn('"$ROUNDTRIP/FSTR Stretch.plugin"', script)
        self.assertIn('"$DIST/delivery/FSTR Stretch.artifact.json"', script)
        self.assertNotIn('xattr -', script.split('# User-facing package:')[1])
        workflow = (ROOT / '.github/workflows/macos-source-gate.yml').read_text()
        self.assertIn('dist/mac/delivery/FSTR Stretch.plugin.zip', workflow)


if __name__ == '__main__':
    unittest.main()
