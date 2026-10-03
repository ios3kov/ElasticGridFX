"""Build/static Windows validation record tests; no PE execution."""
from pathlib import Path
import json
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import build_identity as bi
import finalize_windows_validation as f


class FinalizeWindowsValidationTests(unittest.TestCase):
    def fixture(self,root:Path):
        source={'version':'0.9.3','commit':'b'*40,'source_state':'clean','source_sha256':'c'*64}
        build=bi.identity(source,'x86_64-pc-windows-msvc','release',
                          {'rustc':'r','cxx':'c'}, {})
        aex=root/'FSTR Stretch.aex'; aex.write_bytes(b'MZ fixture')
        manifest={'schema':1,'artifact':'FSTR Stretch.aex',
                  'sha256':bi.digest(aex.read_bytes()),'build':build}
        mp=root/'manifest.json'; mp.write_text(json.dumps(manifest),encoding='utf-8')
        headers=root/'headers.txt'; headers.write_text('8664 machine (x64)\n',encoding='utf-8')
        exports=root/'exports.txt'; exports.write_text('ordinal hint RVA name\n1 0 0000 EffectMain\n',encoding='utf-8')
        resources=root/'resources.txt'; resources.write_text('Resource type: PiPL\n',encoding='utf-8')
        pipl=root/'pipl.bin'; pipl.write_bytes(b'PiPL fixture')
        return mp,aex,headers,exports,resources,pipl

    def test_record_keeps_runtime_not_run(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-final-') as tmp:
            args=self.fixture(Path(tmp))
            env={'GITHUB_ACTIONS':'true','GITHUB_SHA':'b'*40,'GITHUB_RUN_ID':'123','GITHUB_RUN_ATTEMPT':'1'}
            with patch.object(f.pipl_verify,'embedded_pipl',return_value=b'PiPL fixture'):
                record=f.validate(*(p.resolve() for p in args),env=env)
            self.assertEqual(record['checks']['pe_x64'],'PASS')
            self.assertEqual(record['checks']['pipl_resource_byte_exact'],'PASS')
            self.assertEqual(record['checks']['after_effects_load'],'NOT RUN')
            self.assertIn('not After Effects runtime',record['scope'])

    def test_missing_effectmain_fails_closed(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-final-bad-') as tmp:
            args=list(self.fixture(Path(tmp)))
            args[3].write_text('no export here\n',encoding='utf-8')
            env={'GITHUB_ACTIONS':'true','GITHUB_SHA':'b'*40,'GITHUB_RUN_ID':'123','GITHUB_RUN_ATTEMPT':'1'}
            with patch.object(f.pipl_verify,'embedded_pipl',return_value=b'PiPL fixture'):
                with self.assertRaises(ValueError):
                    f.validate(*(p.resolve() for p in args),env=env)


    def test_non_ci_or_wrong_sha_cannot_emit_pass_record(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-final-env-') as tmp:
            args=self.fixture(Path(tmp))
            with patch.object(f.pipl_verify,'embedded_pipl',return_value=b'PiPL fixture'):
                with self.assertRaises(ValueError):
                    f.validate(*(p.resolve() for p in args),env={})
                bad={'GITHUB_ACTIONS':'true','GITHUB_SHA':'a'*40,'GITHUB_RUN_ID':'123','GITHUB_RUN_ATTEMPT':'1'}
                with self.assertRaises(ValueError):
                    f.validate(*(p.resolve() for p in args),env=bad)

if __name__=='__main__':
    unittest.main()
