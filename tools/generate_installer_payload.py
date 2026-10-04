#!/usr/bin/env python3
"""Generate embedded native payload from an explicitly pinned verified artifact.

Build tooling only. No installer destination, network, privileged execution or
user-machine discovery. The resulting installer has no external payload input.
"""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import zipfile
import build_identity as bi


def collect(payload, manifest, expected_build_id):
    m = json.loads(manifest.read_text(encoding='utf-8-sig'))
    b = bi.validate_identity(m['build'])
    if b['build_id'] != expected_build_id or b['profile'] != 'release':
        raise ValueError('Wrong explicitly pinned ordinary release payload')
    files = []
    if b['target'] == 'x86_64-pc-windows-msvc':
        data = payload.read_bytes()
        if m.get('artifact') != 'FSTR Stretch.aex' or bi.digest(data) != m['sha256']:
            raise ValueError('Wrong Windows payload')
        files = [('FSTR Stretch.aex', data, False)]
        platform = 'windows-x64'
    elif b['target'] == 'aarch64-apple-darwin':
        if m.get('bundle_name') != 'FSTR Stretch.plugin' or bi.digest(payload.read_bytes()) != m['package_sha256']:
            raise ValueError('Wrong Mac archive')
        with zipfile.ZipFile(payload) as z:
            if z.testzip() is not None:
                raise ValueError('Corrupt archive')
            members = [i for i in z.infolist() if not i.is_dir()]
            expected = {'FSTR Stretch.plugin/' + n for n in m['files']}
            if len(members) != len(expected) or {i.filename for i in members} != expected:
                raise ValueError('Unexpected or duplicate archive member')
            for n, item in sorted(m['files'].items()):
                info = z.getinfo('FSTR Stretch.plugin/' + n)
                if (info.external_attr >> 16) & 0o170000 == 0o120000:
                    raise ValueError('Link in archive')
                data = z.read(info)
                if bi.digest(data) != item['sha256'] or bool((info.external_attr >> 16) & 0o111) != item['executable']:
                    raise ValueError('Changed payload bytes or executable bit')
                files.append((n, data, item['executable']))
        platform = 'macos-arm64'
    else:
        raise ValueError('Unsupported architecture')
    if not files or len(files) > 4096 or sum(len(d) for _, d, _ in files) > 32 * 1024 * 1024:
        raise ValueError('Payload size limit')
    for n, data, _ in files:
        p = PurePosixPath(n)
        if p.is_absolute() or p.as_posix() != n or any(x in ('..', '.') for x in p.parts) or '\\' in n or not re.fullmatch(r'[A-Za-z0-9 _./-]+', n):
            raise ValueError('Unsafe payload path')
    if ('ElasticGridBuildID=' + b['build_id']).encode() not in b''.join(d for _, d, _ in files):
        raise ValueError('Missing binary Build ID')
    return b, platform, files


def generate(payload, manifest, expected_build_id, output):
    b, platform, files = collect(payload, manifest, expected_build_id)
    rows = ['#include "Payload.h"', 'namespace fstr::payload {']
    for i, (_, data, _) in enumerate(files):
        rows.append('static const unsigned char data' + str(i) + '[] = {')
        rows.extend(','.join(str(v) for v in data[j:j + 40]) + ',' for j in range(0, len(data), 40))
        if not data:
            rows.append('0,')
        rows.append('};')
    for name, value in [('version', b['version']), ('buildId', b['build_id']), ('commit', b['commit']), ('platform', platform)]:
        rows.append('const char ' + name + '[] = ' + json.dumps(value) + ';')
    rows.append('const File files[] = {')
    for i, (name, data, execute) in enumerate(files):
        rows.append('{' + json.dumps(name) + ',data' + str(i) + ',' + str(len(data)) + ',' + json.dumps(hashlib.sha256(data).hexdigest()) + ',' + str(execute).lower() + '},')
    rows.extend(['};', 'const std::size_t count=sizeof(files)/sizeof(files[0]);', '}'])
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open('x', encoding='ascii') as f:
        f.write('\n'.join(rows) + '\n')
    return {'candidate': b, 'platform': platform, 'payload_cpp_sha256': bi.digest(output.read_bytes())}


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--payload', type=Path, required=True)
    p.add_argument('--manifest', type=Path, required=True)
    p.add_argument('--expected-build-id', required=True)
    p.add_argument('--out', type=Path, required=True)
    a = p.parse_args()
    print(json.dumps(generate(a.payload, a.manifest, a.expected_build_id, a.out), indent=2))
