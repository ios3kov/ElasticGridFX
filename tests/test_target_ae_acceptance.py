"""Target-AE orchestrator unit tests. These do not execute After Effects."""
from pathlib import Path
import platform
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'tools'))
import target_ae_acceptance as ta


class Acceptance(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='egfx-acceptance-')
        self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name).resolve()
        self.report=self.root/'reports'
        self.app=self.root/'Adobe After Effects 2025.app'; self.app.mkdir()
        self.plugin=self.root/'ElasticGrid.plugin'; self.plugin.mkdir()
        self.manifest={'build':{'build_id':'EGFX-test','commit':'fd69988c10b25268eb8cad6ee6ced7f6a28bee9d'}}
        self.identity={'status':'PASS','loaded_image_status':'PASS','observed_build_id':'EGFX-test',
                       'ae':{'pid':42,'path':str(self.app)}}
        self.smoke={'actual_ae_execution':True,'target_pid':42,'pixels':{'status':'PASS'},'status':'BLOCKED'}

    def run_case(self, identity=None, smoke=None):
        identity=self.identity if identity is None else identity
        smoke=self.smoke if smoke is None else smoke
        with patch.object(ta.platform,'system',return_value='Darwin'),              patch.object(ta.platform,'machine',return_value='arm64'),              patch.object(ta,'load_candidate',return_value=self.manifest),              patch.object(ta,'verify_installed'),              patch.object(ta.live_identity,'diagnose',return_value=identity),              patch.object(ta.ae_smoke_runner,'prepare',side_effect=self.prepare),              patch.object(ta.ae_smoke_runner,'execute',return_value=smoke):
            return ta.run_acceptance(self.report,self.app,self.plugin,self.root/'m.json',self.root/'p.zip')

    def prepare(self, parent, build):
        folder=parent/'run'; folder.mkdir(parents=True)
        for name in ta.FRAMES:
            (folder/(name+'.png')).write_bytes(b'png')
        (folder/'capture.json').write_text('{}')
        return folder, {'status':'NOT RUN'}

    def test_identity_and_pixels_pass_functional_but_not_release(self):
        result, archive=self.run_case()
        self.assertEqual(result['functional_status'],'PASS')
        self.assertEqual(result['release'],'BLOCKED')
        self.assertTrue(archive.is_file())

    def test_identity_blocked_never_runs_smoke(self):
        blocked=dict(self.identity,status='BLOCKED',loaded_image_status='NOT RUN')
        with patch.object(ta.platform,'system',return_value='Darwin'),              patch.object(ta.platform,'machine',return_value='arm64'),              patch.object(ta,'load_candidate',return_value=self.manifest),              patch.object(ta,'verify_installed'),              patch.object(ta.live_identity,'diagnose',return_value=blocked),              patch.object(ta.ae_smoke_runner,'prepare') as prepare:
            result,_=ta.run_acceptance(self.report,self.app,self.plugin,self.root/'m',self.root/'p')
        self.assertEqual(result['functional_status'],'BLOCKED'); prepare.assert_not_called()

    def test_pixel_failure_is_fail_not_pass(self):
        smoke=dict(self.smoke,pixels={'status':'FAIL'})
        result,_=self.run_case(smoke=smoke)
        self.assertEqual(result['functional_status'],'FAIL')

    def test_process_change_blocks(self):
        smoke=dict(self.smoke,target_pid=43)
        result,_=self.run_case(smoke=smoke)
        self.assertEqual(result['functional_status'],'BLOCKED')

    def test_wrong_observed_build_blocks(self):
        identity=dict(self.identity,observed_build_id='EGFX-other')
        result,_=self.run_case(identity=identity)
        self.assertEqual(result['functional_status'],'BLOCKED')

    def test_non_mac_refuses_before_host_work(self):
        with patch.object(ta.platform,'system',return_value='Linux'),              patch.object(ta.live_identity,'diagnose') as diag:
            with self.assertRaises(ValueError):
                ta.run_acceptance(self.report,self.app,self.plugin)
            diag.assert_not_called()

    def test_report_zip_excludes_private_sample(self):
        result, archive=self.run_case()
        import zipfile
        with zipfile.ZipFile(archive) as z:
            names=set(z.namelist())
            self.assertNotIn('sample-private.txt',names)
            self.assertIn('acceptance.json',names)
            self.assertTrue(any(n.startswith('frames/') for n in names))


if __name__=='__main__': unittest.main()
