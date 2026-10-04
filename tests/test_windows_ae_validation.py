"""Portable checks for the Windows AE validation packet; no AE execution."""
from pathlib import Path
import json
import re
from unittest.mock import patch
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
            self.assertIn('"reset_owned_after_capture": true', (folder/'run.jsx').read_text())

    def test_prepared_identity_rejects_stale_or_foreign_records(self):
        good = dict(schema=1, run_id='a'*32, status='PREPARED', item_ids=[1,2,3,4,5], comp_id=2, chain_id=3)
        self.assertEqual(w.validate_prepared(good, 'a'*32), good)
        for changes in [dict(run_id='b'*32), dict(status='FAIL'), dict(schema=2),
                        dict(item_ids=[1,2,3,4,4]), dict(item_ids=[True,2,3,4,5]),
                        dict(item_ids=[1,2,3,4]), dict(comp_id=9), dict(chain_id=2), dict(comp_id=True), dict(schema=True)]:
            with self.subTest(changes=changes), self.assertRaises(ValueError):
                w.validate_prepared(dict(good, **changes), 'a'*32)

    def phased_case(self, failure=None):
        with tempfile.TemporaryDirectory() as tmp:
            folder = Path(tmp)
            calls = []
            def transport(afterfx, script, result):
                name = script.stem
                text = script.read_text()
                config = json.loads(re.search(r"egfxPhased(?:Prepare|Capture)\((.+)\);", text.splitlines()[-1]).group(1))
                calls.append(name)
                record = dict(run_id=config['run_id'], status='PREPARED' if name=='prepare' else
                              ('CAPTURED' if name=='cleanup' else ('READY' if name.endswith('-set') else 'EXPORTED')))
                if name=='prepare':
                    record.update(schema=1, item_ids=[1,2,3,4,5], comp_id=2, chain_id=3)
                else:
                    self.assertEqual(config['prepared']['item_ids'], [1,2,3,4,5])
                if name == failure: record['status'] = 'FAIL'
                result.write_text(json.dumps(record))
                return dict(returncode=0, log='mock')
            with patch.object(w, 'run_jsx', side_effect=transport):
                if failure:
                    with self.assertRaises(ValueError): w.run_phased_smoke(Path('AfterFX.exe'), folder, 'a'*32)
                    self.assertEqual(calls[-1], failure)
                else:
                    results = w.run_phased_smoke(Path('AfterFX.exe'), folder, 'a'*32)
                    expected = ['prepare'] + [f+'-'+a for f in w.FRAMES for a in ('set','export')] + ['cleanup']
                    self.assertEqual(calls, expected)
                    self.assertEqual(len(results), 22)

    def test_phased_smoke_keeps_set_export_and_cleanup_order(self):
        self.phased_case()

    def test_phased_smoke_stops_on_failed_host_step(self):
        for failure in ('prepare','static_a-set','chain_before_corner-export','cleanup'):
            with self.subTest(failure=failure): self.phased_case(failure)

    def test_phase_script_refuses_overwriting_old_evidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            folder = Path(tmp)
            w.write_smoke_phase(folder, 'prepare.jsx', {}, 'egfxPhasedPrepare')
            with self.assertRaises(FileExistsError):
                w.write_smoke_phase(folder, 'prepare.jsx', {}, 'egfxPhasedPrepare')
            with self.assertRaises(ValueError):
                w.write_smoke_phase(folder, 'other.jsx', {}, 'foreignFunction')

    def test_capture_requires_clean_followup_before_roundtrip(self):
        with tempfile.TemporaryDirectory(prefix='egfx-followup-') as tmp:
            folder = Path(tmp)
            for guard, revision, accepted in [('CLEAN','2',True), ('DIRTY_UNAVAILABLE','1',True),
                                              ('DIRTY','1',False), ('NOT_CHECKED','1',False),
                                              ('DIRTY_UNAVAILABLE','2',False)]:
                (folder/'capture.json').write_text(json.dumps(dict(run_id='test', status='CAPTURED',
                    fresh_guard=guard, fresh_project_revision=revision)))
                with patch.object(w, 'validate_frames', return_value={'status':'PASS'}), \
                     patch.object(w, 'sha256_file', return_value='test'):
                    if accepted:
                        self.assertEqual(w.inspect_capture(folder, 'test')['status'], 'PASS')
                    else:
                        with self.assertRaises(ValueError): w.inspect_capture(folder, 'test')

    def roundtrip_case(self, status="PASS", code=0, stale=False, corrupt=False):
        with tempfile.TemporaryDirectory(prefix="egfx-roundtrip-unit-") as tmp:
            parent = Path(tmp)
            def reserve(**kwargs):
                root = parent / "run"
                root.mkdir()
                return str(root)
            def transport(afterfx, script, result):
                config = json.loads(re.search(r"ELASTICGRID_ROUNDTRIP_CONFIG = (.+);", script.read_text()).group(1))
                folder = Path(config["folder"])
                folder.mkdir()
                (folder / "project.aep").write_bytes(b"owned fixture; state assertions reported by mocked AE")
                if corrupt:
                    (folder / "frame.png").write_bytes(b"not a PNG")
                else:
                    w.pattern(folder / "frame.png", 480, 270)
                result.write_text(json.dumps(dict(schema=1, run_id="wrong" if stale else config["run_id"],
                    build_id=config["build_id"], status=status, exit_code=code)))
                return dict(returncode=0, log="mock transport")
            with patch.object(w.tempfile, "mkdtemp", side_effect=reserve), patch.object(w, "run_jsx", side_effect=transport):
                return w.run_roundtrip(Path("AfterFX.exe"), self.build())

    def test_roundtrip_requires_decodable_frame_and_matching_completion(self):
        self.assertEqual(self.roundtrip_case()["status"], "PASS")
        with self.assertRaises(ValueError):
            self.roundtrip_case(corrupt=True)
        with self.assertRaises(ValueError):
            self.roundtrip_case(stale=True)

    def test_roundtrip_never_accepts_script_or_cleanup_failure(self):
        with self.assertRaises(ValueError):
            self.roundtrip_case(status="FAIL", code=48)
        with self.assertRaises(ValueError):
            self.roundtrip_case(status="PASS", code=48)

    def test_roundtrip_requires_completion_even_with_zero_transport_exit(self):
        with tempfile.TemporaryDirectory(prefix="egfx-roundtrip-no-result-") as tmp:
            root = Path(tmp) / "run"
            root.mkdir()
            with patch.object(w.tempfile, "mkdtemp", return_value=str(root)), patch.object(w, "run_jsx", side_effect=TimeoutError("missing completion")):
                with self.assertRaises(TimeoutError):
                    w.run_roundtrip(Path("AfterFX.exe"), self.build())

    def test_non_windows_runtime_gate_fails_closed(self):
        if sys.platform.startswith('win'):
            self.skipTest('non-Windows guard is not applicable')
        with self.assertRaises(RuntimeError):
            w._require_windows()


if __name__=='__main__':
    unittest.main()
