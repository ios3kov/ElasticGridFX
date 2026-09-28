"""Regression for full-parent-path false positives. No user processes are killed."""
from pathlib import Path
import os
import platform
import select
import subprocess
import sys
import tempfile
import types
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / 'tools'))
import install_candidate as ic

AE = '/Applications/Adobe After Effects 2025/Adobe After Effects 2025.app/Contents/MacOS/'
START_CHECK = (ROOT/'CHECK_UPDATE_START_MAC.command').read_text().split("<<'EGFX_PY'\n", 1)[1].split('\nEGFX_PY\n', 1)[0]


class ProcessGuard(unittest.TestCase):
    def test_reported_crashpad_only_is_not_an_open_host(self):
        # Matches the reporter path from the returned report, NOT a new Mac observation.
        text = '1474 ' + AE + 'crashpad_handler\n'
        with patch.object(ic.subprocess, 'check_output', return_value=text):
            ic.adobe_hosts_stopped()
        self.assertEqual(ic.adobe_host_processes(text), [])

    def test_actual_ae_still_blocks_and_names_pid(self):
        text = '1474 '+AE+'crashpad_handler\n76889 '+AE+'After Effects\n'
        with patch.object(ic.subprocess, 'check_output', return_value=text):
            with self.assertRaises(ValueError) as caught:
                ic.adobe_hosts_stopped()
        self.assertIn('PID 76889', str(caught.exception))
        self.assertIn(AE+'After Effects', str(caught.exception))
        self.assertNotIn('1474', str(caught.exception))

    def test_dynamic_link_is_not_silently_exempted(self):
        text = '1474 '+AE+'crashpad_handler\n76896 '+AE+'dynamiclinkmanager\n'
        self.assertEqual([p['pid'] for p in ic.adobe_host_processes(text)], [76896])
        with patch.object(ic.subprocess, 'check_output', return_value=text):
            with self.assertRaisesRegex(ValueError, 'dynamiclinkmanager'):
                ic.adobe_hosts_stopped()

    def test_other_hosts_and_unknown_helpers_stay_conservative(self):
        for leaf in ('After Effects','aerender','aerendercore','Adobe Premiere Pro',
                     'Adobe Media Encoder','dynamiclinkmanager','CEPHtmlEngine',
                     'TeamProjectsLocalHub', 'crashpad_handler_fake', 'crashpad_handler Helper'):
            text = '42 '+AE+leaf+'\n'
            with self.subTest(leaf=leaf):
                self.assertEqual(len(ic.adobe_host_processes(text)), 1)

    def test_unrelated_processes_are_not_blockers(self):
        text = '0 kernel_task\n1 /sbin/launchd\n200 /Applications/Safari.app/Contents/MacOS/Safari\n'
        self.assertEqual(ic.adobe_host_processes(text), [])

    def test_failed_empty_or_malformed_collection_never_authorizes(self):
        for text in ('', ' ', 'not-a-pid '+AE+'After Effects', '42', '-1 /bin/sleep',
                     '42 /bin/sleep\n42 /bin/sleep', '12 '+AE+'After\tEffects'):
            with self.subTest(text=text), patch.object(ic.subprocess, 'check_output', return_value=text):
                with self.assertRaises(ValueError): ic.adobe_hosts_stopped()
        for error in (OSError('permission'), subprocess.TimeoutExpired('ps', 10)):
            with patch.object(ic.subprocess, 'check_output', side_effect=error):
                with self.assertRaises(type(error)): ic.adobe_hosts_stopped()

    def test_collects_full_comm_not_command_arguments(self):
        with patch.object(ic.subprocess, 'check_output', return_value='1 /sbin/launchd\n') as collect:
            ic.adobe_hosts_stopped()
        self.assertEqual(collect.call_args.args[0], ['/bin/ps', '-axww', '-o', 'pid=,comm='])
        self.assertEqual(collect.call_args.kwargs['timeout'], 10)

    def test_standalone_checker_agrees_with_installer(self):
        # Intentional duplication keeps old read-only diagnostic self-contained.
        module = types.ModuleType('embedded_process_guard_fixture')
        exec(compile(START_CHECK, 'CHECK_UPDATE_START_MAC.command', 'exec'), module.__dict__)
        for text in ('1 /sbin/launchd\n', '1474 '+AE+'crashpad_handler\n',
                     '76889 '+AE+'After Effects\n', '76896 '+AE+'dynamiclinkmanager\n',
                     '14 '+AE+'CEPHtmlEngine\n'):
            self.assertEqual(module.hosts(text), ic.adobe_host_processes(text))
        for text in ('', 'missing PID'):
            with self.assertRaises(ValueError): module.hosts(text)


