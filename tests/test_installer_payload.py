import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest
import zipfile
sys.path.insert(0,str(Path(__file__).resolve().parents[1]/'tools'))
import build_identity as bi
import generate_installer_payload as payload


class EmbeddedPayloadTests(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory();self.addCleanup(self.temp.cleanup)
        self.root=Path(self.temp.name)
        record={'version':'0.9.4','commit':'a'*40,'source_state':'clean','source_sha256':'b'*64}
        self.build=bi.identity(record,'x86_64-pc-windows-msvc','release',{'rustc':'fixture'}, {})
        self.data=b'MZ ElasticGridBuildID='+self.build['build_id'].encode()
        self.exe=self.root/'aex';self.exe.write_bytes(self.data)
        self.manifest=self.root/'manifest.json';self.write_manifest()

    def write_manifest(self,**changes):
        self.manifest.write_text(json.dumps({'build':self.build,'artifact':'FSTR Stretch.aex','sha256':bi.digest(self.data),**changes}))

    def test_valid_pin_generates_external_payload(self):
        out=self.root/'Payload.cpp';r=payload.generate(self.exe,self.manifest,self.build['build_id'],out)
        self.assertEqual(r['platform'],'windows-x64');self.assertIn(self.build['build_id'],out.read_text())
        with self.assertRaises(FileExistsError):payload.generate(self.exe,self.manifest,self.build['build_id'],out)

    def test_tampered_file_and_wrong_pin_refused(self):
        with self.assertRaises(ValueError):payload.collect(self.exe,self.manifest,'EGFX-'+'0'*24)
        self.exe.write_bytes(self.data+b'tamper')
        with self.assertRaises(ValueError):payload.collect(self.exe,self.manifest,self.build['build_id'])

    def test_dirty_or_wrong_architecture_refused(self):
        for key,value in [('source_state','dirty'),('target','x86_64-unknown-linux-gnu')]:
            b={**self.build,key:value};b.pop('build_id');b['build_id']='EGFX-'+bi.digest(bi.encoded(b))[:24]
            self.write_manifest(build=b)
            with self.assertRaises(ValueError):payload.collect(self.exe,self.manifest,b['build_id'])

    def test_unsafe_mac_paths_refused(self):
        record={'version':'0.9.4','commit':'a'*40,'source_state':'clean','source_sha256':'b'*64}
        b=bi.identity(record,'aarch64-apple-darwin','release',{}, {})
        for name in ['../outside','Contents/MacOS/ElasticGrid']:
            data=b'ElasticGridBuildID='+b['build_id'].encode();archive=self.root/(str(len(name))+'.zip')
            with zipfile.ZipFile(archive,'w') as z:
                info=zipfile.ZipInfo('FSTR Stretch.plugin/'+name);info.external_attr=0o100644<<16;z.writestr(info,data)
            self.manifest.write_text(json.dumps({'build':b,'bundle_name':'FSTR Stretch.plugin','package_sha256':bi.digest(archive.read_bytes()),'files':{name:{'sha256':bi.digest(data),'executable':False}}}))
            if '..' in name:
                with self.assertRaises(ValueError):payload.collect(archive,self.manifest,b['build_id'])
            else:
                self.assertEqual(payload.collect(archive,self.manifest,b['build_id'])[1],'macos-arm64')

    def test_duplicate_link_extra_and_permission_mac_members_refused(self):
        b=bi.identity({'version':'0.9.4','commit':'a'*40,'source_state':'clean','source_sha256':'b'*64},'aarch64-apple-darwin','release',{}, {})
        name='Contents/MacOS/ElasticGrid';data=b'ElasticGridBuildID='+b['build_id'].encode()
        for fault in ['duplicate','link','extra','executable','tamper']:
            with self.subTest(fault=fault):
                archive=self.root/(fault+'.zip')
                with zipfile.ZipFile(archive,'w') as z:
                    info=zipfile.ZipInfo('FSTR Stretch.plugin/'+name)
                    info.external_attr=(0o120644 if fault=='link' else 0o100755 if fault=='executable' else 0o100644)<<16
                    z.writestr(info,data+b'changed' if fault=='tamper' else data)
                    if fault=='duplicate':
                        import warnings
                        with warnings.catch_warnings():
                            warnings.simplefilter('ignore',UserWarning);z.writestr(info,data)
                    if fault=='extra':z.writestr('FSTR Stretch.plugin/extra',b'extra')
                self.manifest.write_text(json.dumps({'build':b,'bundle_name':'FSTR Stretch.plugin','package_sha256':bi.digest(archive.read_bytes()),'files':{name:{'sha256':bi.digest(data),'executable':False}}}))
                with self.assertRaises(ValueError):payload.collect(archive,self.manifest,b['build_id'])


if __name__=='__main__':unittest.main()
