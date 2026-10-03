"""Portable checks for the Windows AE validation packet; no AE execution."""
from pathlib import Path
import json
import sys
import tempfile
import unittest

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import build_identity as bi
import windows_ae_validation as w


class WindowsAEValidationTests(unittest.TestCase):
    def build(self):
        record={
            'version':'0.9.3',
            'commit':'b'*40,
            'source_state':'clean',
            'source_sha256':'c'*64,
        }
        return bi.identity(record,'x86_64-pc-windows-msvc','release',
                           {'rustc':'rustc-test','cxx':'msvc-test'}, {})

    def test_manifest_and_candidate_identity_are_exact(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-ae-') as tmp:
            root=Path(tmp)
            build=self.build()
            payload=b'MZ fixture\0ElasticGridBuildID='+build['build_id'].encode()+b'\0'
            candidate=root/'FSTR Stretch.aex'
            installed=root/'installed.aex'
            candidate.write_bytes(payload)
            installed.write_bytes(payload)
            manifest={
                'schema':1,
                'artifact':'FSTR Stretch.aex',
                'sha256':w.sha256_file(candidate),
                'build':build,
            }
            manifest_path=root/'manifest.json'
            manifest_path.write_text(json.dumps(manifest),encoding='utf-8')
            loaded=w.load_manifest(manifest_path)
            exact=w.verify_candidate(candidate,installed,loaded)
            self.assertEqual(exact['build_id'],build['build_id'])
            installed.write_bytes(payload+b'changed')
            with self.assertRaises(ValueError):
                w.verify_candidate(candidate,installed,loaded)

    def test_workspace_generation_is_nonexecuting_and_self_identified(self):
        with tempfile.TemporaryDirectory(prefix='egfx-win-workspace-') as tmp:
            folder,meta=w.prepare_workspace(Path(tmp),self.build())
            self.assertEqual(meta['status'],'NOT RUN')
            self.assertFalse(meta['actual_ae_execution'])
            self.assertTrue((folder/'pattern.png').is_file())
            for name in ('arm.jsx','disarm.jsx','run.jsx','run.json'):
                self.assertTrue((folder/name).is_file())

    def test_non_windows_runtime_gate_fails_closed(self):
        if sys.platform.startswith('win'):
            self.skipTest('non-Windows guard is not applicable')
        with self.assertRaises(RuntimeError):
            w._require_windows()


if __name__=='__main__':
    unittest.main()
