# Historical Mac runtime plan — superseded

The previous text said the v1.0 candidate was “READY TO RUN ON TARGET MAC”. That statement is no longer current.

The current original-parity branch is **not yet cleared for a user manual test**. The previous real-AE run failed, and subsequent work changed SmartFX parameter checkout, SmartPreRender, drag-event behavior, rendered Visualization, clean-install duplicate detection and Build / Artifact Identity.

Current manual-test gate:
1. all automatic gates in `docs/automatic-parity-audit.md` pass on the same current commit;
2. current artifact identity and SHA-256 are recorded;
3. code audit / debugging / regression / performance review are complete;
4. only then run a clean After Effects integration/profiling test.

This file is retained only as historical context.
