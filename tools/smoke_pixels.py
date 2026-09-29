#!/usr/bin/env python3
"""Bounded PNG decoding and pixel-level smoke assertions, without extra packages."""
from __future__ import annotations
from array import array
import binascii
from dataclasses import dataclass
from pathlib import Path
import struct
import zlib

SIGNATURE = b'\x89PNG\r\n\x1a\n'
MAX_PIXELS = 2_000_000


@dataclass
class Image:
    width: int
    height: int
    pixels: array  # normalized RGBA; PNG output is not a float/HDR oracle


def chunk(kind: bytes, payload: bytes) -> bytes:
    return struct.pack('>I', len(payload)) + kind + payload + struct.pack('>I', binascii.crc32(kind + payload) & 0xffffffff)


def pattern(path: Path, width: int = 319, height: int = 241) -> None:
    rows = bytearray()
    for y in range(height):
        rows.append(0)
        for x in range(width):
            rows.extend((230 if (x // 17 + y // 23) % 2 else 25,
                         x * 255 // max(1, width-1),
                         220 if (x + 2*y) % 53 < 21 else 35, 255))
    with path.open('xb') as handle:
        handle.write(SIGNATURE + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)) +
                     chunk(b'IDAT', zlib.compress(rows)) + chunk(b'IEND', b''))


