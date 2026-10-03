#!/usr/bin/env python3
"""Package an already-built Windows cdylib as an identified AE .aex. No install."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil


MARKER = re.compile(rb"ElasticGridBuildID=(EGFX-[0-9a-f]{24})")


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def load_identity(path: Path) -> dict:
    data = json.loads(path.read_text(encoding="utf-8"))
    required = {"build_id", "target", "commit", "source_state", "source_sha256"}
    missing = sorted(required - data.keys())
    if missing:
        raise ValueError("identity missing fields: " + ", ".join(missing))
    if "windows-msvc" not in data["target"]:
        raise ValueError("identity is not a Windows MSVC build")
    if data.get("artifact_type") != "AE native effect (.aex)":
        raise ValueError("identity does not describe a Windows .aex artifact")
    return data


def package(dll: Path, identity_file: Path, out_dir: Path) -> tuple[Path, Path]:
    dll = dll.resolve(strict=True)
    identity_file = identity_file.resolve(strict=True)
    identity = load_identity(identity_file)

    payload = dll.read_bytes()
    markers = {m.decode("ascii") for m in MARKER.findall(payload)}
    if markers != {identity["build_id"]}:
        raise ValueError("binary Build ID does not match BuildIdentity.json")

    out_dir.mkdir(parents=True, exist_ok=True)
    aex = out_dir / "FSTR Stretch.aex"
    if aex.exists():
        raise FileExistsError(aex)
    shutil.copyfile(dll, aex)

    record = {
        "schema": 1,
        "artifact": aex.name,
        "sha256": sha256(aex),
        "size": aex.stat().st_size,
        "build": identity,
        "source_binary": dll.name,
        "validation": {
            "windows_pe_build": "NOT RUN",
            "after_effects_load": "NOT RUN",
            "after_effects_runtime": "NOT RUN",
        },
    }
    manifest = out_dir / "FSTR Stretch.windows-artifact.json"
    manifest.write_text(json.dumps(record, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return aex, manifest


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dll", required=True, type=Path)
    parser.add_argument("--identity", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    aex, manifest = package(args.dll, args.identity, args.out)
    print(aex)
    print(manifest)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
