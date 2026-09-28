"""Execute the real read-only launcher, including failure before Python starts."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile
import types
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / 'CHECK_UPDATE_START_MAC.command'
SOURCE = SCRIPT.read_text().split("<<'EGFX_PY'\n", 1)[1].split('\nEGFX_PY\n', 1)[0]


def module():
    obj = types.ModuleType('startup_check_fixture')
    exec(compile(SOURCE, str(SCRIPT), 'exec'), obj.__dict__)
    return obj


class StartupCheck(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='egfx-start-test-')
        self.addCleanup(self.tmp.cleanup)
        self.home = Path(self.tmp.name).resolve()
        (self.home / 'Desktop').mkdir()
        self.tool = module()

    def package(self):
        root = self.home / 'Downloads/ElasticGridFX-Test-Update'
        root.mkdir(parents=True)
        # Syntax parsing must never run this otherwise-destructive fixture.
        payload = b'raise RuntimeError("THIS_CODE_MUST_NOT_RUN")\n'
        (root / 'module.py').write_bytes(payload)
        (root / 'module.py').chmod(0o644)
        meta = dict(commit='fixture', files={'module.py':dict(sha256=hashlib.sha256(payload).hexdigest(), executable=False)})
        raw = json.dumps(meta).encode()
        (root / 'InstallToolIdentity.json').write_bytes(raw)
        self.tool.IDENTITY_SHA = hashlib.sha256(raw).hexdigest()
        return root

    def test_valid_package_is_parsed_without_execution(self):
        root = self.package()
        before = {p.name:p.read_bytes() for p in root.iterdir()}
        self.assertEqual(self.tool.inspect_package(root)['issues'], [])
        self.assertEqual(before, {p.name:p.read_bytes() for p in root.iterdir()})

    def test_changed_bytes_permissions_and_identity_are_reported(self):
        root = self.package()
        path = root / 'module.py'
        path.write_bytes(b'this is invalid python !!!')
        path.chmod(0o755)
        issues = self.tool.inspect_package(root)['issues']
        self.assertTrue(any('CHANGED_BYTES' in x for x in issues))
        self.assertTrue(any('CHANGED_EXECUTE_BIT' in x for x in issues))
        self.assertTrue(any('SyntaxError' in x for x in issues))
        (root / 'InstallToolIdentity.json').write_text('{}')
        self.assertIn('NOT_THE_DELIVERED', self.tool.inspect_package(root)['issues'][0])

    def test_symlink_and_oversized_file_are_refused(self):
        source = self.home / 'source'
        source.write_bytes(b'x' * 64)
        link = self.home / 'link'
        link.symlink_to(source)
        with self.assertRaises(ValueError): self.tool.read_safe(link)
        with self.assertRaises(ValueError): self.tool.read_safe(source, 32)

    def test_search_is_bounded_and_does_not_follow_links(self):
        root = self.package()
        self.assertEqual(self.tool.locate([self.home/'Downloads'])[0], [root])
        deep = self.home/'a/b/c/d/e'; deep.mkdir(parents=True)
        (deep/'InstallToolIdentity.json').write_text('{}')
        (self.home/'Desktop/link').symlink_to(root)
        self.assertEqual(self.tool.locate([self.home/'Desktop', self.home/'a'], limit=3)[0], [])

    def test_only_adobe_processes_are_reported(self):
        text = '123 /Applications/Adobe After Effects.app/Contents/MacOS/After Effects\n'
        text += '456 /Library/Adobe/dynamiclinkmanager\n789 /Applications/Safari.app/Safari\n'
        self.assertEqual([v['pid'] for v in self.tool.hosts(text)], [123, 456])

    def test_current_blockers_do_not_become_historical_claims(self):
        root = self.package()
        with patch.object(self.tool.Path,'home',return_value=self.home), \
             patch.object(self.tool.platform,'system',return_value='Darwin'), \
             patch.object(self.tool.platform,'machine',return_value='arm64'), \
             patch.object(self.tool.os,'geteuid',return_value=501), \
             patch.object(self.tool.subprocess,'check_output',return_value='42 /Library/Adobe/dynamiclinkmanager\n'):
            result = self.tool.run(SCRIPT, str(root))
        self.assertFalse(result['installation_attempted'])
        self.assertEqual(result['historical_failure_reason'], 'UNKNOWN')
        self.assertIn('ADOBE_HOST_OR_RENDER_HELPER_IS_STILL_RUNNING', result['blockers'])
        self.assertFalse((self.home/'Library/Application Support/ElasticGridFX').exists())

    def execute_shell(self, fake_python=None, missing=False):
        env = dict(os.environ, HOME=str(self.home))
        if fake_python is not None or missing:
            fake = self.home/'bin'; fake.mkdir()
            env['PATH'] = str(fake)
            if not missing:
                py = fake/'python3'
                py.write_text('#!/bin/sh\n'+fake_python+'\n')
                py.chmod(0o755)
        result = subprocess.run(['/bin/bash',str(SCRIPT)],env=env,capture_output=True,text=True,timeout=20)
        reports = list((self.home/'Desktop').glob('EGFX-StartCheck.*/report.txt'))
        self.assertEqual(len(reports),1, result.stdout+result.stderr)
        self.assertEqual(reports[0].parent.stat().st_mode & 0o777, 0o700)
        self.assertEqual(reports[0].stat().st_mode & 0o777, 0o600)
        return result, reports[0].read_text()

    def test_missing_python_still_leaves_report(self):
        result, report = self.execute_shell(missing=True)
        self.assertEqual(result.returncode,2)
        self.assertIn('PYTHON_NOT_FOUND',report)

    def test_interpreter_failure_still_leaves_report(self):
        result, report = self.execute_shell('echo "BOOTSTRAP_FAILURE_FIXTURE" >&2\nexit 71')
        self.assertEqual(result.returncode,71)
        self.assertIn('BOOTSTRAP_FAILURE_FIXTURE',report)
        self.assertIn('PYTHON_EXIT_CODE=71',report)

    def test_actual_launcher_only_writes_diagnostic(self):
        (self.home/'sentinel').write_text('old user data')
        result, report = self.execute_shell()
        self.assertEqual(result.returncode,0,result.stderr)
        self.assertIn('"installation_attempted": false',report)
        self.assertIn('"historical_failure_reason": "UNKNOWN"',report)
        self.assertEqual((self.home/'sentinel').read_text(),'old user data')
        self.assertFalse((self.home/'Library').exists())
        self.assertNotIn(str(self.home),report)


if __name__ == '__main__':
    unittest.main()
