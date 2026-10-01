# Development rules

The authoritative process for this project is the complete document:

https://github.com/ios3kov/AE-Development-Rules/blob/main/DEVELOPMENT_RULES.md

Reference re-read before the FSTR Stretch 0.9.4 performance cycle on 2026-10-01:
- rules repository commit: `320f80902fe5f50041935712e8d144ac61d29687`;
- DEVELOPMENT_RULES.md blob: `740f752dd9bd437391e141aea333646798bd547e`.

This file is a pointer, not a shortened replacement. Re-read the authoritative
rules before every significant stage and record a substantive revision of that
source when it changes.

For current implementation, verification and release blockers, read
`docs/current-status.md`. Historical PASS reports do not approve newer artifacts.
Do not publish or hand off an installable candidate while a required check is FAIL,
BLOCKED or NOT RUN. Never discard user work or represent a mock, compilation,
source audit or portable benchmark as a real After Effects runtime test.
