# ElasticGrid — clean original-parity live gate

Each independent run starts clean.

1. Fully quit AE.
2. Run `INSTALL_HOT_LOADER.command`.
3. Wait for `CLEAN ELASTICGRID ORIGINAL-PARITY INSTALL COMPLETE`.
4. Start AE and apply ElasticGrid FX.
5. Without Purge, drag an internal column guide: rendered pixels must deform immediately.
6. Drag an internal row guide: rendered pixels must deform immediately.
7. Strong drags must not create bright vertical/horizontal seam lines.

FAIL if only the overlay moves, pixels remain unchanged, or seam artifacts appear.

After bundled behavior passes: keep AE open → `STAGE_CANDIDATE.command` → Reload Plugins → verify `elasticgrid-candidate-v2` and deformation → second Reload must be unchanged → rollback → retest bundled behavior.
