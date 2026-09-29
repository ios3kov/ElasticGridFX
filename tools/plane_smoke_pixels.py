#!/usr/bin/env python3
"""Pixel assertions for the Stage 9 perspective-plane AE fixture."""
from __future__ import annotations
from pathlib import Path

from smoke_pixels import Image, difference, read_png

DEPTHS = (8, 16, 32)
DEPTH_FRAMES = (
    'original', 'identity', 'legacy-wave', 'plane-wave',
    'skew-wave', 'invalid', 'half-identity', 'half-original',
)
EXTRA_FRAMES = (
    'roundtrip-before', 'roundtrip-after',
    '3d-base', '3d-position', '3d-scale', '3d-layer-rotate',
    '3d-camera-move', '3d-parent', '3d-camera-switch', '3d-no-camera',
)
FRAMES = tuple(f'd{depth}-{name}' for depth in DEPTHS for name in DEPTH_FRAMES) + EXTRA_FRAMES


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
