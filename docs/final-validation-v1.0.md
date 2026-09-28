# Historical final-validation record — superseded

The old v1.0 validation plan predates the current original-GridWarp parity rebuild and the real-AE failure that exposed stale SmartFX/ROI behavior.

**Status: superseded. Do not treat any PASS/READY statement from the old v1.0 milestone as current evidence.**

The current branch requires, before another user test:
- original-binary contract research;
- current portable regression;
- sanitizers and static analysis;
- SmartFX/Visualization/clean-installer contract gates;
- macOS source/preflight gate;
- performance evidence;
- Build / Artifact Identity;
- current clean package hash;
- final code audit/debugging/profiling;
- only then a clean real-AE integration/profiling run.

See `docs/automatic-parity-audit.md` and `DEVELOPMENT_RULES.md`.