@unittest.skipUnless(platform.system() == 'Darwin', 'Actual ps fixture requires macOS; not AE')
class NativeProcessGuard(unittest.TestCase):
    def test_actual_mac_ps_distinguishes_owned_reporter_and_host(self):
        with tempfile.TemporaryDirectory(prefix='egfx-process-fixture-') as tmp:
            folder = Path(tmp).resolve()
            inside = folder/'Adobe After Effects 2025.app/Contents/MacOS'
            inside.mkdir(parents=True)
            source = folder/'fixture.c'
            source.write_text('#include <stdio.h>\n#include <unistd.h>\nint main(void){puts("READY");fflush(stdout);sleep(60);return 0;}\n')
            binary = inside/'crashpad_handler'
            subprocess.run(['clang', str(source), '-o', str(binary)], check=True, timeout=30)
            children = []
            try:
                for leaf, expected in (('crashpad_handler',False), ('After Effects',True), ('dynamiclinkmanager',True)):
                    image = inside/leaf
                    if image != binary:
                        image.write_bytes(binary.read_bytes()); image.chmod(0o755)
                    child = subprocess.Popen([str(image)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                    children.append(child)
                    self.assertTrue(select.select([child.stdout], [], [], 5)[0], 'owned child startup timeout')
                    self.assertEqual(child.stdout.readline().strip(), 'READY')
                    text = subprocess.check_output(['/bin/ps','-axww','-o','pid=,comm='], text=True, timeout=10)
                    # Prove this process was listed; absence is not an ignored process.
                    rows = [line.strip().split(None,1) for line in text.splitlines()]
                    self.assertIn([str(child.pid),str(image)], rows)
                    blocking = {p['pid'] for p in ic.adobe_host_processes(text)}
                    self.assertEqual(child.pid in blocking, expected, leaf)
            finally:
                # These specific Popen children belong only to this test.
                for child in children:
                    if child.poll() is None: child.terminate()
                    try: child.communicate(timeout=5)
                    except subprocess.TimeoutExpired: child.kill(); child.communicate(timeout=5)


class UpdaterLog(unittest.TestCase):
    def run_launcher(self, fake_python=None, mock_mac=False):
        tmp = tempfile.TemporaryDirectory(prefix='egfx-launch-log-')
        self.addCleanup(tmp.cleanup)
        home = Path(tmp.name).resolve()
        (home/'Desktop').mkdir()
        script = (ROOT/'UPDATE_ELASTICGRID_MAC.command').read_text()
        # Simulated OS cases are separate from actual Mac execution above.
        if mock_mac:
            script = script.replace('$(/usr/bin/uname -s)', 'Darwin')
        launcher = home/'UPDATE_ELASTICGRID_MAC.command'
        launcher.write_text(script)
        fake = home/'bin'; fake.mkdir()
        if fake_python is not None:
            py = fake/'python3'; py.write_text('#!/bin/sh\n'+fake_python+'\n'); py.chmod(0o755)
        (home/'keep').write_text('unchanged')
        proc = subprocess.run(['/bin/bash',str(launcher)], env={**os.environ,'HOME':str(home),'PATH':str(fake)},
                              text=True,capture_output=True,timeout=20)
        logs = list((home/'Desktop').glob('EGFX-Update.*/terminal.log'))
        self.assertEqual(len(logs),1,proc.stdout+proc.stderr)
        self.assertEqual(logs[0].stat().st_mode & 0o777,0o600)
        self.assertEqual(logs[0].parent.stat().st_mode & 0o777,0o700)
        self.assertEqual((home/'keep').read_text(),'unchanged')
        self.assertFalse((home/'Library').exists())
        return proc, logs[0].read_text()

    def test_missing_python_leaves_log_before_exit(self):
        proc, log = self.run_launcher(mock_mac=True)
        self.assertEqual(proc.returncode,2)
        self.assertIn('Python 3 не найден', log)
        self.assertIn('EXIT_CODE=2',log)

    def test_python_boot_failure_preserves_output_and_code(self):
        proc, log = self.run_launcher('echo BOOT_FAILURE >&2\nexit 71',mock_mac=True)
        self.assertEqual(proc.returncode,71)
        self.assertIn('BOOT_FAILURE',proc.stdout)
        self.assertIn('BOOT_FAILURE',log)
        self.assertIn('EXIT_CODE=71',log)

    def test_actual_os_launcher_has_durable_exit_log(self):
        proc,log = self.run_launcher('echo OWNED_INTERPRETER_FIXTURE >&2\nexit 72')
        expected = 72 if platform.system()=='Darwin' else 2
        self.assertEqual(proc.returncode,expected)
        self.assertIn('EXIT_CODE='+str(expected),log)


if __name__=='__main__': unittest.main()
