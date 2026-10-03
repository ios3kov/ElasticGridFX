"""Portable preparation checks for Windows first-application validation."""
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import windows_first_application as f


class WindowsFirstApplicationTests(unittest.TestCase):
    def test_control_scripts_do_not_pollute_fresh_evidence_folder(self):
        with tempfile.TemporaryDirectory(prefix='egfx-first-prep-') as tmp:
            root=Path(tmp)
            evidence=root/('EGFX-first-'+'a'*32)
            control=root/'control'
            evidence.mkdir()
            control.mkdir()
            first=f._write_wrapper(control,evidence,'a'*32,32,'solid',False)
            follow=f._write_followup(control,evidence,'a'*32)
            self.assertEqual(list(evidence.iterdir()),[])
            self.assertTrue(first.is_file())
            self.assertTrue(follow.is_file())
            text=first.read_text(encoding='utf-8')
            self.assertIn(str(evidence).replace('\\','\\\\'),text)
            node=shutil.which('node')
            if node:
                for script in (first,follow):
                    checked=subprocess.run([node,'--check',str(script)],capture_output=True,text=True)
                    self.assertEqual(checked.returncode,0,checked.stderr)

    def test_single_png_rejects_ambiguity(self):
        with tempfile.TemporaryDirectory(prefix='egfx-first-png-') as tmp:
            root=Path(tmp)
            with self.assertRaises(ValueError):
                f._single_png(root,'bypass')
            (root/'bypass-00000.png').write_bytes(b'x')
            self.assertEqual(f._single_png(root,'bypass').name,'bypass-00000.png')
            (root/'bypass-00001.png').write_bytes(b'x')
            with self.assertRaises(ValueError):
                f._single_png(root,'bypass')


if __name__=='__main__':
    unittest.main()
