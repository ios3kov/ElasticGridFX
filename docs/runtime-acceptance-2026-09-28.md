# Runtime acceptance tooling — 2026-09-28

Baseline: 37bc8bc924e07f11eef25c989490aec8ac0180ef. Plan/acceptance was recorded in
runtime-acceptance-plan-2026-09-28.md before implementation. No renderer, Rust host
or serialized parameter changes. **NOT a release, and no actual AE/Metal run.**

## Reproduced baseline failures

The old install_macos.command ignores --help and attempts sudo operations. A
fixture executes that real script with a fake sudo which refuses mutation;
expected exit 0, actual 73 / UNSAFE_MUTATION_ATTEMPT. No system directories or
user plugins were changed. The initial suite also found a too-narrow test
exception expectation for symlink refusal; that fixture was corrected to accept
the deliberate ValueError refusal as well as an OS refusal.

The old smoke assumes a failed/unknown dirty-state query means safe. Executing
that JSX in a VM returns exit 0 in the unsafe-state regression; the revised
script fails closed. A separate image fixture demonstrates the coverage gap:
seven valid patterned PNG-equivalent frames that all pass through unchanged
FAIL the deformation and animation checks. File presence cannot close this gate.

## Installer

`tools/install_macos.command` calls `install_candidate.py`. Default: inspect only.
It verifies the signed-candidate payload/ZIP hashes, lists candidate conflicts
from common MediaCore, per-user MediaCore, AE-specific roots and supplied custom
roots. Plist identity finds renamed bundles; nested FSTR FX copies are included.
Symlinks, unreadable scan paths and excessive nesting refuse the operation.
Absence from these explicit roots is not proof about undocumented/custom roots.

Only `--apply-test-install --test-scope <existing-directory>` permits a write,
and the CLI additionally requires macOS and matching native architecture.
The test scope must already be authorized, canonical (no symlink ancestors),
private to this operation and not concurrently edited by unrelated software.
Running AE/aerender/Premiere/Media Encoder/Dynamic Link blocks installation;
no process is killed and no security preference is altered. No implicit sudo.

Copy into a uniquely named non-loading staging directory, verify the exact
payload/signature again, recheck hosts/conflicts, reserve the destination with
exclusive mkdir, then publish Contents. Do not overwrite even an existing empty
directory. Same identical verified candidate is idempotent. Other builds are
preserved and refused. Successful and failed staged operations retain receipts;
failed/partial publication remains for diagnosis, not recursively deleted.
A crash between destination reservation and publication can leave an incomplete
new directory. This is not an upgrade/rollback installer; existing data is never
replaced. Upgrades/removal require a separate authorized reversible plan.

The legacy BUILD_AND_INSTALL_MAC.command now only builds. The old --install
build flag is rejected before toolchain/build actions. No user install happened.

## Patterned smoke

`tools/ae_smoke_test_macos.command` / `ae_runtime_check_macos.command` call one
runner. With no execution flag it only prepares a new workspace (NOT RUN).
Use --help for exact options. Execution requires --execute-in-test-ae, --ae-app
and --installed-bundle, the signed package manifest and a running authorized
test AE. The runner requires one selected executable rather than choosing the
highest installed version. Old --full/--launch shortcuts no longer claim PASS.

The JSX imports a deterministic 319x241 opaque colored/checkered fixture and
captures Final Bicubic at project depth 32: bypass, identity, static wave at two
times, moving wave at two times, and amplitude-zero reset. It retains all frames.
The legacy hidden Wave Animation checkbox is not used. Known parameter values
are recorded in the versioned runner source; no state migration is introduced.

Five pixel checks: bypass≈identity; identity differs from static wave; static wave
is unchanged across time; moving wave changes across time; reset≈identity.
Equality allows at most 1/255 normalized channel error. Required changes affect
at least 1% of pixels by more than 2/255 and mean channel error at least 0.001.
All frames must remain opaque and spatially nonflat in at least two color
channels. These thresholds are smoke criteria, not a measured quality guarantee.

The bounded stdlib PNG decoder accepts noninterlaced 8/16-bit gray/RGB/gray-alpha/
RGBA, verifies CRCs/deflate bounds and rejects unsupported formats, truncated or
oversized data. Indexed/interlaced/tRNS PNGs require extending and testing the
decoder rather than silently bypassing comparisons. PNG captures do not verify
32-bit floating HDR precision, exact deformation geometry, viewer dragging,
GPU dispatch, render queue or aerender. Those remain separate mandatory gates.

Fresh random Run ID and separate directory; existing result/frame files refuse
reuse. The JSX reports CAPTURED, never image PASS. Parent validates the current
run, fixture hash and pixels; keeps hashes and tool versions. AppleScript has a
120-second limit, its transport process 125 seconds. Timeout is BLOCKED: AE may
continue executing; no host kill, workspace reuse or automatic retry occurs.

Project safety: absent/saved/occupied/dirty/unknown state refuses execution.
Only created comp/footage are removed; foreign project context is not touched.
Cleanup/write/transport errors cannot become PASS. Initial project bit depth is
restored, but its dirty flag/UI state may change through normal AE operations.
The runner does not close that project or reset preferences to conceal this;
a fresh authorized empty project is needed for subsequent host tests. The older
roundtrip checker remains a separate persistence diagnostic, not a full gate.

Expected disk identity is explicitly distinguished from loaded identity:
loaded_build_id remains null unless a separate real host observation is added.
Even if all pixel checks pass, the runner returns BLOCKED (exit 3), not full AE
or release PASS. Source/package hashes cannot prove what AE actually loaded.

## Executed checks

Linux: Python 53/53; smoke VM 14/14; prior roundtrip VM 11/11; strict GCC Release
C++ 10/10 (normal full fuzz and soak); shell syntax, Python compile and whitespace
checks PASS. Filesystem tests use isolated fixtures; native signatures/process
state are injected test doubles. Positive image fixtures are synthetic; they do
not claim to be real plugin renders. C++ sources are unchanged in this stage.

CI runs these checks on the committed source. macOS source workflow additionally
runs the portable tooling tests, read-only inspection of its actual signed
package, and smoke preparation only. Consult exact-head Actions/PR evidence for
those results. Actual installation, AE capture, loaded identity and physical
Metal/performance tests remain NOT RUN/BLOCKED; no binaries handed over.

## References used for implementation

- Adobe-derived SDK installation guide:
  https://ae-plugins.docsforadobe.dev/intro/where-installers-should-put-plug-ins/
- Adobe-derived/community scripting reference:
  https://ae-scripting.docsforadobe.dev/general/project/
  (dirty is documented there as officially undocumented; absence refuses tests)
- https://ae-scripting.docsforadobe.dev/general/application/
- https://docs.python.org/3.12/library/shutil.html
- PNG specification: https://www.w3.org/TR/png-3/
