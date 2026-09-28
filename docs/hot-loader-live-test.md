# ElasticGrid FX — AE Hot Loader live test

Target: After Effects 25.6 / Apple Silicon Mac.

This is the next live gate after the AE Hot Loader Control Shell gate, which is already passed.

## Important cache note

Manual **Edit → Purge** is not part of the intended product workflow. During validation, if AE reuses an old cached frame, force a fresh evaluation by changing any visible ElasticGrid parameter slightly (and change it back afterward). AE Hot Loader itself still has an open product requirement to invalidate stale cached output automatically after a successful reload.

## 1. Install the stable ElasticGrid shell

1. Fully quit After Effects.
2. Run `INSTALL_HOT_LOADER.command`.
3. Enter the macOS password only if the existing ElasticGrid is installed system-wide.
4. The installer backs up the existing ElasticGrid bundle, installs the hot-reload shell in the same location, clears any stale staged candidate, and verifies the signed arm64 bundle.
5. Start After Effects once.

Do not install a second ElasticGrid copy. The installer refuses ambiguous duplicates.

## 2. Verify bundled default

1. Open an existing project that already uses ElasticGrid if available; otherwise apply **ElasticGrid FX** to a test layer.
2. Confirm the effect resolves under the existing match name `com.elasticgrid.fx.warp`.
3. Confirm the existing/custom viewer UI is present and interactive.
4. Confirm the image really deforms when ElasticGrid controls are changed.
5. Preview several frames.

Expected: no missing effect, no reset parameter IDs, no crash, no parasitic render artifacts.

## 3. Hot reload default → candidate

1. Keep After Effects open.
2. Run `STAGE_CANDIDATE.command`.
3. Open `Window → AE Hot Loader`.
4. Click **Reload Plugins** once.

Expected panel result includes an ElasticGrid shell with:
- `reloaded=1`;
- `failed=0`;
- label `elasticgrid-candidate-v2`.

5. Change one ElasticGrid parameter slightly to force a fresh evaluation.
6. Preview/render again.
7. Confirm deformation and custom UI still work.

## 4. Unchanged/no-op

Click **Reload Plugins** again without staging another build.

Expected ElasticGrid result: `unchanged=1`, `failed=0`.

## 5. Rollback

1. Run `ROLLBACK_TO_BUNDLED.command`.
2. Keep AE open.
3. Click **Reload Plugins**.

Expected: ElasticGrid reloads `elasticgrid-default`.

4. Change one ElasticGrid parameter slightly and Preview again.
5. Confirm the effect still renders correctly.
6. Click Reload once more; expected `unchanged=1`.

## 6. Send back

Run `COLLECT_LOGS.command` and send the output together with:
- whether the existing project/effect instance survived;
- whether the custom viewer UI still worked;
- whether deformation was correct before and after reload;
- whether CPU/GPU preview showed any visual difference or artifacts.

The later live pass will add focused MFR, SmartPreRender/SmartRender generation, Metal setup/render/setdown, save/reopen, and repeated A→B→C stress once this basic adapter gate is clean.
