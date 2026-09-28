# Automatic parity audit status

This file records automatic gates that must pass before another real-AE user test is requested.

## Required automatic gates

- portable GCC/Clang regression tests;
- ASan + UBSan;
- ThreadSanitizer MFR/determinism;
- static analysis;
- Rust implementation build/tests;
- SmartFX contract verifier;
- Hot Loader shell/reload smoke;
- macOS source/preflight build;
- Metal source compile;
- CPU parity benchmark + memory evidence;
- clean installer verification.

## Manual gate

A real After Effects run remains a separate final integration/profiling gate. It is not used as a substitute for the automatic checks above.


## Build / Artifact Identity gate

Build Identity is now an automatic prerequisite. The plugin embeds Git commit/state and Build ID, exposes it through About and Hot Loader, writes it as the first runtime log line, packages a BuildIdentity manifest, and hashes the final ZIP. CI rejects a packaged identity that does not match the tested clean commit.


## Build Identity baseline/fix

- Baseline: Hot Loader CI #77 — **FAIL** at bundle identity verification because a clean CI source was embedded as `dirty`.
- Required behavior: clean source is verified before build outputs are created; the verified state is then embedded in the artifact.
- Target/platform and Rust toolchain are part of the build record.
- A successful compile alone does not close this gate; package manifest, Info.plist, exports and final ZIP SHA-256 must agree.


## Native pixel-contract coverage — automatic Level 1

Research basis:
- Adobe SmartFX documentation requires render inputs to be checked out and supports preserving RGB under zero alpha.
- AE's 16-bpc API uses `PF_MAX_CHAN16` rather than the full `uint16` range.
- The host/core is channel-agnostic for the warp itself, so it must not silently reinterpret premultiplication or clamp 32f extended range.

New automatic regression coverage:
- 8 bpc identity including non-zero RGB under alpha=0;
- 16 bpc identity using the AE 0..32768 channel range;
- 32f identity preserving negative, >1.0, NaN and ±Inf payloads;
- deformed 32f keeps finite extended-range samples outside 0..1;
- transparent RGB survives deformation without alpha fabrication;
- padded rowbytes preserve padding;
- undersized/misaligned 16-bit rowbytes fail closed.

These tests prove the portable CPU/bridge contract only. Working Space, Linear Working Space, OCIO and real AE premultiplication integration remain **NOT RUN** until the controlled AE integration gate.
