# ElasticGridFX — финальный gate, 30.09.2026

Обновление 30.09: этап 9 **REOPENED**, этап 10 **BLOCKED**. Пользователь показал
смещение сетки при включении 3D у текстового слоя. Проверка на solid не покрыла
этот случай. Причина пока не доказана; новая сборка не установлена. Нижеследующие
результаты сохранены как исторические, а не подтверждение исправления этого бага.
Это не разрешение на merge/release; `main` не изменяется.

## Кандидат и приёмка

- Plugin commit: `099e49208ebe70a81707dbe905f291e139b9eeb0`.
- Build ID: `EGFX-97a79761c3f9f35751e06853`.
- ZIP SHA-256: `4c48ca09c83d764e36c2640c8aff971c0de616876fde75ee0ab0a9127773ef10`.
- Установленный payload повторно проверен, совпадает с immutable package.
- Между кандидатом и source/evidence checkpoint `f60a109f0846fbdaafdaa70bd74b1ccfc4f1e2e9` нет изменений в `src/` и `host-rust/`.
- 30.09 пользователь ответил «работает» на совместный запрос drag → Undo/Redo и проверки старого проекта с анимацией. Это **USER-REPORTED PASS**, не автоматическое доказательство.
- Этап 8 пропущен пользователем: ускорение/profiling не выполняются и не считаются PASS.

## Матрица доказательств

| Проверка | Результат и границы |
|---|---|
| Целевая среда | macOS 26.6.2, Apple Silicon, AE 2025 25.6.0 |
| C++ регрессия | 18/18 PASS, повторно 30.09 |
| Rust контракты | 24/24 PASS, повторно 30.09 |
| Python tooling | 203/203 PASS, повторно 30.09 |
| Защита test-owned проектов | Node guard/capture/UI/migration mocks PASS |
| Stage 9 пиксели | 40 кадров / 29 проверок PASS; run `EGFX-PLANE-9a2ea8679da941f4855d9ea42bd1be20` |
| 8/16/32 bpc, Full/Half, перспектива | PASS в контролируемом None/non-linearized PNG fixture; это не HDR/OCIO certification |
| Нативная 3D-сетка | Визуально подтверждена на square-pixel слое с камерой/parenting; скриншоты в чате, параметры сцены в `tests/ae_plane_ui_probe.jsx` |
| Углы/панель | Layer Plane/Four Corners, отключение полей, мишени, Render Quality последним — визуальный PASS |
| Старые AEP | Статические значения: автоматический PASS; анимация: USER-REPORTED PASS |
| Drag/Undo/Redo | USER-REPORTED PASS; автоматический mouse drag недоступен |
| Установка/restart | Atomic INSTALLED_FOR_TEST и реальная loaded identity подтверждены; предыдущий кандидат сохранён |
| Статический аудит | REVIEW REQUIRED: прежние workflow pinning/checkout и test auth-heuristic findings; не security PASS |
| CI checkpoint f60a109 | Пять workflows PASS; macOS source gate ещё in_progress при проверке |

## Non-interactive smoke — без performance-claims

Используется неизменённый test-owned AEP `EGFX-perf-160f855b30cc4ba3a4d609139a168e85`,
1920×1080, 32 bpc, Final Bicubic, 60 кадров. Это прежний animated Layer Plane
fixture, а не отдельная сертификация Four Corners в aerender.

Первый OFF-запуск завершился с кодом 0, 60 PNG правильного размера. Основной
identity runner был вызван с ошибочным путём executable и сохранил BLOCKED.
Отдельный capture того же render PID 26296, привязанный к parent PID 26293 и
точному output path, подтвердил UUID `3A8DD976-4BFE-3800-B976-65F075DEBD59`
и installed payload 099e492. Исторический BLOCKED не переписан.

Источник: `work/stage8/EGFX-perf-160f855b30cc4ba3a4d609139a168e85/identity-probe-33ffce5b0a6d40d798012deaffa6ab0b/`,
включая `supplemental-identity.json`. Сводка и новый ON-запуск:
`outputs/stage10/noninteractive-8c3bfffdd52b4fba9d7a67ded219d7cc/`.

Первый ON-вызов не дал кадров из-за относительного output path. AE вернул 0,
но лог содержит «Directory does not exist», и проверка корректно отказала в PASS.
Повтор с абсолютным путём в отдельном `run-2` — PASS: 60 кадров, точный mapped
candidate, output digest совпадает с OFF:
`86e9acf00ad67d4743983335c830e6935100e87bfc65a9194672aac2a68a282d`.
`parity.json` фиксирует одну пару CLI OFF/ON, не доказывает фактическое количество
одновременно исполняемых кадров и не является benchmark. Никакого принудительного
завершения процессов или правки плагина.

## Оставшиеся ограничения финального gate

- Дождаться/проверить macOS source CI для точного checkpoint.
- RAM Preview и отмена рендера в реальном host не получили отдельного инструментированного результата; portable cancellation не заменяет эту проверку.
  Сейчас BLOCKED по защите работы пользователя: в AE открыт `migrated.aep` с
  несохранёнными изменениями после ручной приёмки. Хотя файл из test-owned каталога,
  новые ручные изменения пользователя нельзя считать disposable. Проект не закрывался.
- 3D UI с non-square PAR ограничен текущим прототипом; поддержка не заявляется.
- Windows/Intel, дополнительные версии AE, OCIO/HDR и полный линейный цветовой тракт не сертифицированы.
- GPU dispatch отключён; GPU-производительность и parity не заявляются.
- Подпись ad-hoc подходит текущей тестовой установке; публичная подпись/notarization и выпуск не выполнены и не разрешены.

Не заменять эти ограничения общим «релиз готов». Основная пользовательская
функциональность принята; оставшиеся проверки относятся к финальному gate.
