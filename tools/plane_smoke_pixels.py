#!/usr/bin/env python3
"""Pixel assertions for the Stage 9 perspective-plane AE fixture."""
from __future__ import annotations
from pathlib import Path

from smoke_pixels import Image, difference, read_png

DEPTHS = (8, 16, 32)
DEPTH_FRAMES = (
    'original', 'identity', 'skew-identity', 'legacy-wave', 'plane-wave',
    'skew-wave', 'invalid', 'half-identity', 'half-original',
)
EXTRA_FRAMES = (
    'roundtrip-before', 'roundtrip-after',
    '3d-no-camera', '3d-base', '3d-position', '3d-scale', '3d-layer-rotate',
    '3d-camera-move', '3d-parent', '3d-camera-switch',
)
FRAMES = tuple(f'd{depth}-{name}' for depth in DEPTHS for name in DEPTH_FRAMES) + EXTRA_FRAMES


def perspective_coordinates():
    """Independent eight-equation inverse homography; no plugin/core calls."""
    quad = ((10, 5), (120, 0), (127, 85), (0, 95))
    source = ((0, 0), (127, 0), (127, 95), (0, 95))
    matrix = []
    for (x, y), (u, v) in zip(quad, source):
        matrix.extend(([x,y,1,0,0,0,-u*x,-u*y,u],
                       [0,0,0,x,y,1,-v*x,-v*y,v]))
    for c in range(8):
        pivot = max(range(c,8), key=lambda r: abs(matrix[r][c]))
        matrix[c], matrix[pivot] = matrix[pivot], matrix[c]
        scale = matrix[c][c]
        if abs(scale) < 1e-12:
            raise ValueError('singular oracle fixture')
        matrix[c] = [v/scale for v in matrix[c]]
        for r in range(8):
            if r != c:
                scale = matrix[r][c]
                matrix[r] = [a-scale*b for a,b in zip(matrix[r],matrix[c])]
    h = [row[8] for row in matrix]
    for y in range(96):
        for x in range(128):
            denominator = h[6]*x+h[7]*y+1
            yield x,y,(h[0]*x+h[1]*y+h[2])/denominator,(h[3]*x+h[4]*y+h[5])/denominator


def check_zero_wave_projection(original: Image, projected: Image) -> dict:
    if (original.width,original.height,projected.width,projected.height)!=(128,96,128,96):
        raise ValueError('unexpected perspective fixture dimensions')
    errors, exterior = [], []
    # Green is the fixture's smooth horizontal coordinate ramp. Compare against
    # rendered original, avoiding an assumption about display gamma. A mask over
    # stationary content cannot satisfy these inverse-projective coordinates.
    for x,y,u,v in perspective_coordinates():
        k=(y*128+x)*4
        if 3 <= u <= 123 and 3 <= v <= 91:
            a=int(u); b=int(v); t=u-a
            expected=(1-t)*original.pixels[(b*128+a)*4+1]+t*original.pixels[(b*128+a+1)*4+1]
            errors.append(abs(projected.pixels[k+1]-expected))
            errors.append(abs(projected.pixels[k+3]-1))
        elif u < -2 or u > 129 or v < -2 or v > 97:
            exterior.append(abs(projected.pixels[k+3]))
    maximum=max(errors,default=1)
    outside=max(exterior,default=1)
    passed=len(errors)>1000 and len(exterior)>100 and maximum<=0.02 and outside<=1/255
    return dict(status='PASS' if passed else 'FAIL', expected='projective coordinate ramp and transparent exterior',
                max_coordinate_alpha_error=maximum, max_exterior_alpha=outside,
                interior_samples=len(errors)//2, exterior_samples=len(exterior))


def _nonflat(name: str, image: Image) -> None:
    if image.width < 1 or image.height < 1:
        raise ValueError('empty frame: ' + name)
    ranges = [max(image.pixels[c::4]) - min(image.pixels[c::4]) for c in range(3)]
    if sum(r > 0.08 for r in ranges) < 2:
        raise ValueError('blank/flat frame cannot prove plane behavior: ' + name)
    if max(image.pixels[3::4]) < 0.05:
        raise ValueError('fully transparent frame cannot prove plane behavior: ' + name)


def _check(images: dict[str, Image], a: str, b: str, should_change: bool) -> dict:
    diff = difference(images[a], images[b])
    if should_change:
        passed = diff['changed_fraction'] >= 0.005 and diff['mean_abs'] >= 0.0005
    else:
        passed = diff['max_abs'] <= 1.0 / 255.0 + 1.0e-7
    return dict(status='PASS' if passed else 'FAIL', expected='change' if should_change else 'equal', **diff)


def validate_plane_frames(folder: Path) -> dict:
    images = {name: read_png(folder / (name + '.png')) for name in FRAMES}
    for name, image in images.items():
        _nonflat(name, image)

    checks = {}
    for depth in DEPTHS:
        prefix = f'd{depth}-'
        checks[prefix+'zero_wave_projection'] = check_zero_wave_projection(images[prefix+'original'], images[prefix+'skew-identity'])
        pairs = (
            ('original', 'identity', False),
            ('identity', 'plane-wave', True),
            ('legacy-wave', 'plane-wave', False),
            ('plane-wave', 'skew-wave', True),
            ('original', 'invalid', False),
            ('half-identity', 'half-original', False),
        )
        for a, b, changed in pairs:
            key = f'{prefix}{a}_{b}'
            checks[key] = _check(images, prefix + a, prefix + b, changed)

    checks['roundtrip'] = _check(images, 'roundtrip-before', 'roundtrip-after', False)
    checks['3d_position'] = _check(images, '3d-base', '3d-position', True)
    checks['3d_scale'] = _check(images, '3d-position', '3d-scale', True)
    checks['3d_rotation'] = _check(images, '3d-scale', '3d-layer-rotate', True)
    checks['3d_camera_motion'] = _check(images, '3d-layer-rotate', '3d-camera-move', True)
    checks['3d_parent_motion'] = _check(images, '3d-camera-move', '3d-parent', True)
    checks['3d_camera_switch'] = _check(images, '3d-parent', '3d-camera-switch', True)
    checks['3d_no_camera'] = _check(images, '3d-camera-switch', '3d-no-camera', True)

    status = 'PASS' if all(item['status'] == 'PASS' for item in checks.values()) else 'FAIL'
    return dict(
        status=status,
        checks=checks,
        frames=len(images),
        scope=('AE PNG plane pixels: 8/16/32 bpc identity/wave/skew/invalid/half-res, '
               'AEP save-reopen, 3D position/scale/rotation/parenting, active-camera motion/switch and no-camera fallback; '
               'not native guide drag/Undo/Redo or GPU parity'),
    )
