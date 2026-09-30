"""Read-only, bounded filesystem metadata capture on Darwin and Linux."""
from __future__ import annotations
import ctypes
from functools import lru_cache
import hashlib
import os
from pathlib import Path
import platform

LIMIT = 32 * 1024 * 1024


@lru_cache(maxsize=1)
def darwin_api():
    # Public Apple sys/xattr.h signatures; CPython os.*xattr is Linux-only.
    lib = ctypes.CDLL('/usr/lib/libSystem.B.dylib', use_errno=True)
    listing, reading = lib.listxattr, lib.getxattr
    listing.argtypes = [ctypes.c_char_p, ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int]
    listing.restype = ctypes.c_ssize_t
    reading.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_void_p,
                       ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int]
    reading.restype = ctypes.c_ssize_t
    return listing, reading


def bounded_read(call, limit: int = LIMIT) -> bytes:
    size = call(None, 0)
    if size < 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error))
    if size > limit:
        raise ValueError('Extended attribute exceeds capture limit')
    # Use a non-null buffer even for empty attributes to detect concurrent growth.
    buffer = ctypes.create_string_buffer(max(1, size))
    actual = call(buffer, max(1, size))
    if actual < 0:
        error = ctypes.get_errno()
        raise OSError(error, os.strerror(error))
    if actual != size:
        raise ValueError('Extended attributes changed during capture')
    return buffer.raw[:actual]


def attribute_hashes(path: Path) -> dict:
    values = {}
    total = 0
    if platform.system() == 'Darwin':
        listing, reading = darwin_api()
        encoded = os.fsencode(path)
        # XATTR_NOFOLLOW only. Never disable authorization/security checks.
        raw = bounded_read(lambda buf, size: listing(encoded, buf, size, 1), 65536)
        if raw and not raw.endswith(b'\0'):
            raise ValueError('Malformed extended attribute list')
        names = raw[:-1].split(b'\0') if raw else []
        if len(names) > 1024 or len(names) != len(set(names)) or any(not n for n in names):
            raise ValueError('Invalid extended attribute names')
        def read(name):
            return bounded_read(lambda buf, size: reading(encoded, name, buf, size, 0, 1))
    elif platform.system() == 'Linux':
        names = os.listxattr(path, follow_symlinks=False)
        def read(name):
            return os.getxattr(path, name, follow_symlinks=False)
    else:
        raise ValueError('Extended attribute capture is unsupported on this platform')
    for name in sorted(names):
        value = read(name)
        total += len(value)
        if total > LIMIT:
            raise ValueError('Extended attributes exceed total capture limit')
        values[os.fsdecode(name)] = hashlib.sha256(value).hexdigest()
    return values
