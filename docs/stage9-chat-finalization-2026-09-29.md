# ElasticGridFX — итоги Stage 9 из чата 2026-09-29

Дата: 2026-09-29  
Ветка: `fix/final-validation`  
Статус: **Stage 9 of 10 — IN PROGRESS**  
Stage 8: **SKIPPED BY USER**, не PASS  
Stage 10: **NOT STARTED**

## Текущий установленный runtime-кандидат

- Commit: `de314981005606741bc75c517d8bb33798b46a1d`
- Build ID: `EGFX-0fa68430a170b3612e8d00f7`
- ZIP SHA-256: `6a43f734c7dc5fd298b356b12db75ab5986c171a2ae241ef9c85ffc0ac22ec55`
- State: `INSTALLED_FOR_TEST`
- Rollback receipt: `EGFX-update-371ef6e56f92407d8b04b16af52e2c10`

После `de31498` runtime-код в `src/` и `host-rust/` не менялся.
Все последующие изменения относятся только к тестам, acceptance tooling, workflows и документации.

## Что реализовано в Stage 9

- режимы Deformation Plane: Existing Grid / Four Corners;
- четыре native corner point-параметра;
- Fit Layer;
- общий projective plane для render / overlay / inverse drag;
- legacy Render и SmartRender;
- immutable SmartPreRender snapshot;
- 8/16/32 bpc;
- Draft Bilinear / Final Catmull-Rom;
- Clamp / Wrap / Mirror;
- invalid-plane fallback на original image;
- perspective guides;
- native corner handles;
- guide drag в plane coordinates;
- wave-aware guides;
- 3D layer / parent / camera acceptance fixture.

Legacy parameter IDs и wire schema сохранены. Новые параметры добавлены append-only.

## Stage 9 target-AE acceptance tooling

Добавлены:

- `tools/ae_plane_acceptance.py`
- `RUN_ELASTICGRID_PLANE_AE_TEST_MAC.command`
- `tests/ae_plane_smoke.jsx`
- `tools/plane_smoke_pixels.py`
- safety/control-flow regression tests.

Runner жёстко пинит:

- exact commit `de31498`;
- exact Build ID;
- exact ZIP SHA-256.

Подменённый manifest или package не принимается.

## Автоматическая матрица

Fixture рассчитан на 34 PNG:

### 24 depth/frame captures

Для 8/16/32 bpc:

1. original
2. identity
3. legacy-wave
4. plane-wave
5. skew-wave
6. invalid
7. half-identity
8. half-original

### Roundtrip

- `roundtrip-before`
- `roundtrip-after`

Проект сохраняется в реальный AEP и открывается повторно.

### 3D / camera

- no explicit camera / Default Camera
- 3d-base
- 3d-position
- 3d-scale
- 3d-layer-rotate
- 3d-camera-move
- 3d-parent
- 3d-camera-switch

Numeric comparator проверяет равенство / изменение кадров и отказывается принимать flat/blank frames.

## Исправления acceptance tooling

### 1. Portable C++ test include

В `tests/test_plane_bridge.cpp` добавлен `<cmath>`.

Commit: `0921708`

### 2. Saved parameter contract

Legacy 17 IDs закреплены как immutable prefix, Stage 9 append разрешён только явно.

Commit: `8d9e546`

### 3. Exact package digest

Stage 9 runner теперь проверяет известный package SHA-256, а не только commit/Build ID.

Commits:

- `7b36452`
- `0291bb8`

### 4. Safe launcher

Добавлен executable Stage 9 launcher и safety-test.

Commits:

- `01c5159`
- `ec7fb6f`
- `8ae2eac`

## Реальный AE run #1

Report:

`EGFX-PLANE-94219e58cc354fadae128742a2ccbdb4`

BLOCKED на первом PNG:

`Missing frame d8-original`

Причина: ExtendScript сразу после `saveFrameToPng` видел stale `File` metadata.

Это не plugin FAIL.

Исправление:

- переоткрывать `File(path)`;
- bounded wait до 50 × 100 ms;
- каждый цикл проверять project ownership;
- не перезапускать render;
- не убивать и не перезапускать AE.

Commit:

`724001cc6772d5c4fdc6015a66bb138a3144d8b8`

Gates:

- Final Validation PASS
- PR CI PASS
- macOS source gate `36620529285` PASS

## Реальный AE run #2

Report:

`EGFX-PLANE-814b2dbd8b67486c8f657c0d6ca1855c`

Успешно снято **33/34** кадров:

- вся 8/16/32 bpc matrix;
- оба roundtrip кадра;
- 3D position / scale / rotation / parent;
- camera move;
- camera switch.

BLOCKED только на:

`No-camera state not reached`

Fixture пытался получить no-camera через disable уже созданных cameras.

Это не plugin FAIL.

Исправление:

снимать no-camera до создания Camera A/B.

Commit:

`e6dbc26409644f2202a6349ecf2124ae01f31b8a`

Gates:

- Final Validation PASS
- PR CI PASS
- macOS source gate `36623152320` PASS

## Реальный AE run #3

Report:

`EGFX-PLANE-77f143f3385447ebbb2ce00e80548756`

Успешно снято:

- все 24 depth frames;
- оба roundtrip frames.

Итого до остановки: **26 frames**.

BLOCKED на:

`Unexpected camera before camera-layer creation`

Target AE 25.6 вернул non-null `CompItem.activeCamera` даже при отсутствии camera layer в test composition.

Это не plugin FAIL.

## Последнее исправление no-camera проверки

Убрана зависимость от `activeCamera`.

No-camera теперь доказывается topology test:

```javascript
comp.numLayers === 1
comp.layer(1) === layer
```

То есть до создания камер test comp содержит только test footage layer и ни одного explicit camera layer.

Commit:

`b327964ce218055c7c1dfffe2baabae5e1b18227`

Gates:

- Final Validation PASS
- PR CI PASS
- macOS acceptance-tooling regression PASS
- macOS source gate `36625654300` PASS

## Текущий docs checkpoint

После runtime/tooling фикса сделан только документационный commit:

`0b731f100a383347db09e337482687b0f6759851`

Message:

`docs(stage9): record target active-camera discrepancy`

Для этого состояния:

- CI PASS
- Final Validation PASS
- Authorized update safety PASS
- Live-image diagnostic PASS
- Target AE acceptance package PASS

## Что ещё НЕ закрыто

Stage 9 пока не PASS.

Нужен свежий real-AE rerun уже на topology-based no-camera fixture.

После автоматического PASS отдельно остаются real-host interaction checks:

- native corner drag;
- native guide drag;
- Undo;
- Redo;
- UI save/reopen after real drag.

## Следующий ожидаемый автоматический результат

Успешный Stage 9 target run должен дать:

```text
Stage 9 functional status: PASS
identity_status: PASS
pixel_status: PASS
cleanup_status: CLEAN
fixture_frames: 34
```

После этого можно закрывать автоматическую часть Stage 9 и переходить к native interaction acceptance.

## Release policy

До явной команды пользователя:

- не merge в `main`;
- не создавать production release;
- не делать production deploy.

Stage 10 остаётся финальным compatibility/release gate.
