#!/usr/bin/env python3
"""Run the controlled FSTR Stretch 0.9.3 aerender baseline matrix for 0.9.4 Stage 1.

This tool is read-only with respect to the installed plug-in and user projects.
It requires one already-running target After Effects instance whose live loaded
FSTR Stretch image matches the exact accepted 0.9.3 artifact. It creates only
test-owned synthetic projects and retained evidence under --evidence-root.

RAM Preview and native-3D profiling are deliberately NOT represented as PASS by
this runner; they remain separate real-host Stage 1 cases.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import platform
import sys
import uuid

import aerender_benchmark as ab
import build_identity as bi
from install_candidate import checked_path
import live_identity as li
import perf_fixture_runner as pf

BASELINE_COMMIT = "2ccc5f674b8b94b534d4c3ad6abdec3524e3624f"
BASELINE_BUILD_ID = "EGFX-bd19dee13315abc0b7e6090e"
BASELINE_VERSION = "0.9.3"
BASELINE_AE_PREFIX = "25.6"
BASELINE_PACKAGE_SHA256 = "1448a5231fc561e6b10491d1b9d1839de22f21a11266a43e2990244a25176ebc"
PACKAGE_NAME = "FSTR Stretch.plugin.zip"
MANIFEST_NAME = "FSTR Stretch.artifact.json"


def matrix_conditions(name: str) -> list[dict]:
    if name not in ("smoke", "core"):
        raise ValueError("unsupported baseline matrix")
    smoke = [
        dict(condition_id="grid-1080p-32-animated-mfr-on", test_case_id="PERF094-RQ-001",
             width=1920, height=1080, bit_depth=32, mode="animated", geometry="grid", mfr="on"),
        dict(condition_id="grid-4k-32-animated-mfr-on", test_case_id="PERF094-RQ-002",
             width=3840, height=2160, bit_depth=32, mode="animated", geometry="grid", mfr="on"),
        dict(condition_id="four-corners-4k-32-animated-mfr-on", test_case_id="PERF094-RQ-004",
             width=3840, height=2160, bit_depth=32, mode="animated", geometry="four_corners", mfr="on"),
    ]
    if name == "smoke":
        return smoke

    result = []
    for width, height, case in ((1920,1080,"PERF094-RQ-001"),(3840,2160,"PERF094-RQ-002")):
        label = "1080p" if width == 1920 else "4k"
        for mode in ("static","animated"):
            for mfr in ("on","off"):
                result.append(dict(
                    condition_id=f"grid-{label}-32-{mode}-mfr-{mfr}",
                    test_case_id=case,width=width,height=height,bit_depth=32,
                    mode=mode,geometry="grid",mfr=mfr))
    for depth in (8,16,32):
        result.append(dict(
            condition_id=f"rq3-grid-4k-{depth}-static-mfr-on",
            test_case_id="PERF094-RQ-003",width=3840,height=2160,bit_depth=depth,
            mode="static",geometry="grid",mfr="on"))
    for mode in ("static","animated"):
        for mfr in ("on","off"):
            result.append(dict(
                condition_id=f"four-corners-4k-32-{mode}-mfr-{mfr}",
                test_case_id="PERF094-RQ-004",width=3840,height=2160,bit_depth=32,
                mode=mode,geometry="four_corners",mfr=mfr))
    return result


def discover_aerender(ae_app: Path) -> Path:
    candidates = [
        ae_app.parent / "aerender",
        ae_app / "Contents" / "MacOS" / "aerender",
    ]
    found = []
    for candidate in candidates:
        try:
            candidate = checked_path(candidate)
        except (OSError, ValueError):
            continue
        if candidate.is_file() and candidate.name == "aerender":
            found.append(candidate)
    unique = []
    for candidate in found:
        if not any(candidate.samefile(existing) for existing in unique):
            unique.append(candidate)
    if len(unique) != 1:
        raise ValueError("could not resolve exactly one aerender executable for the running AE")
    return unique[0]


def candidate_files(candidate_dir: Path) -> tuple[Path, Path, dict]:
    candidate_dir = checked_path(candidate_dir, directory=True)
    package = bi.safe_file(candidate_dir, PACKAGE_NAME)
    manifest_path = bi.safe_file(candidate_dir, MANIFEST_NAME)
    manifest = json.loads(manifest_path.read_text())
    build = bi.validate_identity(manifest["build"])
    if build.get("commit") != BASELINE_COMMIT:
        raise ValueError("candidate manifest is not the frozen 0.9.3 source commit")
    if build.get("build_id") != BASELINE_BUILD_ID:
        raise ValueError("candidate manifest is not the frozen 0.9.3 Build ID")
    if build.get("version") != BASELINE_VERSION:
        raise ValueError("candidate manifest is not version 0.9.3")
    if ab.file_digest(package) != BASELINE_PACKAGE_SHA256:
        raise ValueError("candidate package SHA-256 differs from frozen 0.9.3 baseline")
    return package, manifest_path, manifest


def running_target() -> dict:
    hosts = li.running_ae()
    if len(hosts) != 1:
        raise ValueError("require exactly one running After Effects process")
    host = hosts[0]
    app = checked_path(Path(host["app"]), directory=True)
    executable = checked_path(Path(host["executable"]))
    return dict(pid=host["pid"], app=app, executable=executable)


def require_same_host(expected: dict) -> None:
    current = running_target()
    if current["pid"] != expected["pid"] or current["executable"] != expected["executable"]:
        raise ValueError("target After Effects process changed during baseline run")


def live_baseline_identity(run_root: Path, manifest: dict, host: dict) -> tuple[dict, Path]:
    folder = run_root / "identity"
    folder.mkdir(mode=0o700)
    result = li.diagnose(folder, manifest, [])
    public = li.public_record(result)
    bi.dump(run_root / "identity-report.json", public)
    if result.get("status") != "PASS" or result.get("loaded_image_status") != "PASS":
        raise ValueError("live loaded baseline identity is BLOCKED; inspect identity-report.json")
    if result.get("observed_build_id") != BASELINE_BUILD_ID:
        raise ValueError("running AE loaded a different FSTR Stretch Build ID")
    if result.get("ae", {}).get("pid") != host["pid"]:
        raise ValueError("identity probe observed a different AE process")
    ae_version = str(result.get("ae", {}).get("version") or "")
    if not ae_version.startswith(BASELINE_AE_PREFIX):
        raise ValueError("Stage 1 baseline requires After Effects 25.6")
    if checked_path(Path(result["ae"]["path"]), directory=True) != host["app"]:
        raise ValueError("identity probe observed a different AE application")
    binary = checked_path(Path(result["loaded_images"][0]["path"]))
    installed = checked_path(binary.parent.parent.parent, directory=True)
    return result, installed


def write_summary(path: Path, value: dict) -> None:
    if path.exists() or path.is_symlink():
        raise ValueError("refusing stale baseline summary")
    bi.dump(path, value)


def validate_cross_condition_outputs(conditions: list[dict]) -> list[dict]:
    groups: dict[tuple, list[dict]] = {}
    for condition in conditions:
        key = (condition["width"], condition["height"], condition["bit_depth"],
               condition["mode"], condition["geometry"])
        groups.setdefault(key, []).append(condition)
    records = []
    for key, entries in groups.items():
        if len(entries) < 2:
            continue
        digests = {entry["summary"]["output_digest"] for entry in entries}
        record = dict(
            width=key[0], height=key[1], bit_depth=key[2],
            mode=key[3], geometry=key[4],
            condition_ids=[entry["condition_id"] for entry in entries],
            mfr_states=sorted({entry["mfr"] for entry in entries}),
            status="PASS" if len(digests) == 1 else "FAIL",
            encoded_output_digests=sorted(digests),
        )
        records.append(record)
        if len(digests) != 1:
            raise ValueError("encoded output differs across equivalent MFR/test-case conditions")
    return records


def run_baseline(evidence_root: Path, candidate_dir: Path, matrix: str,
                 warmups: int, samples: int, timeout: int) -> tuple[dict, Path]:
    if platform.system() != "Darwin" or platform.machine() != "arm64":
        raise ValueError("0.9.4 target baseline requires Apple Silicon macOS")
    if not 0 <= warmups <= 5 or not 5 <= samples <= 20 or timeout < 30:
        raise ValueError("invalid baseline timing settings")

    evidence_root = evidence_root.expanduser()
    if evidence_root.exists():
        evidence_root = checked_path(evidence_root, directory=True)
    else:
        parent = checked_path(evidence_root.parent, directory=True)
        evidence_root = parent / evidence_root.name
        evidence_root.mkdir(mode=0o700)

    run_root = evidence_root / ("FSTR-Stretch-094-baseline-" + uuid.uuid4().hex)
    run_root.mkdir(mode=0o700)
    result = dict(
        schema=1,
        status="IN_PROGRESS",
        stage="0.9.4 Stage 1/10",
        release="BLOCKED",
        performance_claim_allowed=False,
        baseline=dict(commit=BASELINE_COMMIT,build_id=BASELINE_BUILD_ID,
                      version=BASELINE_VERSION,package_sha256=BASELINE_PACKAGE_SHA256),
        matrix=matrix,
        settings=dict(warmups=warmups,samples=samples,timeout_seconds=timeout),
        conditions=[],
        real_host=dict(ram_preview="BLOCKED / NOT RUN",
                       native_3d="BLOCKED / NOT RUN",
                       deep_profile="BLOCKED / NOT RUN"),
    )
    bi.dump(run_root / "run-start.json", result)

    try:
        package, manifest_path, manifest = candidate_files(candidate_dir)
        host = running_target()
        aerender = discover_aerender(host["app"])

        identity, installed = live_baseline_identity(run_root, manifest, host)
        ab.verify_candidate(installed, package, manifest_path)
        require_same_host(host)
        result["identity"] = dict(
            status="PASS", ae_pid=host["pid"],
            ae_version=identity["ae"]["version"],
            observed_build_id=identity["observed_build_id"],
            observed_image_uuid=identity["observed_image_uuid"],
            installed_bundle=str(installed),
            aerender=str(aerender),
        )

        fixtures_root = run_root / "fixtures"
        fixtures_root.mkdir(mode=0o700)
        runtime_identity = None
        for condition in matrix_conditions(matrix):
            require_same_host(host)
            folder, meta = pf.prepare(
                fixtures_root, condition["width"], condition["height"],
                condition["bit_depth"], condition["mode"], condition["geometry"])
            meta = pf.execute(folder, meta, host["app"], installed, package, manifest_path)
            bi.dump(folder / "prepare.json", meta)
            require_same_host(host)
            if runtime_identity is None:
                runtime_identity = ab.runtime_identity_preflight(
                    folder,aerender,installed,package,manifest_path,host["executable"])
                result["aerender_runtime_identity"] = ab.validate_runtime_identity(
                    runtime_identity,BASELINE_BUILD_ID)
                require_same_host(host)
            report, report_path = ab.benchmark(
                folder, folder/"fixture.json", aerender, installed, package, manifest_path,
                warmups, samples, condition["mfr"], timeout, condition["test_case_id"],
                runtime_identity)
            require_same_host(host)
            result["conditions"].append(dict(
                **condition,
                status="MEASURED",
                report=str(report_path.relative_to(run_root)),
                report_sha256=ab.file_digest(report_path),
                summary=report["summary"],
            ))

        result["cross_condition_output_parity"] = validate_cross_condition_outputs(result["conditions"])
        result["status"] = "AERENDER_BASELINE_MEASURED"
        result["stage_status"] = "BLOCKED"
        result["stage_blockers"] = [
            "PERF094-RAM-001/002/003 real RAM Preview baseline",
            "PERF094-3D-001 native 3D projective baseline",
            "PERF094-PROF-001 real-host deep profile",
        ]
        result["scope"] = (
            "Exact 0.9.3 live identity plus controlled aerender timing/memory and "
            "encoded-output repeatability. This is not RAM Preview, native-3D or HDR/32f quality certification."
        )
        summary = run_root / "baseline-summary.json"
        write_summary(summary, result)
        return result, summary
    except Exception as error:
        result["status"] = "BLOCKED"
        result["stage_status"] = "BLOCKED"
        result["conditions_completed"] = len(result["conditions"])
        result["reason"] = type(error).__name__ + ": " + str(error)
        failure = run_root / "baseline-failure.json"
        if not failure.exists():
            bi.dump(failure, result)
        raise


def main() -> int:
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--evidence-root", type=Path,
                   default=Path.home()/"Desktop/FSTR-Stretch-0.9.4-Baselines",
                   help="Evidence directory; created safely when its parent exists")
    p.add_argument("--candidate-dir", type=Path, required=True,
                   help=f"Directory containing {PACKAGE_NAME!r} and {MANIFEST_NAME!r}")
    p.add_argument("--matrix", choices=("smoke","core"), default="core")
    p.add_argument("--warmups", type=int, default=2)
    p.add_argument("--samples", type=int, default=5)
    p.add_argument("--timeout", type=int, default=1800)
    args = p.parse_args()
    try:
        report, path = run_baseline(args.evidence_root, args.candidate_dir, args.matrix,
                                    args.warmups, args.samples, args.timeout)
    except (OSError, ValueError, KeyError) as error:
        print("BLOCKED: "+str(error), file=sys.stderr)
        return 3
    print(json.dumps(dict(status=report["status"],stage_status=report["stage_status"],
                          summary=str(path),release="BLOCKED"),indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
