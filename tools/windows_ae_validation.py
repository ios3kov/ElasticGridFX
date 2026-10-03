#!/usr/bin/env python3
"""Windows After Effects validation runner for an already-installed exact .aex.

This tool never installs, replaces, moves, or deletes plug-ins. It requires the
caller to provide the exact candidate .aex, its manifest, the already-installed
copy, and the exact AfterFX.exe to test.
"""
from __future__ import annotations

import argparse
import ctypes
from ctypes import wintypes
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import time
import uuid

import build_identity as bi
from smoke_pixels import FRAMES, pattern, validate_frames

ROOT = Path(__file__).resolve().parents[1]
MATCH_NAME = b"com.elasticgrid.fx.warp"
BUILD_MARKER = re.compile(rb"ElasticGridBuildID=(EGFX-[0-9a-f]{24})")


def sha256_file(path: Path) -> str:
    return bi.digest(path.read_bytes())


def load_manifest(path: Path) -> dict:
    record = json.loads(path.read_text(encoding="utf-8-sig"))
    if record.get("schema") != 1 or record.get("artifact") != "FSTR Stretch.aex":
        raise ValueError("invalid Windows artifact manifest")
    build = bi.validate_identity(record["build"])
    if build.get("artifact_type") != "AE native effect (.aex)" or "windows-msvc" not in build.get("target", ""):
        raise ValueError("manifest does not describe a Windows .aex build")
    if not re.fullmatch(r"[0-9a-f]{64}", record.get("sha256", "")):
        raise ValueError("invalid artifact SHA-256")
    return record


def verify_candidate(candidate: Path, installed: Path, manifest: dict) -> dict:
    candidate = candidate.resolve(strict=True)
    installed = installed.resolve(strict=True)
    expected = manifest["sha256"]
    if sha256_file(candidate) != expected:
        raise ValueError("candidate .aex does not match manifest")
    if sha256_file(installed) != expected:
        raise ValueError("installed .aex does not match candidate")
    marker = BUILD_MARKER.findall(installed.read_bytes())
    if marker != [manifest["build"]["build_id"].encode("ascii")]:
        raise ValueError("installed .aex Build ID does not match manifest")
    return dict(candidate=str(candidate), installed=str(installed), sha256=expected,
                build_id=manifest["build"]["build_id"])


def _windows_path(path: Path | str) -> str:
    return os.path.normcase(os.path.abspath(str(path)))


def _require_windows() -> None:
    if os.name != "nt":
        raise RuntimeError("Windows AE validation must run on Windows")


def _snapshot(flags: int, pid: int = 0):
    kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
    kernel32.CreateToolhelp32Snapshot.argtypes = [wintypes.DWORD, wintypes.DWORD]
    kernel32.CreateToolhelp32Snapshot.restype = wintypes.HANDLE
    handle = kernel32.CreateToolhelp32Snapshot(flags, pid)
    if handle == wintypes.HANDLE(-1).value:
        raise OSError(ctypes.get_last_error(), "CreateToolhelp32Snapshot failed")
    return kernel32, handle


def running_selected_ae(afterfx: Path) -> int:
    _require_windows()
    TH32CS_SNAPPROCESS = 0x00000002
    PROCESS_QUERY_LIMITED_INFORMATION = 0x1000
    kernel32, snap = _snapshot(TH32CS_SNAPPROCESS)

    class PROCESSENTRY32W(ctypes.Structure):
        _fields_ = [
            ("dwSize", wintypes.DWORD),
            ("cntUsage", wintypes.DWORD),
            ("th32ProcessID", wintypes.DWORD),
            ("th32DefaultHeapID", ctypes.c_size_t),
            ("th32ModuleID", wintypes.DWORD),
            ("cntThreads", wintypes.DWORD),
            ("th32ParentProcessID", wintypes.DWORD),
            ("pcPriClassBase", wintypes.LONG),
            ("dwFlags", wintypes.DWORD),
            ("szExeFile", wintypes.WCHAR * 260),
        ]

    kernel32.Process32FirstW.argtypes = [wintypes.HANDLE, ctypes.POINTER(PROCESSENTRY32W)]
    kernel32.Process32NextW.argtypes = [wintypes.HANDLE, ctypes.POINTER(PROCESSENTRY32W)]
    kernel32.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
    kernel32.OpenProcess.restype = wintypes.HANDLE
    kernel32.QueryFullProcessImageNameW.argtypes = [
        wintypes.HANDLE, wintypes.DWORD, wintypes.LPWSTR, ctypes.POINTER(wintypes.DWORD)
    ]
    kernel32.CloseHandle.argtypes = [wintypes.HANDLE]

    wanted = _windows_path(afterfx.resolve(strict=True))
    matches = []
    entry = PROCESSENTRY32W()
    entry.dwSize = ctypes.sizeof(entry)
    try:
        ok = kernel32.Process32FirstW(snap, ctypes.byref(entry))
        while ok:
            process = kernel32.OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, False, entry.th32ProcessID)
            if process:
                try:
                    buffer = ctypes.create_unicode_buffer(32768)
                    size = wintypes.DWORD(len(buffer))
                    if kernel32.QueryFullProcessImageNameW(process, 0, buffer, ctypes.byref(size)):
                        if _windows_path(buffer.value) == wanted:
                            matches.append(int(entry.th32ProcessID))
                finally:
                    kernel32.CloseHandle(process)
            ok = kernel32.Process32NextW(snap, ctypes.byref(entry))
    finally:
        kernel32.CloseHandle(snap)
    if len(matches) != 1:
        raise ValueError(f"require exactly one running selected After Effects process, found {len(matches)}")
    return matches[0]


