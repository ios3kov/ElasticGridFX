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
