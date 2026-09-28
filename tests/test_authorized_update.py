"""Isolated replacement/rollback fixtures; native macOS cases are explicit."""
import ctypes
import hashlib
import json
import os
from pathlib import Path
import platform
import plistlib
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
import authorized_update as au
import build_identity as bi


def simulated_swap(a, b):
    """Unit fixture ONLY, not atomic and never shipped as a production fallback."""
    temp = a.parent/'fixture-exchange'
    os.rename(a, temp); os.rename(b, a); os.rename(temp, b)


class UpdateSafety(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='egfx-upgrade-test-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.home = self.root/'home with spaces'
        self.old = self.home/au.TARGET
        (self.old/'Contents/MacOS').mkdir(parents=True)
        (self.old/'Contents/MacOS/ElasticGrid').write_bytes(b'old fixture binary')
        (self.old/'Contents/MacOS/ElasticGrid').chmod(0o755)
        (self.old/'Contents/Info.plist').write_bytes(plistlib.dumps({'CFBundleIdentifier':'com.elasticgrid.fx'}))
        (self.old/'Contents/keep').write_bytes(b'original extra file')
        self.sha = bi.digest((self.old/'Contents/MacOS/ElasticGrid').read_bytes())
        self.before = au.snapshot(self.old)
        self.new = self.root/'input/ElasticGrid.plugin'
        meta = bi.identity(dict(version='0.9.0', commit='a'*40, source_state='clean', source_sha256='b'*64),
                           'aarch64-apple-darwin', 'release', {}, {})
        bi.dump(self.new/'Contents/Resources/BuildIdentity.json',meta)
        (self.new/'Contents/MacOS').mkdir(parents=True)
        self.marker = 'ElasticGridBuildID='+meta['build_id']
        (self.new/'Contents/MacOS/ElasticGrid').write_bytes(self.marker.encode())
        (self.new/'Contents/MacOS/ElasticGrid').chmod(0o755)
        (self.new/'Contents/Info.plist').write_bytes(plistlib.dumps({'CFBundleIdentifier':'com.elasticgrid.fx','CFBundleExecutable':'ElasticGrid','CFBundlePackageType':'eFKT'}))
        self.package, self.manfile = self.root/'candidate.zip', self.root/'manifest.json'
        self.seal()
        self.sibling = self.old.parent/'OtherVendor.plugin'
        self.sibling.mkdir(); (self.sibling/'keep').write_bytes(b'untouched sibling')
        self.sibling_snapshot = au.snapshot(self.sibling)

    def seal(self):
        bi.seal(self.new, self.package, self.manfile)
        self.manifest = json.loads(self.manfile.read_text())

    def apply(self, **kwargs):
        return au.replace(self.home, self.package, self.manifest, old_sha=self.sha,
                          swap=kwargs.pop('swap', simulated_swap),
                          verify_signature=kwargs.pop('verify_signature', lambda _:None),
                          check_hosts=kwargs.pop('check_hosts', lambda:None), **kwargs)

    def rollback(self, **kwargs):
        return au.rollback(self.home, self.manifest, old_sha=self.sha,
                           swap=kwargs.pop('swap', simulated_swap), check_hosts=lambda:None)

    def assert_old(self):
        self.assertEqual(au.snapshot(self.old),self.before)
        self.assertEqual(au.snapshot(self.sibling),self.sibling_snapshot)

    def test_install_retains_entire_original_inode_and_content(self):
        file, record = self.apply()
        self.assertEqual(record['state'],'INSTALLED_FOR_TEST')
        self.assertEqual(au.snapshot(file.parent/'previous.plugin'),self.before)
        au.verify_new(self.old,self.manifest)
        self.assertEqual(au.snapshot(self.sibling),self.sibling_snapshot)
        self.assertEqual(record['functional_ae'],'NOT RUN')
        self.assertEqual(record['release'],'BLOCKED')

    def test_explicit_rollback_restores_exact_original(self):
        self.apply(); file, record=self.rollback()
        self.assertEqual(record['state'],'ROLLED_BACK'); self.assert_old()
        au.verify_new(file.parent/'previous.plugin',self.manifest)

    def test_repeat_install_is_idempotent(self):
        file, _ = self.apply(); saved=au.snapshot(file.parent/'previous.plugin')
        again, record=self.apply()
        self.assertEqual(file,again); self.assertEqual(record['state'],'INSTALLED_FOR_TEST')
        self.assertEqual(au.snapshot(file.parent/'previous.plugin'),saved)

    def test_wrong_original_hash_refuses_without_modification(self):
        with self.assertRaises(ValueError):
            au.replace(self.home,self.package,self.manifest,old_sha='0'*64,swap=simulated_swap,verify_signature=lambda _:None,check_hosts=lambda:None)
        self.assert_old()

    def test_wrong_bundle_identifier_refuses(self):
        (self.old/'Contents/Info.plist').write_bytes(plistlib.dumps({'CFBundleIdentifier':'other'}))
        before=au.snapshot(self.old)
        with self.assertRaises(ValueError): self.apply()
        self.assertEqual(au.snapshot(self.old),before)

    def test_stopped_host_guard_runs_before_any_write(self):
        def running(): raise ValueError('host running')
        with self.assertRaises(ValueError): self.apply(check_hosts=running)
        self.assertFalse((self.home/au.BACKUPS).exists()); self.assert_old()

    def test_host_started_during_preparation_blocks_swap(self):
        calls=0
        def guard():
            nonlocal calls
            calls+=1
            if calls==2: raise ValueError('host started')
        file, record=self.apply(check_hosts=guard)
        self.assertEqual(record['state'],'FAILED_BEFORE_SWAP'); self.assert_old()

    def test_candidate_corruption_preserves_original(self):
        self.package.write_bytes(b'corrupt')
        _, record=self.apply()
        self.assertEqual(record['state'],'FAILED_BEFORE_SWAP'); self.assert_old()

    def test_bad_signature_preserves_original(self):
        def invalid(_): raise ValueError('invalid signature')
        _, record=self.apply(verify_signature=invalid)
        self.assertEqual(record['state'],'FAILED_BEFORE_SWAP'); self.assert_old()

    def test_failed_exchange_keeps_original(self):
        def fail(a,b): raise OSError('volume refuses swap')
        _, record=self.apply(swap=fail)
        self.assertEqual(record['state'],'FAILED_BEFORE_SWAP'); self.assert_old()

    def test_post_swap_signature_failure_restores_original(self):
        def check(path):
            if path==self.old: raise ValueError('post-install verification failed')
        file, record=self.apply(verify_signature=check)
        self.assertEqual(record['state'],'ROLLED_BACK'); self.assert_old()
        self.assertTrue((file.parent/'previous.plugin').exists())

    def test_interrupt_at_syscall_return_rolls_back_actual_state(self):
        calls = 0
        def interrupted(a, b):
            nonlocal calls
            calls += 1
            simulated_swap(a, b)
            if calls == 1:
                raise KeyboardInterrupt('at syscall return')
        _, record = self.apply(swap=interrupted)
        self.assertEqual(record['state'], 'ROLLED_BACK')
        self.assert_old()

    def test_prepared_receipt_exists_before_swap(self):
        def exchange(a,b):
            record=json.loads((b.parent/'receipt.json').read_text())
            self.assertEqual(record['state'],'PREPARED')
            self.assertEqual(record['old_snapshot'],self.before)
            simulated_swap(a,b)
        self.apply(swap=exchange)

    def test_crash_receipt_after_swap_is_recoverable(self):
        file, record=self.apply()
        record['state']='PREPARED'; au.save_receipt(file,record)
        self.rollback(); self.assert_old()

    def test_prepared_receipt_before_swap_does_not_destroy_original(self):
        file, record=self.apply()
        simulated_swap(self.old,file.parent/'previous.plugin')
        record['state']='PREPARED'; au.save_receipt(file,record)
        self.rollback(); self.assert_old()

    def test_modified_backup_blocks_rollback(self):
        file, _=self.apply()
        (file.parent/'previous.plugin/Contents/keep').write_bytes(b'changed')
        now=au.snapshot(self.old)
        with self.assertRaises(ValueError): self.rollback()
        self.assertEqual(au.snapshot(self.old),now)

    def test_modified_installed_candidate_blocks_rollback(self):
        file,_=self.apply(); saved=au.snapshot(file.parent/'previous.plugin')
        (self.old/'Contents/unexpected').write_bytes(b'new data')
        with self.assertRaises(ValueError): self.rollback()
        self.assertEqual(au.snapshot(file.parent/'previous.plugin'),saved)
        self.assertTrue((self.old/'Contents/unexpected').exists())

    def test_symlinked_original_refused(self):
        real=self.old.parent/'original-held.plugin'; os.rename(self.old,real)
        self.old.symlink_to(real)
        with self.assertRaises(ValueError): self.apply()
        self.assertTrue(self.old.is_symlink()); self.assertEqual(au.snapshot(real),self.before)

    def test_symlinked_backup_root_refused(self):
        backups=self.home/au.BACKUPS; backups.parent.mkdir(parents=True)
        foreign=self.root/'foreign'; foreign.mkdir(); (foreign/'keep').write_text('preserve')
        backups.symlink_to(foreign)
        with self.assertRaises(ValueError): self.apply()
        self.assertEqual((foreign/'keep').read_text(),'preserve'); self.assert_old()

    def test_symlinked_lock_refused(self):
        backups=self.home/au.BACKUPS; backups.mkdir(parents=True)
        (backups/'.elasticgrid-install.lock').symlink_to(self.sibling/'keep')
        with self.assertRaises((ValueError,OSError)): self.apply()
        self.assert_old()

    def test_report_contains_no_native_plugin_and_keeps_runtime_not_run(self):
        file,record=self.apply()
        report=au.report(self.home,file,record)
        import zipfile
        with zipfile.ZipFile(report) as z:
            self.assertEqual(set(z.namelist()),{'report.json','report.txt'})
            data=json.loads(z.read('report.json'))
            self.assertEqual(data['loaded_identity'],'NOT RUN')
            self.assertNotIn(str(self.home),z.read('report.json').decode())

    def test_cli_requires_explicit_action(self):
        result=subprocess.run([sys.executable,str(ROOT/'tools/authorized_update.py')],capture_output=True,text=True)
        self.assertNotEqual(result.returncode,0)
        self.assertIn('Choose --apply',result.stderr)


@unittest.skipUnless(platform.system()=='Darwin','Real directory exchange/signature requires macOS')
class MacUpdate(unittest.TestCase):
    setUp = UpdateSafety.setUp
    seal = UpdateSafety.seal
    apply = UpdateSafety.apply
    rollback = UpdateSafety.rollback
    assert_old = UpdateSafety.assert_old
    # These cases use the actual Mac exchange, separate from portable mocks.
    def test_real_signed_install_restore_preserves_xattrs(self):
        source=self.root/'candidate.c'
        source.write_text('const char* marker(void) { return "'+self.marker+'"; }\n')
        subprocess.run(['clang','-dynamiclib',str(source),'-o',str(self.new/'Contents/MacOS/ElasticGrid')],check=True,timeout=30)
        subprocess.run(['/usr/bin/codesign','--force','--sign','-',str(self.new)],check=True,timeout=30)
        os.setxattr(self.old/'Contents/keep','com.elasticgrid.test-preserve',b'original metadata')
        self.before=au.snapshot(self.old)
        self.seal()
        file,record=self.apply(swap=au.native_swap,verify_signature=au.signature)
        self.assertEqual(record['state'],'INSTALLED_FOR_TEST',record)
        self.assertEqual(au.snapshot(file.parent/'previous.plugin'),self.before)
        self.rollback(swap=au.native_swap)
        self.assert_old()
        self.assertEqual(os.getxattr(self.old/'Contents/keep','com.elasticgrid.test-preserve'),b'original metadata')

    def test_real_exchange_postcheck_failure_rolls_back(self):
        def postcheck(path):
            if path == self.old:
                raise ValueError('Injected post-install check failure')
        _, record = self.apply(swap=au.native_swap, verify_signature=postcheck)
        self.assertEqual(record['state'], 'ROLLED_BACK', record)
        self.assert_old()

    def test_real_exchange_failure_preserves_both(self):
        file,record=self.apply(swap=lambda a,b:au.native_swap(a,b.parent/'missing.plugin'))
        self.assertEqual(record['state'],'FAILED_BEFORE_SWAP'); self.assert_old()


if __name__=='__main__': unittest.main()
