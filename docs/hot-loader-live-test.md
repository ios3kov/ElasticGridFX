# ElasticGrid FX — AE Hot Loader live test

Target: After Effects 25.6 / Apple Silicon Mac.

This is the next live gate after the AE Hot Loader Control Shell gate, which is already passed.

## Important cache note

Manual **Edit → Purge** is not part of the intended product workflow. During validation, if AE reuses an old cached frame, force a fresh evaluation by changing any visible ElasticGrid parameter slightly (and change it back afterward). AE Hot Loader itself still has an open product requirement to invalidate stale cached output automatically after a successful reload.

## 1. Clean install the stable ElasticGrid shell

1. Fully quit After Effects.
2. Run `INSTALL_HOT_LOADER.command`.
3. Enter the macOS password only if old ElasticGrid copies exist in system-wide Adobe plug-in folders.
4. Wait for `CLEAN ELASTICGRID INSTALL COMPLETE`.
5. Start After Effects once.

The installer performs a clean migration:

- scans system/user MediaCore, Adobe Plug-Ins/CC, and AE application plug-in roots;
- finds every `ElasticGrid.plugin` and every bundle using `com.elasticgrid.fx`;
- refuses to modify a copy inside the signed Adobe application bundle;
- moves all other old/test/duplicate ElasticGrid copies to timestamped backups;
- clears stale ElasticGrid candidate dylibs, ElasticGrid runtime images, and relevant Loader logs;
- installs exactly one fresh shell at the canonical user path:
  `~/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin`;
- verifies signature, arm64 architecture, bundle ID, and that exactly one active ElasticGrid copy remains.

Nothing outside ElasticGrid / its Hot Loader test state is removed. Existing files are backed up rather than deleted.

## 2. Verify bundled default

1. Open an existing project that already uses ElasticGrid if available; otherwise apply **ElasticGrid FX** to a test layer.
2. Confirm the effect resolves under the existing match name `com.elasticgrid.fx.warp`.
3. Confirm the existing/custom viewer UI is present and interactive.
4. Drag an internal grid guide clearly away from its default position.
5. Confirm the **pixels deform immediately with the guide**; moving only the overlay is a FAIL.
6. Preview several frames.

This package explicitly checks all render-affecting parameters during SmartPreRender and uses a full-source CPU SmartFX checkout for the first correctness gate. This prevents stale SmartFX cache reuse and ROI seams from masking the actual deformation.

Expected: no missing effect, no reset parameter IDs, image deformation follows the guide, no crash, no parasitic render artifacts.

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

The later live pass will re-enable and validate Metal, then add focused MFR, SmartPreRender/SmartRender generation, save/reopen, and repeated A→B→C stress once CPU deformation/cache correctness is clean.