def read_png(path: Path) -> Image:
    if path.is_symlink() or not path.is_file() or path.stat().st_size > 32*1024*1024:
        raise ValueError('invalid or oversized PNG')
    data = path.read_bytes()
    if not data.startswith(SIGNATURE):
        raise ValueError('not a PNG')
    offset = 8
    header = None
    compressed = bytearray()
    ended = False
    idat_ended = False
    while offset < len(data):
        if offset + 12 > len(data):
            raise ValueError('truncated PNG chunk')
        size = struct.unpack_from('>I', data, offset)[0]
        kind = data[offset+4:offset+8]
        stop = offset+12+size
        if stop > len(data):
            raise ValueError('truncated PNG payload')
        payload = data[offset+8:offset+8+size]
        crc = struct.unpack_from('>I', data, offset+8+size)[0]
        if binascii.crc32(kind+payload) & 0xffffffff != crc:
            raise ValueError('PNG CRC mismatch')
        if header is None and kind != b'IHDR':
            raise ValueError('IHDR must be first')
        if kind == b'IHDR':
            if header is not None or size != 13:
                raise ValueError('invalid IHDR')
            header = struct.unpack('>IIBBBBB', payload)
        elif kind == b'IDAT':
            if idat_ended:
                raise ValueError('nonconsecutive IDAT')
            compressed.extend(payload)
        elif kind == b'IEND':
            if size or not compressed or stop != len(data):
                raise ValueError('invalid IEND/trailing bytes')
            ended = True
            break
        elif kind == b'tRNS':
            raise ValueError('tRNS transparency not supported by this smoke decoder')
        elif kind == b'PLTE':
            pass  # optional suggested palette for truecolor, not used
        elif not kind[0] & 32:
            raise ValueError('unsupported critical PNG chunk')
        if compressed and kind != b'IDAT':
            idat_ended = True
        offset = stop
    if not ended or header is None:
        raise ValueError('incomplete PNG')
    w, h, depth, color, compression, filtering, interlace = header
    channels = {0: 1, 2: 3, 4: 2, 6: 4}.get(color)
    if not channels or depth not in (8, 16) or compression or filtering or interlace:
        raise ValueError('unsupported PNG format (require non-interlaced 8/16-bit gray/RGB/RGBA)')
    if w < 1 or h < 1 or w*h > MAX_PIXELS:
        raise ValueError('invalid PNG dimensions')
    bpp = channels*(depth//8)
    stride = w*bpp
    expected = h*(stride+1)
    decoder = zlib.decompressobj()
    try:
        raw = decoder.decompress(compressed, expected+1)
    except zlib.error as error:
        raise ValueError('invalid PNG deflate stream') from error
    if len(raw) != expected or not decoder.eof or decoder.unused_data or decoder.unconsumed_tail:
        raise ValueError('invalid or oversized PNG deflate stream')
    pixels = array('f')
    previous = bytearray(stride)
    for y in range(h):
        start = y*(stride+1)
        kind = raw[start]
        if kind > 4:
            raise ValueError('invalid PNG filter')
        row = bytearray(raw[start+1:start+1+stride])
        for i in range(stride):
            a = row[i-bpp] if i >= bpp else 0
            b = previous[i]
            c = previous[i-bpp] if i >= bpp else 0
            if kind == 0: predictor = 0
            elif kind == 1: predictor = a
            elif kind == 2: predictor = b
            elif kind == 3: predictor = (a+b)//2
            else:
                p = a+b-c
                pa, pb, pc = abs(p-a), abs(p-b), abs(p-c)
                predictor = a if pa <= pb and pa <= pc else b if pb <= pc else c
            row[i] = (row[i]+predictor) & 255
        values = list(row) if depth == 8 else struct.unpack('>'+'H'*(w*channels), row)
        scale = 255.0 if depth == 8 else 65535.0
        for x in range(w):
            sample = values[x*channels:(x+1)*channels]
            rgb = sample[:3] if color in (2, 6) else [sample[0]]*3
            alpha = sample[-1] if color in (4, 6) else scale
            pixels.extend([rgb[0]/scale, rgb[1]/scale, rgb[2]/scale, alpha/scale])
        previous = row
    return Image(w, h, pixels)


def difference(a: Image, b: Image) -> dict:
    if (a.width, a.height) != (b.width, b.height):
        raise ValueError('frame dimensions changed')
    maximum = total = 0.0
    changed = 0
    for offset in range(0, len(a.pixels), 4):
        delta = [abs(a.pixels[offset+c] - b.pixels[offset+c]) for c in range(4)]
        d = max(delta)
        maximum = max(maximum, d)
        total += sum(delta)
        changed += d > 2.0/255.0
    return dict(max_abs=maximum, mean_abs=total/len(a.pixels), changed_fraction=changed/(a.width*a.height))


FRAMES = ('bypass', 'identity', 'static_a', 'static_b', 'animated_a', 'animated_b', 'reset',
          'chain_before_corner', 'chain_corner_identity', 'chain_corner_moved')


def validate_frames(folder: Path) -> dict:
    images = {name: read_png(folder / (name+'.png')) for name in FRAMES}
    reference = images['bypass']
    if (reference.width, reference.height) != (319, 241):
        raise ValueError('unexpected smoke dimensions')
    for image in images.values():
        ranges = [max(image.pixels[c::4])-min(image.pixels[c::4]) for c in range(3)]
        if sum(r > 0.2 for r in ranges) < 2 or min(image.pixels[3::4]) < 0.99:
            raise ValueError('blank/flat/transparent frame cannot prove deformation')
    checks = {}
    for a, b, should_change in (('bypass','identity',False), ('identity','static_a',True),
                                ('static_a','static_b',False), ('animated_a','animated_b',True),
                                ('identity','reset',False),
                                ('chain_before_corner','chain_corner_identity',False),
                                ('chain_corner_identity','chain_corner_moved',True)):
        diff = difference(images[a], images[b])
        passed = (diff['changed_fraction'] >= 0.01 and diff['mean_abs'] >= 0.001) if should_change else diff['max_abs'] <= 1.0/255.0 + 1e-7
        checks[a+'_'+b] = dict(status='PASS' if passed else 'FAIL', **diff)
    return dict(status='PASS' if all(v['status']=='PASS' for v in checks.values()) else 'FAIL',
                checks=checks, scope='PNG image smoke incl. Adjustment Layer -> ElasticGrid -> Corner Pin; not HDR accuracy, guide-drag or GPU verification')
