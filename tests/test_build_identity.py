"""Run without Rust/AE; tests identity provenance and package integrity only."""
from contextlib import redirect_stdout
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch
import zipfile

TOOL = Path(__file__).resolve().parents[1] / 'tools/build_identity.py'
SPEC = importlib.util.spec_from_file_location('build_identity', TOOL)
bi = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(bi)


class IdentityTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='elasticgrid-identity-test-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / 'source'
        self.root.mkdir()
        self.cmd('init', '-q')
        self.cmd('config', 'user.email', 'test@example.invalid')
        self.cmd('config', 'user.name', 'Identity Test')
        self.write('host-rust/Cargo.toml', '[package]\nname="elasticgrid_ae"\nversion="0.9.0"\n')
        self.write('src/example.cpp', 'int value = 1;\n')
        self.write('.gitignore', 'dist/\nhost-rust/target/\n')
        self.commit()

    def cmd(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], stderr=subprocess.PIPE).decode().strip()

    def write(self, name, text):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)

    def commit(self):
        self.cmd('add', '.')
        self.cmd('commit', '-qm', 'test state')

    def meta(self):
        return bi.identity(bi.source_record(self.root), 'aarch64-apple-darwin', 'release',
                           {'rustc': 'test compiler', 'cxx': 'test clang'}, {})

    def snapshot(self):
        record = bi.source_record(self.root)
        path = Path(self.tmp.name) / 'snapshot.json'
        bi.dump(path, record)
        target = Path(self.tmp.name) / 'export'
        shutil.copytree(self.root, target, ignore=shutil.ignore_patterns('.git'))
        return target, path, record

    def bundle(self):
        bundle = Path(self.tmp.name) / 'ElasticGrid.plugin'
        meta = self.meta()
        bi.dump(bundle / 'Contents/Resources/BuildIdentity.json', meta)
        exe = bundle / 'Contents/MacOS/ElasticGrid'
        exe.parent.mkdir(parents=True)
        exe.write_bytes(b'fixture\0ElasticGridBuildID=' + meta['build_id'].encode() + b'\0')
        exe.chmod(0o755)
        (bundle / 'Contents/Resources/ElasticGrid.rsrc').write_bytes(b'fixture PiPL')
        return bundle

    def test_clean_deterministic_identity(self):
        self.assertEqual(self.meta(), self.meta())
        self.assertEqual(self.meta()['source_state'], 'clean')
        self.assertEqual(self.meta()['commit'], self.cmd('rev-parse', 'HEAD'))
        self.assertEqual(self.meta()['version'], '0.9.0')

    def test_dirty_and_each_changed_content_get_unique_identity(self):
        original = self.meta()
        self.write('src/example.cpp', 'int value = 2;\n')
        first = self.meta()
        self.write('src/example.cpp', 'int value = 3;\n')
        second = self.meta()
        self.assertEqual(first['source_state'], 'dirty')
        self.assertEqual(first['commit'], original['commit'])
        self.assertEqual(len({m['build_id'] for m in (original, first, second)}), 3)
        with self.assertRaises(ValueError):
            bi.validate_identity(first)

    def test_target_profile_toolchain_and_flags_change_id(self):
        src = bi.source_record(self.root)
        args = [('aarch64-apple-darwin', 'release', {'rustc': 'a'}, {}),
                ('x86_64-apple-darwin', 'release', {'rustc': 'a'}, {}),
                ('aarch64-apple-darwin', 'debug', {'rustc': 'a'}, {}),
                ('aarch64-apple-darwin', 'release', {'rustc': 'b'}, {}),
                ('aarch64-apple-darwin', 'release', {'rustc': 'a'}, {'RUSTFLAGS': '-Copt-level=2'})]
        self.assertEqual(len({bi.identity(src, *a)['build_id'] for a in args}), len(args))

    def test_artifact_type_matches_target_platform(self):
        src = bi.source_record(self.root)
        mac = bi.identity(src, 'aarch64-apple-darwin', 'release', {'rustc':'a'}, {})
        win = bi.identity(src, 'x86_64-pc-windows-msvc', 'release', {'rustc':'a'}, {})
        self.assertEqual(mac['artifact_type'], 'AE native effect (.plugin)')
        self.assertEqual(win['artifact_type'], 'AE native effect (.aex)')

    def test_ignored_build_files_do_not_make_source_dirty(self):
        before = self.meta()
        self.write('dist/log.txt', 'generated')
        self.write('host-rust/target/output.txt', 'generated')
        self.assertEqual(self.meta(), before)

    def test_new_untracked_source_is_included(self):
        before = self.meta()
        self.write('src/new.cpp', 'int new_value=1;')
        self.assertNotEqual(self.meta()['source_sha256'], before['source_sha256'])
        self.assertEqual(self.meta()['source_state'], 'dirty')

    def test_committed_docs_change_invalidates_commit(self):
        before = self.meta()
        self.write('README.md', 'new checkpoint')
        self.commit()
        self.assertNotEqual(self.meta()['commit'], before['commit'])
        self.assertNotEqual(self.meta()['build_id'], before['build_id'])

    def test_snapshot_reproduces_identity_without_git(self):
        target, path, record = self.snapshot()
        self.assertEqual(bi.source_record(target, path), record)

    def test_snapshot_rejects_modified_and_additional_code(self):
        target, path, _ = self.snapshot()
        source = target / 'src/example.cpp'
        original = source.read_bytes()
        source.write_bytes(b'modified')
        with self.assertRaises(ValueError):
            bi.source_record(target, path)
        source.write_bytes(original)
        (target / 'src/extra.cpp').write_text('unexpected')
        with self.assertRaises(ValueError):
            bi.source_record(target, path)

    def test_snapshot_cannot_override_real_git(self):
        _, path, record = self.snapshot()
        record['commit'] = 'a' * 40
        bi.dump(path, record)
        with self.assertRaises(ValueError):
            bi.source_record(self.root, path)

    def test_worktree_git_file_is_supported(self):
        target = Path(self.tmp.name) / 'worktree'
        self.cmd('worktree', 'add', '--detach', str(target), 'HEAD')
        self.assertEqual(bi.source_record(target), bi.source_record(self.root))
        self.cmd('worktree', 'remove', str(target))

    def test_symlinks_and_path_traversal_refused(self):
        (self.root / 'src/link.cpp').symlink_to(self.root / 'src/example.cpp')
        with self.assertRaises(ValueError):
            bi.source_record(self.root)
        for path in ('../outside', '/outside', 'src/../example.cpp', 'src/./example.cpp', 'src/evil\ncargo:rustc-cfg=evil'):
            with self.assertRaises(ValueError):
                bi.safe_file(self.root, path)

    def test_about_message_uses_single_byte_copyright_and_no_terminal_periods(self):
        about = bi.about_message('0.9.0')
        self.assertEqual(
            about,
            b'FSTR Stretch\rVersion 0.9.0'
            b'\r\rProfessional mesh deformation for Adobe After Effects'
            b'\r\r\xa9 2026 FSTR.tech. All rights reserved',
        )
        self.assertEqual(about.count(b'\xa9'), 1)
        self.assertNotIn(b'\xc2\xa9', about)
        self.assertNotIn(b'fstr.tech', about)
        self.assertLess(len(about), 256)

    def test_generated_about_is_branded_and_diagnostic_retains_identity(self):
        out = Path(self.tmp.name) / 'out'
        original_run = bi.run
        def fake_run(args, cwd=None):
            if '--version' in args:
                return 'test-toolchain 1.0'
            return original_run(args, cwd)
        stdout = io.StringIO()
        with patch.object(bi, 'run', fake_run), patch.dict(os.environ, {}, clear=True), redirect_stdout(stdout):
            # git is still resolved by the system default search path.
            bi.generate(self.root, out, 'aarch64-apple-darwin', 'release')
        meta = json.loads((out / 'BuildIdentity.json').read_text())
        rust = (out / 'build_identity.rs').read_text(encoding='ascii')
        about_line = next(line for line in rust.splitlines() if line.startswith('pub const ABOUT_BYTES:'))
        diagnostic_line = next(line for line in rust.splitlines() if line.startswith('pub const DIAGNOSTIC:'))
        expected_bytes = ', '.join(f'0x{byte:02x}' for byte in bi.about_message('0.9.0'))
        self.assertEqual(about_line, 'pub const ABOUT_BYTES: &[u8] = &[' + expected_bytes + '];')
        self.assertIn('0xa9', about_line)
        self.assertNotIn('0xc2, 0xa9', about_line)
        self.assertNotIn(meta['build_id'], about_line)
        self.assertNotIn(meta['commit'], about_line)
        self.assertIn(meta['build_id'], diagnostic_line)
        self.assertIn(meta['commit'], diagnostic_line)
        self.assertIn('ElasticGridBuildID=', diagnostic_line)
        self.assertIn('aarch64-apple-darwin', diagnostic_line)
        self.assertIn('Source:', diagnostic_line)
        self.assertIn('cargo:rerun-if-env-changed=RUSTFLAGS', stdout.getvalue())
        self.assertIn(str(self.root / 'src/example.cpp'), stdout.getvalue())

    def test_adobe_sdk_override_participates_in_settings_identity(self):
        record=bi.source_record(self.root)
        with patch.dict(os.environ,{'AESDK_ROOT':'C:/Adobe/AE-SDK'},clear=True):
            settings=bi.build_settings()
        self.assertEqual(settings['AESDK_ROOT'],'C:/Adobe/AE-SDK')
        with patch.dict(os.environ,{},clear=True):
            default=bi.build_settings()
        self.assertEqual(default['AESDK_ROOT'],'')
        self.assertNotEqual(
            bi.identity(record,'x86_64-pc-windows-msvc','release',{'rustc':'r','cxx':'c'},settings)['build_id'],
            bi.identity(record,'x86_64-pc-windows-msvc','release',{'rustc':'r','cxx':'c'},default)['build_id'])

    def test_same_source_diagnostic_feature_has_distinct_identity(self):
        record=bi.source_record(self.root)
        def tag(env):
            with patch.dict(os.environ,env,clear=True):
                return bi.identity(record,'aarch64-apple-darwin','release',{'rustc':'fixed','cxx':'fixed'},bi.build_settings())
        default=tag({'CARGO_FEATURE_NATIVE_PLANE':'1'})
        diagnostic=tag({'CARGO_FEATURE_NATIVE_PLANE':'1','CARGO_FEATURE_RENDER_DIAGNOSTICS':'1'})
        legacy=tag({})
        self.assertEqual(default['source_sha256'],diagnostic['source_sha256'])
        self.assertEqual(default['commit'],diagnostic['commit'])
        self.assertEqual(len({x['build_id'] for x in (default,diagnostic,legacy)}),3)
        self.assertEqual(default,tag({'UNRELATED_PRIVATE_VALUE':'not recorded','CARGO_FEATURE_NATIVE_PLANE':'1'}))

    def test_stamp_rejects_stale_or_ambiguous_cargo_metadata(self):
        bundle = self.bundle()
        out = Path(self.tmp.name) / 'out'
        bi.dump(out / 'BuildIdentity.json', self.meta())
        log = Path(self.tmp.name) / 'cargo.jsonl'
        entry = json.dumps({'reason': 'build-script-executed', 'out_dir': str(out)}) + '\n'
        log.write_text(entry)
        bi.stamp(self.root, bundle, log)
        log.write_text(entry + entry)
        with self.assertRaises(ValueError):
            bi.stamp(self.root, bundle, log)
        log.write_text(entry)
        self.write('src/example.cpp', 'modified after compilation')
        with self.assertRaises(ValueError):
            bi.stamp(self.root, bundle, log)

    def test_signed_payload_and_package_roundtrip(self):
        bundle = self.bundle()
        package, manifest = Path(self.tmp.name) / 'plugin.zip', Path(self.tmp.name) / 'artifact.json'
        bi.seal(bundle, package, manifest)
        bi.verify(bundle, package, manifest)
        data = json.loads(manifest.read_text())
        self.assertEqual(data['runtime_verification'], 'NOT RUN')
        self.assertEqual(data['package_sha256'], bi.digest(package.read_bytes()))
        with zipfile.ZipFile(package) as archive:
            item = archive.getinfo('ElasticGrid.plugin/Contents/MacOS/ElasticGrid')
            self.assertEqual((item.external_attr >> 16) & 0o777, 0o755)

    def test_payload_and_package_tampering_rejected(self):
        bundle = self.bundle()
        package, manifest = Path(self.tmp.name) / 'plugin.zip', Path(self.tmp.name) / 'artifact.json'
        bi.seal(bundle, package, manifest)
        exe = bundle / 'Contents/MacOS/ElasticGrid'
        original = exe.read_bytes()
        exe.write_bytes(original + b'changed')
        with self.assertRaises(ValueError):
            bi.verify(bundle, package, manifest)
        exe.write_bytes(original)
        exe.chmod(0o644)
        with self.assertRaises(ValueError):
            bi.verify(bundle, package, manifest)
        exe.chmod(0o755)
        package.write_bytes(package.read_bytes() + b'changed')
        with self.assertRaises(ValueError):
            bi.verify(bundle, package, manifest)

    def test_generated_names_inside_source_are_not_ignored(self):
        target, path, _ = self.snapshot()
        directory = target / 'src/dist'
        directory.mkdir()
        (directory / 'build-extra.cpp').write_text('unexpected source')
        with self.assertRaises(ValueError):
            bi.source_record(target, path)

    def test_seal_cannot_modify_signed_bundle(self):
        bundle = self.bundle()
        with self.assertRaises(ValueError):
            bi.seal(bundle, bundle / 'package.zip', Path(self.tmp.name) / 'manifest.json')

    def test_archive_duplicate_entries_rejected(self):
        bundle = self.bundle()
        package = Path(self.tmp.name) / 'plugin.zip'
        files = bi.payload_files(bundle)
        bi.seal(bundle, package, Path(self.tmp.name) / 'manifest.json')
        # A different extra file must not be accepted even with a matching ZIP hash.
        with zipfile.ZipFile(package, 'a') as archive:
            archive.writestr('unexpected.txt', 'extra')
        with self.assertRaises(ValueError):
            bi.verify_package(bundle, package, files)

    def test_mismatched_binary_or_dirty_metadata_rejected(self):
        bundle = self.bundle()
        exe = bundle / 'Contents/MacOS/ElasticGrid'
        exe.write_bytes(b'an old binary')
        with self.assertRaises(ValueError):
            bi.payload_metadata(bundle)
        meta = self.meta()
        meta['commit'] = 'a' * 40
        with self.assertRaises(ValueError):
            bi.validate_identity(meta)


if __name__ == '__main__':
    unittest.main()