def module_paths(pid: int) -> list[Path]:
    _require_windows()
    TH32CS_SNAPMODULE = 0x00000008
    TH32CS_SNAPMODULE32 = 0x00000010
    kernel32, snap = _snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)

    class MODULEENTRY32W(ctypes.Structure):
        _fields_ = [
            ("dwSize", wintypes.DWORD),
            ("th32ModuleID", wintypes.DWORD),
            ("th32ProcessID", wintypes.DWORD),
            ("GlblcntUsage", wintypes.DWORD),
            ("ProccntUsage", wintypes.DWORD),
            ("modBaseAddr", ctypes.POINTER(ctypes.c_byte)),
            ("modBaseSize", wintypes.DWORD),
            ("hModule", wintypes.HMODULE),
            ("szModule", wintypes.WCHAR * 256),
            ("szExePath", wintypes.WCHAR * 260),
        ]

    kernel32.Module32FirstW.argtypes = [wintypes.HANDLE, ctypes.POINTER(MODULEENTRY32W)]
    kernel32.Module32NextW.argtypes = [wintypes.HANDLE, ctypes.POINTER(MODULEENTRY32W)]
    kernel32.CloseHandle.argtypes = [wintypes.HANDLE]
    entry = MODULEENTRY32W()
    entry.dwSize = ctypes.sizeof(entry)
    paths = []
    try:
        ok = kernel32.Module32FirstW(snap, ctypes.byref(entry))
        while ok:
            if entry.szExePath:
                paths.append(Path(entry.szExePath))
            ok = kernel32.Module32NextW(snap, ctypes.byref(entry))
    finally:
        kernel32.CloseHandle(snap)
    return paths


def verify_loaded_effect(pid: int, installed: Path, manifest: dict) -> dict:
    installed = installed.resolve(strict=True)
    matches = []
    for path in module_paths(pid):
        if path.suffix.lower() != ".aex":
            continue
        try:
            data = path.read_bytes()
        except OSError:
            continue
        if MATCH_NAME in data:
            matches.append(path.resolve())
    if len(matches) != 1:
        raise ValueError(f"expected one loaded ElasticGridFX module, found {len(matches)}")
    loaded = matches[0]
    if _windows_path(loaded) != _windows_path(installed):
        raise ValueError("loaded ElasticGridFX module is not the selected installed candidate")
    if sha256_file(loaded) != manifest["sha256"]:
        raise ValueError("loaded module hash does not match manifest")
    markers = BUILD_MARKER.findall(loaded.read_bytes())
    build_id = manifest["build"]["build_id"]
    if markers != [build_id.encode("ascii")]:
        raise ValueError("loaded module Build ID does not match manifest")
    return dict(path=str(loaded), sha256=manifest["sha256"], build_id=build_id, pid=pid)


