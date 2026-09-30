# Host metadata and build identity — 2026-09-28

Baseline: e1aa77e79cea7a4b1c63e7285871338d8ec76224, fix/final-validation.
Acceptance/risk plan: host-identity-plan-2026-09-28.md.
Status: implementation and portable tooling tests completed; exact-head macOS
compile/package and actual AE host gates must be read separately. No release.

## Parameter contract (no render algorithm or wire-format changes)

The previous host labels disagreed with existing FFI behavior. Fix labels rather
than remapping saved popup numbers: 1 Smoothstep, 2 Gaussian, 3 Linear,
4 Smoothstep (Legacy). Keep all four slots and default ordinal 2. Existing
projects in this parity branch therefore keep the same effective falloff.
A Rust-to-C++ regression compares all four ordinals against baseline drag output.

Columns/Rows allow 1–50 internal guides, matching the existing core clamp.
Tension Radius maximum becomes 20 and Wave Frequency maximum 10. Their old higher
UI values were already clamped in the core. Real AE loading/interpolation of
out-of-range legacy keys still needs host verification; this is not a claim of
lossless preservation of unsupported numeric values.
Wave Phase exposes -360..360 degrees; existing stored values are not rescaled.

The parity renderer deliberately enables waves by amplitude rather than by the
old Wave Animation checkbox. Hide that no-op checkbox using INVISIBLE, retaining
its identifier, type, order and serialized slot. Amplitude=0 remains wave-off;
nonzero amplitude with speed=0 remains a static wave. No new on/off semantics are
silently imposed on old projects. All 17 parameter IDs/order/types and grid wire
version 3 remain unchanged. Earlier pre-parity states need their documented
migration tests and actual AE acceptance; those are not inferred here.

## Source → build → package identity

`tools/build_identity.py generate` runs from Cargo build.rs and writes only into
OUT_DIR. It computes commit, clean/dirty state, content/executable-bit hashes of
source files, target, profile, compiler versions and a hash of the selected
build-affecting environment settings. Build ID is a deterministic 96-bit SHA-256
prefix with an EGFX prefix. It is not the final artifact hash or a code-signing
attestation. Build settings are hashed rather than exposing local path/flag
values in the distributed metadata.

Full identity is available in About and one startup diagnostic per GlobalSetup.
The existing custom control shows a shortened ID to fit its width. There are no
new saved parameters, render-time file accesses, UI dialogs or per-frame logs.
Cargo tracks source files, source directories, Git HEAD/index/ref (including
linked worktrees), and relevant build environment changes.

Reproducible builds export the source record from clean Git, then validate every
listed byte/executable bit and reject additional code in the isolated snapshot.
An environment-supplied commit string alone is never used as identity. A record
cannot override a different actual Git checkout. Repo-less records must come
from the trusted build pipeline; their checksums do not authenticate an attacker
who can rewrite both source and metadata.

The bundle command obtains the exact identity output directory from Cargo JSON
messages, rechecks current source state and rejects dirty/stale/mismatched
metadata. It embeds BuildIdentity.json before signing. After signing and native
bundle verification it creates ElasticGrid.plugin.zip from that same payload,
then records every file hash/executable bit and the final ZIP SHA-256 in
ElasticGrid.artifact.json. Verification detects missing/additional/modified
payload files, wrong binary markers, changed executable permission and archive
content/hash differences. No self-referential hash is embedded in the binary.
Signing/notarization beyond existing ad-hoc signing is not added by this stage.

The artifact record deliberately says runtime_verification=NOT RUN. Reading a
matching file on disk, a binary marker, a generated ID or a successful source
build is not evidence that the corresponding code was loaded by After Effects.
The runtime gate still requires actual target-AE observation of the full ID.

## Checks executed before the GitHub checkpoint

- Baseline C++ Release: 10/10 PASS (full fuzz and default soak). C++ production
  sources remain unchanged in this stage.
- New host-contract guards reproduce 3 source-contract failures on baseline
  metadata and pass all 5 after the correction. These parse source, not AE.
- Identity/tooling tests: 19/19 PASS; combined Python suite 24/24 PASS. Includes
  clean/dirty/changed state, compiler/settings/target identity, ignored outputs,
  worktrees, repo-less snapshots, source tampering, stale Cargo metadata, path
  traversal/symlinks, package contents and executable permission.
- Existing Node VM project-safety tests: 11/11 PASS; not a real host test.
- Shell syntax and git diff whitespace checks: PASS.
- Rust/AE host compilation, new FFI ordinal regression and bundle packaging:
  require the exact-head macOS source workflow result. Not run locally (Linux,
  no Rust toolchain/AE/Metal device in this execution environment).
- Actual loaded identity, viewer behavior, save/reopen, GPU/AE output and target
  performance: BLOCKED, not PASS. No claim of fixing the reported AE deformation
  failure is made without reproducing it inside the host.

## CI adjustment

The old macOS workflow ran all 20 preflight stages once explicitly and then a
second time inside build_macos_sdkless.command. Remove only the duplicate outer
call. The bundle command still runs the entire mandatory preflight, including
sanitizers, audits, Rust tests and two reproducible builds. No required check is
removed. `.preflight-macos/` is an ignored, test-owned generated directory so it
does not falsely mark source dirty. Workflow artifacts now retain final ZIP,
manifest, Cargo build record and prior reports.

## Primary references checked

- Cargo build scripts/change detection: https://doc.rust-lang.org/cargo/reference/build-scripts.html
- Pinned after-effects 0.4.0 parameter flags/PopupDef source:
  https://docs.rs/crate/after-effects/0.4.0/source/src/pf/parameters.rs
- Pinned return-message ABI guard (less than 256 bytes):
  https://docs.rs/crate/after-effects/0.4.0/source/src/pf/out_data.rs
- Existing project: src/bridge/elasticgrid_ffi.cpp and src/core/GridModel.h.
