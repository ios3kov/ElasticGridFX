"""Negative/positive diagnostic tests. Native sampler integration is separate."""
import json
import contextlib
import plistlib
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
import uuid
import zipfile

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tools'))
import live_identity as li

IMAGE_UUID = 'ED25AD9A-C563-431B-BC28-1D5A1E8D038E'
OTHER_UUID = '00FC3C76-5664-4A28-932B-9B0B671CC755'
EXE = '/Applications/Adobe After Effects.app/Contents/MacOS/After Effects'
BINARY = Path('/Library/MediaCore/ElasticGrid.plugin/Contents/MacOS/ElasticGrid')


def thin(value=IMAGE_UUID, endian='<'):
    return struct.pack(endian+'8I',0xfeedfacf,0x100000c,0,6,1,24,0,0)+struct.pack(endian+'II',0x1b,24)+uuid.UUID(value).bytes


def report(value=IMAGE_UUID, path=str(BINARY), header=True):
    return (f'Process: After Effects [123]\nPath: {EXE}\nCode Type: ARM-64\n' if header else '') + f'Binary Images:\n  0x1000 - 0x2000 +com.elasticgrid.fx (0.9.0) <{value}> {path}\n'


class LiveIdentity(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='egfx-live-unit-')
        self.addCleanup(self.tmp.cleanup)
        self.folder = Path(self.tmp.name).resolve()

    def test_thin_endianness(self):
        for endian in ('<','>'):
            self.assertEqual(li.macho_uuids(thin(endian=endian)), {IMAGE_UUID})

    def test_fat32_and_fat64(self):
        for endian in ('>','<'):
            for wide in (False, True):
                fmt=endian+('IIQQII' if wide else 'IIIII')
                table=bytearray(struct.pack(endian+'II',0xcafebabf if wide else 0xcafebabe,2))
                size=56
                start=8+2*struct.calcsize(fmt)
                for i in range(2):
                    values=(0x100000c+i,0,start+size*i,size,0)
                    table.extend(struct.pack(fmt,*(values+(0,) if wide else values)))
                data=bytes(table)+thin()+thin(OTHER_UUID)
                self.assertEqual(li.macho_uuids(data),{IMAGE_UUID,OTHER_UUID})

    def test_malformed_macho_refused(self):
        original=thin()
        invalid=[b'',b'ELF file',original[:31],original[:-1],thin('00000000-0000-0000-0000-000000000000'),
                 original[:16]+struct.pack('<II',5000,24)+original[24:],
                 original[:36]+struct.pack('<I',0)+original[40:],
                 struct.pack('>II',0xcafebabe,1)+bytes(20)]
        for data in invalid:
            with self.subTest(size=len(data)):
                with self.assertRaises(li.Blocked): li.macho_uuids(data)

    def test_duplicate_uuid_load_command_refused(self):
        data=bytearray(thin()); struct.pack_into('<II',data,16,2,48)
        data+=data[32:56]
        with self.assertRaises(li.Blocked): li.macho_uuids(bytes(data))

    def test_sample_and_uuid_mapping(self):
        for value in (IMAGE_UUID,IMAGE_UUID.replace('-','').lower()):
            images=li.parse_sample(report(value),123,EXE)
            self.assertEqual(li.select_image(images,BINARY,{IMAGE_UUID})['uuid'],IMAGE_UUID)

    def test_wrong_header_or_stack_token_refused(self):
        for text in (report(header=False), report().replace('[123]','[124]'),
                     report().replace('Path: '+EXE,'Path: /other'),
                     report().replace('Binary Images:','Stack only:'),
                     report()+'\nBinary Images:\n'):
            with self.assertRaises(li.Blocked): li.parse_sample(text,123,EXE)

    def test_missing_malformed_or_ambiguous_live_image_refused(self):
        images=li.parse_sample(report(),123,EXE)
        for candidate in ([],images+images, [dict(images[0], uuid=OTHER_UUID)], [dict(images[0],path=str(BINARY)+'-old')]):
            with self.assertRaises(li.Blocked): li.select_image(candidate,BINARY,{IMAGE_UUID})
        with self.assertRaises(li.Blocked): li.parse_sample(report('not-a-uuid'),123,EXE)

    def test_no_ae_and_linux_never_call_sampler(self):
        manifest={'build':{'build_id':'test'}}
        with patch.object(li.platform,'system',return_value='Linux'), patch.object(li,'capture') as capture:
            result=li.diagnose(self.folder,manifest,[])
            self.assertEqual(result['status'],'BLOCKED'); capture.assert_not_called()
        with patch.object(li.platform,'system',return_value='Darwin'),patch.object(li,'running_ae',return_value=[]),patch.object(li,'capture') as capture:
            result=li.diagnose(self.folder,manifest,[])
            self.assertEqual(result['status'],'BLOCKED'); capture.assert_not_called()

    def test_ps_selects_only_actual_ae_executables(self):
        text=f' 123 {EXE}\n 456 /usr/bin/aerender\n 789 /tmp/After Effects helper\n 101 {EXE} helper\n'
        with patch.object(li.subprocess,'check_output',return_value=text):
            hosts=li.running_ae()
        self.assertEqual([p['pid'] for p in hosts],[123])

    def test_sampler_refusal_no_result(self):
        with patch.object(li,'process_key',return_value='Mon Sep 28 12:00:00 2026 '+EXE),patch.object(li.subprocess,'run',return_value=subprocess.CompletedProcess([],1,b'',b'denied')):
            with self.assertRaises(li.Blocked): li.capture(123,Path(EXE),self.folder)

    def test_stale_result_refused_without_sampler(self):
        (self.folder/'sample-private.txt').write_text(report())
        with patch.object(li,'process_key',return_value='Mon Sep 28 12:00:00 2026 '+EXE),patch.object(li.subprocess,'run') as run:
            with self.assertRaises(li.Blocked): li.capture(123,Path(EXE),self.folder)
            run.assert_not_called()

    def test_process_replacement_refused(self):
        with patch.object(li,'process_key',side_effect=['Mon Sep 28 12:00:00 2026 '+EXE,'Mon Sep 28 12:00:01 2026 '+EXE]),patch.object(li.subprocess,'run',return_value=subprocess.CompletedProcess([],0,b'',b'')):
            with self.assertRaises(li.Blocked): li.capture(123,Path(EXE),self.folder)

    def test_mocked_capture_retains_only_private_file_hash(self):
        def sample(*args,**kwargs):
            (self.folder/'sample-private.txt').write_text(report())
            return subprocess.CompletedProcess([],0,b'',b'')
        with patch.object(li,'process_key',return_value='Mon Sep 28 12:00:00 2026 '+EXE),patch.object(li.subprocess,'run',side_effect=sample):
            images,observation=li.capture(123,Path(EXE),self.folder)
        self.assertEqual(images[0]['uuid'],IMAGE_UUID)
        self.assertIn('private_sample_sha256',observation)
        self.assertNotIn('sample-private.txt',json.dumps(observation))

    def test_report_excludes_stacks_and_masks_home(self):
        (self.folder/'sample-private.txt').write_text('private project path, never share')
        result={'status':'BLOCKED','loaded_image_status':'NOT RUN','path':str(Path.home()/'Library/my/plugin')}
        output=li.write_report(self.folder,result)
        with zipfile.ZipFile(output) as z:
            self.assertEqual(set(z.namelist()),{'report.json','report.txt'})
            self.assertNotIn(str(Path.home()).encode(),z.read('report.json'))
            self.assertNotIn(b'private project path',z.read('report.txt'))
        with self.assertRaises(FileExistsError): li.write_report(self.folder,result)

    def test_baseline_manifest_identity_is_valid(self):
        manifest=json.loads(li.BASELINE.read_text())
        meta=li.bi.validate_identity(manifest['build'])
        self.assertEqual(meta['commit'],'6d3b846463410f37198fda4b625e56e4cea44c22')
        self.assertEqual(meta['build_id'],'EGFX-603e9d3e4025d271e0488201')
        self.assertEqual(len(manifest['files']),6)