def prepare_workspace(parent: Path, build: dict) -> tuple[Path, dict]:
    parent = parent.resolve()
    parent.mkdir(parents=True, exist_ok=True)
    run_id = uuid.uuid4().hex
    folder = parent / ("EGFX-win-" + run_id)
    folder.mkdir()
    pattern(folder / "pattern.png")
    source = (ROOT / "tests/ae_runtime_smoke.jsx").read_text(encoding="utf-8")
    config = dict(run_id=run_id, folder=str(folder))
    for name, fn in (("arm", "elasticGridSmokeArm"), ("disarm", "elasticGridSmokeDisarm"), ("run", "elasticGridSmoke")):
        (folder / (name + ".jsx")).write_text(
            source + "\n" + fn + "(" + json.dumps(config) + ");\n", encoding="utf-8"
        )
    metadata = dict(schema=1, run_id=run_id, expected_build=build, status="NOT RUN",
                    actual_ae_execution=False, loaded_build_id=None)
    (folder / "run.json").write_text(json.dumps(metadata, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return folder, metadata


def wait_for(path: Path, timeout: float = 150.0) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if path.is_file() and path.stat().st_size:
            return
        time.sleep(0.1)
    raise TimeoutError("timed out waiting for " + path.name)


def run_jsx(afterfx: Path, script: Path, result: Path, timeout: float = 150.0) -> dict:
    started = time.monotonic()
    process = subprocess.run([str(afterfx), "-r", str(script)], capture_output=True, text=True, timeout=timeout)
    log = script.with_suffix(script.suffix + ".transport.log")
    log.write_text(process.stdout + "\n" + process.stderr, encoding="utf-8", errors="replace")
    wait_for(result, max(1.0, timeout - (time.monotonic() - started)))
    return dict(returncode=process.returncode, log=str(log))


def phase_record(path: Path, run_id: str, expected: str) -> dict:
    data = json.loads(path.read_text(encoding="utf-8-sig"))
    if data.get("run_id") != run_id or data.get("status") != expected:
        raise ValueError("AE phase did not complete: " + path.name)
    return data


def inspect_capture(folder: Path, run_id: str) -> dict:
    capture = json.loads((folder / "capture.json").read_text(encoding="utf-8-sig"))
    if capture.get("run_id") != run_id or capture.get("status") != "CAPTURED":
        raise ValueError("AE smoke capture failed")
    result = validate_frames(folder)
    result["frames_sha256"] = {name: sha256_file(folder / (name + ".png")) for name in FRAMES}
    result["ae_version"] = capture.get("ae_version")
    return result


def run_roundtrip(afterfx: Path) -> dict:
    temp = Path(tempfile.gettempdir())
    before = {p.resolve() for p in temp.glob("ElasticGridFX-roundtrip-*") if p.is_dir()}
    script = ROOT / "tests/ae_project_roundtrip.jsx"
    process = subprocess.run([str(afterfx), "-r", str(script)], capture_output=True, text=True, timeout=150)
    if process.returncode != 0:
        raise ValueError("roundtrip script returned nonzero status")
    candidates = [p.resolve() for p in temp.glob("ElasticGridFX-roundtrip-*") if p.is_dir() and p.resolve() not in before]
    if len(candidates) != 1:
        raise ValueError(f"roundtrip did not produce one new evidence folder: {len(candidates)}")
    folder = candidates[0]
    project = folder / "project.aep"
    frame = folder / "frame.png"
    if not project.is_file() or project.stat().st_size <= 0 or not frame.is_file() or frame.stat().st_size <= 0:
        raise ValueError("roundtrip evidence is incomplete")
    return dict(status="PASS", folder=str(folder), project_sha256=sha256_file(project),
                frame_sha256=sha256_file(frame), transport_returncode=process.returncode)


def execute(args) -> dict:
    _require_windows()
    manifest = load_manifest(args.manifest.resolve(strict=True))
    identity = verify_candidate(args.candidate, args.installed_aex, manifest)
    pid = running_selected_ae(args.afterfx)
    folder, metadata = prepare_workspace(args.run_root, manifest["build"])
    result = dict(schema=1, status="FAIL", artifact=identity, afterfx=str(args.afterfx.resolve()),
                  pid=pid, workspace=str(folder), checks={})

    result["checks"]["arm_transport"] = run_jsx(args.afterfx, folder / "arm.jsx", folder / "arm.json")
    result["checks"]["arm"] = phase_record(folder / "arm.json", metadata["run_id"], "ARMED")
    result["loaded_module"] = verify_loaded_effect(pid, args.installed_aex, manifest)
    result["checks"]["disarm_transport"] = run_jsx(args.afterfx, folder / "disarm.jsx", folder / "disarm.json")
    result["checks"]["disarm"] = phase_record(folder / "disarm.json", metadata["run_id"], "CLEAN")
    if running_selected_ae(args.afterfx) != pid:
        raise ValueError("After Effects process changed during validation")

    result["checks"]["smoke_transport"] = run_jsx(args.afterfx, folder / "run.jsx", folder / "capture.json")
    result["checks"]["pixels"] = inspect_capture(folder, metadata["run_id"])
    if result["checks"]["pixels"].get("status") == "FAIL":
        raise ValueError("pixel validation failed")

    result["checks"]["roundtrip"] = run_roundtrip(args.afterfx)
    if running_selected_ae(args.afterfx) != pid:
        raise ValueError("After Effects process changed during roundtrip validation")

    result["status"] = "BLOCKED"
    result["remaining"] = [
        "cold-start first-application/auto-binding validation",
        "interactive custom-UI guide drag and cursor validation",
        "Undo/Redo validation through the real UI",
        "MFR/render-queue/aerender validation for the exact Windows artifact",
    ]
    result["loaded_build_id"] = result["loaded_module"]["build_id"]
    out = folder / "windows-validation.json"
    out.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate", required=True, type=Path)
    parser.add_argument("--manifest", required=True, type=Path)
    parser.add_argument("--installed-aex", required=True, type=Path)
    parser.add_argument("--afterfx", required=True, type=Path)
    parser.add_argument("--run-root", type=Path, default=Path(tempfile.gettempdir()) / "ElasticGridFX-windows-validation")
    args = parser.parse_args()
    try:
        result = execute(args)
    except (ValueError, OSError, RuntimeError, TimeoutError, subprocess.SubprocessError) as error:
        print("ERROR: " + str(error), file=sys.stderr)
        return 1
    print(json.dumps(result, indent=2, sort_keys=True))
    return 3 if result["status"] == "BLOCKED" else 0


if __name__ == "__main__":
    raise SystemExit(main())
