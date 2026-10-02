#!/usr/bin/env python3
"""Verify that the embedded Windows PiPL resource is byte-exact."""
from __future__ import annotations

import argparse
import ctypes
from ctypes import wintypes
import os
from pathlib import Path


LOAD_LIBRARY_AS_DATAFILE = 0x00000002
PIPL_RESOURCE_ID = 16000


def embedded_pipl(aex: Path) -> bytes:
    if os.name != "nt":
        raise RuntimeError("Windows resource verification must run on Windows")

    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel32.LoadLibraryExW.argtypes = [wintypes.LPCWSTR, wintypes.HANDLE, wintypes.DWORD]
    kernel32.LoadLibraryExW.restype = wintypes.HMODULE
    kernel32.FindResourceW.argtypes = [wintypes.HMODULE, ctypes.c_void_p, ctypes.c_void_p]
    kernel32.FindResourceW.restype = wintypes.HRSRC
    kernel32.SizeofResource.argtypes = [wintypes.HMODULE, wintypes.HRSRC]
    kernel32.SizeofResource.restype = wintypes.DWORD
    kernel32.LoadResource.argtypes = [wintypes.HMODULE, wintypes.HRSRC]
    kernel32.LoadResource.restype = wintypes.HGLOBAL
    kernel32.LockResource.argtypes = [wintypes.HGLOBAL]
    kernel32.LockResource.restype = ctypes.c_void_p
    kernel32.FreeLibrary.argtypes = [wintypes.HMODULE]

    module = kernel32.LoadLibraryExW(str(aex.resolve()), None, LOAD_LIBRARY_AS_DATAFILE)
    if not module:
        raise OSError(ctypes.get_last_error(), "LoadLibraryExW failed")

    type_name = ctypes.c_wchar_p("PiPL")
    try:
        resource = kernel32.FindResourceW(
            module,
            ctypes.c_void_p(PIPL_RESOURCE_ID),
            ctypes.cast(type_name, ctypes.c_void_p),
        )
        if not resource:
            raise OSError(ctypes.get_last_error(), "PiPL resource not found")
        size = kernel32.SizeofResource(module, resource)
        handle = kernel32.LoadResource(module, resource)
        pointer = kernel32.LockResource(handle)
        if not size or not handle or not pointer:
            raise OSError(ctypes.get_last_error(), "PiPL resource could not be loaded")
        return ctypes.string_at(pointer, size)
    finally:
        kernel32.FreeLibrary(module)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--aex", required=True, type=Path)
    parser.add_argument("--expected", required=True, type=Path)
    args = parser.parse_args()

    actual = embedded_pipl(args.aex)
    expected = args.expected.read_bytes()
    if actual != expected:
        raise SystemExit(
            f"PiPL mismatch: embedded={len(actual)} bytes expected={len(expected)} bytes"
        )
    print(f"PiPL byte-exact: {len(actual)} bytes")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
