# Non-installing live-image diagnostic

Scope and predefined acceptance: live-check-plan-2026-09-28.md. This is a separate
source-identified diagnostic artifact. Its baseline candidate remains the already
inspected 6d3b846 / EGFX-603e9d3e4025d271e0488201. No native source/ABI changes.

The previous smoke kept loaded_build_id null because disk hashes do not prove
loading. The new tool observes a live image UUID and path via Apple's sample,
checks the exact signed payload before/after, and maps the observed UUID to the
pinned candidate. `direct_runtime_build_id` remains null: this is explicitly a
UUID-to-known-build association, not a direct read of the Build ID from memory.
LC_UUID is a linker identifier, not a cryptographic attestation against injected
code. Full image/pixel correctness and release status are always separate.

User workflow: save work, leave one idle AE open, run the diagnostic launcher,
return the generated report ZIP. No plugin is distributed in that ZIP or in the
diagnostic package. No JSX, saving/closing projects, automatic install/update,
preferences, elevated permissions or process kill. The sampler briefly pauses
threads; it runs once for 1 second at 10 ms intervals and has a 15-second timeout.
Failure never causes an automatic retry or security-setting change.

Reports have unique directories. Only AE/app/plugin identity records and redacted
user paths go into the report ZIP. The complete sample is private local evidence
under a mode-0700 per-run directory, not included in the shareable archive. The
report is never uploaded automatically. Unsupported formats, unreadable scan
roots, duplicates, stale captures, different UUID/hash/path and process changes
are BLOCKED, not PASS. Custom roots must be supplied explicitly when needed.

Local verification: 20 diagnostic unit/pipeline tests PASS; the Apple sampler
integration test is NOT RUN locally (Linux). Existing 53 Python tests and 25 JSX
control-flow cases pass. Strict C++ Release 10/10 passed; native renderer files
are unchanged. Mach-O reader also inspected the actual pinned baseline binary:
UUID 00A8E7CC-AD95-35AA-ADB6-F076A4E83AD7; all six pinned file hashes match.
The first unit run exposed a bytearray/bytes fixture mismatch, corrected by
passing bytes to the bytes-typed parser; no failed run is counted as PASS.

Required macOS evidence: the separate Live-image diagnostic workflow samples an
OWNED child process which dlopens an OWNED fixture dylib. It tests the native
sampler/path/UUID chain and rejects a mismatched UUID. This is not AE; only that
child may be terminated by the fixture cleanup. Diagnostic packaging requires
clean Git and verifies all included bytes. Exact-commit results and final ZIP
hash are recorded in the PR checkpoint. No actual target AE result is claimed
before the user's unique-environment diagnostic is received.

Primary references:
- Apple's loader header for Mach-O LC_UUID:
  https://github.com/apple-oss-distributions/cctools/blob/main/include/mach-o/loader.h
- Apple binary image UUID/path explanation:
  https://developer.apple.com/documentation/xcode/interpreting-the-json-format-of-a-crash-report
- Apple build UUID matching:
  https://developer.apple.com/documentation/xcode/locating-a-missing-debug-symbol-file
The actual current sample behavior is validated by the native fixture gate,
not inferred solely from documentation or a mocked process.

## Native-gate follow-up

The first native run 36464999983 (a76622f) failed at the strict process-header
comparison; it is not PASS. Explicit -fullPaths sampling and retained fixture
header/image diagnostics are now being checked. No user permissions are changed
and no redacted/mismatched path is silently accepted. The exact follow-up native
result is required before handing over the diagnostic.

The follow-up run36465410962 reproduced the same refusal even with -fullPaths.
The retained native fixture report proves the reason: Path was replaced with
/private/var/folders/*/probe-host and its image paths were similarly redacted.
No wildcard is accepted as an exact path. The native fixture now covers both
an owned non-private /Users/Shared location (required positive exact match) and
the default private temporary location (evidenced redaction must be BLOCKED).
This changes fixture coverage, not production acceptance. A new unit case keeps
that observed redaction from being accidentally accepted. Neither historical
failed run is retroactively counted as successful; no diagnostic is delivered
until the positive native fixture and packaging checks pass.

## Independent mapped-path observation

Run36466002735 showed /Users/Shared was also masked, so moving the fixture did
not solve the diagnostic. The unchanged exact-path requirement is now satisfied
by independent native observations: proc_pidpath verifies the executable when
the sampler masks its header, and proc_regionfilename queries the mapped file
at the load address of each masked candidate image. PID/start-time checks still
bracket sampling; UUID comes from the live image table and is matched to the
hashed signed payload. A wildcard alone still proves nothing; native paths must
also agree with the visible sampler prefix/suffix, and contradictions fail.
Reports keep reported_path and path_source so native restoration is not hidden.

These libproc signatures are from Apple's xnu libproc.h, which labels the
interfaces private and subject to change. Their availability/behavior is gated
by real owned-child tests, now required at both private-temp and Shared paths.
Missing permission, missing symbol or invalid output must block rather than
request elevation or weaken matching. The native plugin and its artifact stay
unchanged. The diagnostic still does not read literal Build ID bytes from memory,
verify image deformation or approve release. User runtime remains NOT RUN.
Reference: https://raw.githubusercontent.com/apple-oss-distributions/xnu/main/libsyscall/wrappers/libproc/libproc.h
