#!/usr/bin/env python3
"""Build a native Install/Restore app from a pinned unchanged candidate. No install."""
import argparse
import json
import os
from pathlib import Path
import platform
import subprocess
import zipfile
import build_identity as bi
import generate_installer_payload as payload

ROOT=Path(__file__).resolve().parents[1]
MAC_CORE=('AtomicExchange.cpp','PreparedJournal.cpp','ExchangeCoordinator.cpp',
          'PayloadSnapshot.cpp','SnapshotReceipt.cpp','FreshPublication.cpp','NativeInstaller.mm')


def call(args,out):
    with out.open('w',encoding='utf-8') as log:
        subprocess.run([str(x) for x in args],cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,check=True)


def build(source_payload,manifest,expected_id,out):
    source=bi.source_record(ROOT)
    if source['source_state']!='clean':raise ValueError('Commit native installer sources before packaging')
    if out.exists():raise FileExistsError(out)
    out.mkdir(parents=True)
    record=payload.generate(source_payload,manifest,expected_id,out/'Payload.cpp')
    candidate=record['candidate'];version=candidate['version']
    if version!=source['version']:raise ValueError('Candidate/product version mismatch')
    cpp=out/'Payload.cpp'
    if record['platform']=='macos-arm64':
        if platform.system()!='Darwin' or platform.machine()!='arm64':raise ValueError('Mac arm64 builder required')
        app=out/'FSTR Stretch Installer.app';binary=app/'Contents/MacOS/Installer';binary.parent.mkdir(parents=True)
        args=['clang++','-std=c++20','-fobjc-arc','-arch','arm64','-mmacosx-version-min=11.0','-O2','-Wall','-Wextra','-Werror','-I'+str(ROOT/'installer'),'-I'+str(ROOT/'installer/macos'),ROOT/'installer/macos/InstallerApp.mm',*[ROOT/'installer/macos'/p for p in MAC_CORE],cpp,'-framework','AppKit','-framework','Security','-o',binary]
        call(args,out/'build.log')
        import plistlib
        (app/'Contents/Info.plist').write_bytes(plistlib.dumps({'CFBundleIdentifier':'com.fstr.stretch.installer','CFBundleName':'FSTR Stretch Installer','CFBundleExecutable':'Installer','CFBundlePackageType':'APPL','CFBundleVersion':version,'CFBundleShortVersionString':version,'LSMinimumSystemVersion':'11.0','NSHighResolutionCapable':True}))
        call(['codesign','--force','--sign','-',app],out/'sign.log')
        call(['codesign','--verify','--deep','--strict',app],out/'signature-verification.log')
        archive=out/'FSTR-Stretch-Installer-Mac.zip'
        with zipfile.ZipFile(archive,'x',compression=zipfile.ZIP_DEFLATED) as z:
            for p in sorted(app.rglob('*')):
                if p.is_symlink():raise ValueError('Link in native app')
                if p.is_file():z.write(p,p.relative_to(out))
        delivery=archive
    else:
        if os.name!='nt':raise ValueError('Windows MSVC builder required')
        binary=out/'FSTR Stretch Installer.exe';objects=out/'objects';objects.mkdir()
        xml=(ROOT/'installer/windows/Installer.manifest').read_text().replace('@VERSION@',version)
        native_manifest=out/'Installer.manifest';native_manifest.write_text(xml,encoding='utf-8')
        args=['cl.exe','/nologo','/std:c++20','/EHsc','/utf-8','/MT','/O2','/W4','/WX','/D_WIN32_WINNT=0x0A00','/I'+str(ROOT/'installer'),'/I'+str(ROOT/'installer/windows'),'/I'+str(ROOT/'src'),ROOT/'installer/windows/InstallerApp.cpp',ROOT/'installer/windows/NativeInstaller.cpp',cpp,'/Fo'+str(objects)+os.sep,'/Fe:'+str(binary),'/link','/SUBSYSTEM:WINDOWS','/ENTRY:wWinMainCRTStartup','/MANIFEST:EMBED','/MANIFESTINPUT:'+str(native_manifest),'user32.lib','shell32.lib','ole32.lib','advapi32.lib','bcrypt.lib','uuid.lib']
        call(args,out/'build.log');delivery=binary
    result={'schema':1,'artifact_type':'native Install/Restore validation candidate','installer_source_commit':source['commit'],'installer_source_sha256':source['source_sha256'],'candidate':candidate,'platform':record['platform'],'installer_binary_sha256':bi.digest(binary.read_bytes()),'delivery_file':delivery.name,'delivery_sha256':bi.digest(delivery.read_bytes()),'administrator_installation':'NOT RUN','scope':'native package build only; not host or privileged-install acceptance'}
    (out/'installer-artifact.json').write_text(json.dumps(result,indent=2)+'\n')
    return result


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--payload',required=True,type=Path);p.add_argument('--manifest',required=True,type=Path)
    p.add_argument('--expected-build-id',required=True);p.add_argument('--out',required=True,type=Path)
    a=p.parse_args();print(json.dumps(build(a.payload.resolve(strict=True),a.manifest.resolve(strict=True),a.expected_build_id,a.out.resolve()),indent=2))
