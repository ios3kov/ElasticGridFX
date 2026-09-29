"""Regression for updater ZIP Unix execute-bit identity. No installation."""
from pathlib import Path
import json
import sys
import tempfile
import unittest
import zipfile

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import package_authorized_update as p


class PackagePermissions(unittest.TestCase):
    def test_executable_python_is_not_downgraded_to_0644(self):
        with tempfile.TemporaryDirectory(prefix='egfx-update-zip-') as tmp:
            out=Path(tmp)/'update.zip'
            files={
                'tools/authorized_update.py':{'sha256':'0'*64,'executable':True},
                'tools/helper.py':{'sha256':'1'*64,'executable':False},
            }
            identity={'schema':1,'files':files}
            content={
                'tools/authorized_update.py':b'#!/usr/bin/env python3\n',
                'tools/helper.py':b'pass\n',
                'InstallToolIdentity.json':json.dumps(identity).encode(),
            }
            p.write_archive(out,'Pkg/',content,files)
            with zipfile.ZipFile(out) as z:
                modes={name:z.getinfo('Pkg/'+name).external_attr>>16 for name in content}
                self.assertTrue(modes['tools/authorized_update.py'] & 0o111)
                self.assertFalse(modes['tools/helper.py'] & 0o111)
                self.assertFalse(modes['InstallToolIdentity.json'] & 0o111)

    def test_unknown_member_is_refused(self):
        with self.assertRaises(ValueError):
            p.archive_mode('unexpected.py',{})


if __name__=='__main__':unittest.main()
