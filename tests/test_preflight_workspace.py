"""Verify real script paths/export policy; no macOS, Rust or AE runtime claims."""
import importlib.util
import os
from pathlib import Path
import re
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location('workspace_identity', ROOT / 'tools/build_identity.py')
bi = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(bi)
TOOLS = ('audit_hotpath.command', 'run_metal_bench_macos.command',
         'run_metal_determinism_macos.command', 'run_metal_lifecycle_macos.command',
         'run_metal_parity_macos.command')


def scratch_paths():
    result = []
    for name in TOOLS:
        script = (ROOT / 'tools' / name).read_text()
        result.append(re.search(r'^BUILD="\$ROOT/([^"\n]+)"$', script, re.M).group(1))
    return result


class PreflightWorkspace(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='elasticgrid-workspace-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / 'source with spaces'
        (self.root / 'host-rust').mkdir(parents=True)
        (self.root / '.gitignore').write_bytes((ROOT / '.gitignore').read_bytes())
        (self.root / 'host-rust/Cargo.toml').write_text('[package]\nname="fixture"\nversion="0.9.0"\n')
        (self.root / 'source.cpp').write_text('int value=1;\n')
        self.git('init', '-q')
        self.git('config', 'user.name', 'Workspace Test')
        self.git('config', 'user.email', 'test@example.invalid')
        self.git('add', '.')
        self.git('commit', '-qm', 'fixture')

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], stderr=subprocess.PIPE).decode().strip()

    def populate_scratch(self):
        for name in scratch_paths():
            directory = self.root / name
            directory.mkdir(parents=True, exist_ok=True)
            (directory / 'test-binary').write_bytes(b'generated output')

    def test_five_tools_have_distinct_owned_subdirectories(self):
        paths = scratch_paths()
        self.assertEqual(len(set(paths)), 5)
        for name in paths:
            self.assertEqual(Path(name).parts[0], '.preflight-macos')
            self.assertEqual(len(Path(name).parts), 2)

    def test_generated_outputs_leave_git_and_source_hash_unchanged(self):
        before = bi.source_record(self.root)
        self.populate_scratch()
        self.assertEqual(self.git('status', '--porcelain', '--untracked-files=all'), '')
        self.assertEqual(bi.source_record(self.root), before)

    def test_actual_export_excludes_scratch_but_preserves_source(self):
        record = bi.source_record(self.root)
        self.populate_scratch()
        out = Path(self.tmp.name) / 'export with spaces'
        out.mkdir()
        # Execute the actual tar pipeline from the reproduction script, not a
        # separately maintained imitation of its excludes. No compiler is run.
        script = (ROOT / 'tools/repro_build_macos.command').read_text()
        pipeline = script.split('tar -C "$ROOT"', 1)[1].split('\n\n', 1)[0]
        subprocess.run(['bash', '-c', 'set -euo pipefail\ntar -C "$ROOT"' + pipeline],
                       env={**os.environ, 'ROOT': str(self.root), 'SRC': str(out)}, check=True, timeout=10)
        self.assertFalse((out / '.preflight-macos').exists())
        snapshot = Path(self.tmp.name) / 'source.json'
        bi.dump(snapshot, record)
        self.assertEqual(bi.source_record(out, snapshot), record)

    def test_unknown_source_and_legacy_scratch_still_mark_dirty(self):
        self.populate_scratch()
        for name in ('new.cpp', '.hotpath-audit/old-evidence.txt'):
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('not silently ignored')
        self.assertEqual(bi.source_record(self.root)['source_state'], 'dirty')
        status = self.git('status', '--porcelain', '--untracked-files=all')
        self.assertIn('new.cpp', status)
        self.assertIn('old-evidence.txt', status)

    def test_hotpath_refuses_symlinked_parent_without_touching_target(self):
        # Run the real early guard with a minimal script-owned ROOT; it must
        # stop before any compilation/removal even though no sources are here.
        tools = self.root / 'tools'
        tools.mkdir()
        command = tools / 'audit_hotpath.command'
        command.write_bytes((ROOT / 'tools/audit_hotpath.command').read_bytes())
        foreign = Path(self.tmp.name) / 'foreign'
        (foreign / 'hotpath').mkdir(parents=True)
        marker = foreign / 'hotpath/keep.txt'
        marker.write_text('preserve')
        (self.root / '.preflight-macos').symlink_to(foreign, target_is_directory=True)
        result = subprocess.run(['bash', str(command)], capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 4)
        self.assertIn('refusing symlinked preflight workspace', result.stderr)
        self.assertEqual(marker.read_text(), 'preserve')


if __name__ == '__main__':
    unittest.main()