class DiagnosticPipeline(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(prefix='egfx-pipeline-')
        self.addCleanup(self.tmp.cleanup)
        self.root=Path(self.tmp.name).resolve()
        self.app=self.root/'After Effects.app'
        (self.app/'Contents/MacOS').mkdir(parents=True)
        (self.app/'Contents/Info.plist').write_bytes(plistlib.dumps({'CFBundleIdentifier':'com.adobe.AfterEffects','CFBundleShortVersionString':'fixture'}))
        self.binary=self.root/'MediaCore/ElasticGrid.plugin/Contents/MacOS/ElasticGrid'
        self.binary.parent.mkdir(parents=True)
        self.binary.write_bytes(thin())
        self.bundle=self.binary.parent.parent.parent
        self.meta={'build_id':'EGFX-fixture', 'commit':'fixture'}
        self.manifest={'build':self.meta}
        self.key='Mon Sep 28 12:00:00 2026 '+str(self.app/'Contents/MacOS/After Effects')

    def diagnose(self, *, wrong_uuid=False, conflicts=False, changed_pid=False, bad_disk=False):
        images=[{'path':str(self.binary),'uuid':OTHER_UUID if wrong_uuid else IMAGE_UUID,'label':'com.elasticgrid.fx'}]
        observation={'process_key_sha256':li.bi.digest(self.key.encode())}
        patches=[patch.object(li.platform,'system',return_value='Darwin'),
                 patch.object(li,'running_ae',return_value=[dict(pid=123,app=str(self.app),executable=str(self.app/'Contents/MacOS/After Effects'))]),
                 patch.object(li,'installed_roots',return_value=[self.root/'MediaCore']),
                 patch.object(li,'discover',return_value=[self.bundle,self.root/'old.plugin'] if conflicts else [self.bundle]),
                 patch.object(li,'disk_summary',side_effect=lambda p:{'path':str(p)}),
                 patch.object(li,'verify_disk',side_effect=li.Blocked('old candidate') if bad_disk else None,return_value=self.meta),
                 patch.object(li,'signature'),
                 patch.object(li,'capture',return_value=(images,observation)),
                 patch.object(li,'process_key',return_value='changed' if changed_pid else self.key)]
        with contextlib.ExitStack() as stack:
            for mock in patches: stack.enter_context(mock)
            return li.diagnose(self.root,self.manifest,[])

    def test_identity_pass_is_not_functional_or_release_pass(self):
        result=self.diagnose()
        self.assertEqual(result['loaded_image_status'],'PASS')
        self.assertEqual(result['observed_build_id'],self.meta['build_id'])
        self.assertIsNone(result['direct_runtime_build_id'])
        self.assertEqual(result['functional_test'],'NOT RUN')
        self.assertEqual(result['release'],'BLOCKED')

    def test_live_uuid_mismatch_blocks_confirmation(self):
        self.assertEqual(self.diagnose(wrong_uuid=True)['status'],'BLOCKED')

    def test_scan_conflicts_block_confirmation(self):
        self.assertEqual(self.diagnose(conflicts=True)['status'],'BLOCKED')

    def test_pid_change_blocks_confirmation(self):
        self.assertEqual(self.diagnose(changed_pid=True)['status'],'BLOCKED')

    def test_old_disk_copy_never_inherits_expected_id(self):
        result=self.diagnose(bad_disk=True)
        self.assertEqual(result['status'],'BLOCKED')
        self.assertIsNone(result['observed_build_id'])
        self.assertEqual(len(result['loaded_images']),1)

if __name__=='__main__': unittest.main()
