"""Regression for target-AE acceptance ZIP execute-bit identity."""
from pathlib import Path
import json,sys,tempfile,unittest,zipfile
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import package_target_ae_acceptance as p

class AcceptancePackagePermissions(unittest.TestCase):
    def test_executable_python_members_keep_execute_bits(self):
        with tempfile.TemporaryDirectory(prefix='egfx-acc-zip-') as tmp:
            out=Path(tmp)/'a.zip'
            files={
              'tools/live_identity.py':{'sha256':'0'*64,'executable':True},
              'tools/target_ae_acceptance.py':{'sha256':'1'*64,'executable':True},
              'tests/ae_runtime_smoke.jsx':{'sha256':'2'*64,'executable':False},
            }
            content={
              'tools/live_identity.py':b'#!/usr/bin/env python3\n',
              'tools/target_ae_acceptance.py':b'#!/usr/bin/env python3\n',
              'tests/ae_runtime_smoke.jsx':b'// test\n',
              'AcceptanceToolIdentity.json':json.dumps({'files':files}).encode(),
            }
            p.write_archive(out,'Pkg/',content,files)
            with zipfile.ZipFile(out) as z:
                for name in ('tools/live_identity.py','tools/target_ae_acceptance.py'):
                    self.assertTrue((z.getinfo('Pkg/'+name).external_attr>>16)&0o111)
                self.assertFalse((z.getinfo('Pkg/tests/ae_runtime_smoke.jsx').external_attr>>16)&0o111)

    def test_unknown_member_refused(self):
        with self.assertRaises(ValueError): p.archive_mode('unexpected.py',{})

if __name__=='__main__': unittest.main()
