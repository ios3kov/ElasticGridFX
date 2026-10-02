"""Windows .aex packaging identity checks; no build or installation."""
from pathlib import Path
import json
import sys
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import package_windows as p


class WindowsPackageTests(unittest.TestCase):
    def identity(self, build_id='a'*32):
        return {
            'build_id':build_id,
            'target':'x86_64-pc-windows-msvc',
            'commit':'b'*40,
            'source_state':'clean',
            'source_sha256':'c'*64,
        }

    def test_packages_exact_identified_binary_and_marks_runtime_not_run(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-package-') as tmp:
            root=Path(tmp)
            dll=root/'elasticgrid_ae.dll'
            identity=root/'BuildIdentity.json'
            build=self.identity()
            dll.write_bytes(b'MZ fixture\0ElasticGridBuildID='+build['build_id'].encode()+b'\0')
            identity.write_text(json.dumps(build),encoding='utf-8')
            aex,manifest=p.package(dll,identity,root/'out')
            self.assertEqual(aex.name,'FSTR Stretch.aex')
            record=json.loads(manifest.read_text(encoding='utf-8'))
            self.assertEqual(record['build'],build)
            self.assertEqual(record['sha256'],p.sha256(aex))
            self.assertEqual(set(record['validation'].values()),{'NOT RUN'})

    def test_mismatched_or_non_windows_identity_is_refused(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-package-bad-') as tmp:
            root=Path(tmp)
            dll=root/'elasticgrid_ae.dll'
            identity=root/'BuildIdentity.json'
            dll.write_bytes(b'MZ fixture\0ElasticGridBuildID='+b'a'*32+b'\0')
            bad=self.identity('d'*32)
            identity.write_text(json.dumps(bad),encoding='utf-8')
            with self.assertRaises(ValueError):
                p.package(dll,identity,root/'out1')
            bad=self.identity()
            bad['target']='aarch64-apple-darwin'
            identity.write_text(json.dumps(bad),encoding='utf-8')
            with self.assertRaises(ValueError):
                p.package(dll,identity,root/'out2')

    def test_existing_output_is_not_overwritten(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-package-existing-') as tmp:
            root=Path(tmp)
            dll=root/'elasticgrid_ae.dll'
            identity=root/'BuildIdentity.json'
            build=self.identity()
            dll.write_bytes(b'MZ fixture\0ElasticGridBuildID='+build['build_id'].encode()+b'\0')
            identity.write_text(json.dumps(build),encoding='utf-8')
            out=root/'out'
            out.mkdir()
            (out/'FSTR Stretch.aex').write_bytes(b'existing')
            with self.assertRaises(FileExistsError):
                p.package(dll,identity,out)


if __name__=='__main__':
    unittest.main()
