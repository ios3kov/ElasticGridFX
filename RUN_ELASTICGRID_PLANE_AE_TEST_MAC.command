#!/bin/bash
set -u
umask 077
ROOT="$(cd "$(/usr/bin/dirname "$0")" && pwd)" || exit 2
OUT=""
for base in "$HOME/Desktop" "$HOME/Library/Logs" "$HOME"; do
  if [[ -d "$base" && ! -L "$base" && -w "$base" ]]; then
    OUT="$(/usr/bin/mktemp -d "$base/EGFX-Plane-Test.XXXXXX" 2>/dev/null)" && break
  fi
done
[[ -n "$OUT" ]] || { echo 'Не удалось создать журнал. Ничего не запускалось.' >&2; exit 2; }
LOG="$OUT/terminal.log"
finish(){ status=$?; printf '\nEXIT_CODE=%s\nЖурнал: %s\n' "$status" "$LOG" | /usr/bin/tee -a "$LOG"; }
trap finish EXIT

printf 'ElasticGridFX Stage 9: de31498 plane / 3D / camera acceptance.\n' | /usr/bin/tee "$LOG"
[[ "$(/usr/bin/uname -s)" == Darwin ]] || { echo 'Запусти на Mac.' | /usr/bin/tee -a "$LOG"; exit 2; }
[[ "$(/usr/bin/uname -m)" == arm64 ]] || { echo 'Нужен Apple Silicon Mac.' | /usr/bin/tee -a "$LOG"; exit 2; }
command -v python3 >/dev/null 2>&1 || { echo 'Python 3 не найден. Ничего не изменено.' | /usr/bin/tee -a "$LOG"; exit 2; }

MANIFEST="$ROOT/work/plane-candidate-de31498/ElasticGrid.artifact.json"
PACKAGE="$ROOT/work/plane-candidate-de31498/ElasticGrid.plugin.zip"
[[ -f "$MANIFEST" && ! -L "$MANIFEST" ]] || { echo 'Не найден manifest точного de31498. Ничего не запускалось.' | /usr/bin/tee -a "$LOG"; exit 3; }
[[ -f "$PACKAGE" && ! -L "$PACKAGE" ]] || { echo 'Не найден package точного de31498. Ничего не запускалось.' | /usr/bin/tee -a "$LOG"; exit 3; }

printf 'Требуется ровно один открытый After Effects 2025 с НОВЫМ ПУСТЫМ несохранённым проектом.\n' | /usr/bin/tee -a "$LOG"
printf 'Скрипт не устанавливает плагин, не убивает процессы и не трогает пользовательские проекты.\n' | /usr/bin/tee -a "$LOG"
printf 'Он создаёт только свой тестовый проект, проверяет de31498 и затем закрывает только этот тестовый проект.\n' | /usr/bin/tee -a "$LOG"

export PYTHONDONTWRITEBYTECODE=1
python3 "$ROOT/tools/ae_plane_acceptance.py"   --report-root "$OUT/reports"   --manifest "$MANIFEST"   --package "$PACKAGE"   "$@" 2>&1 | /usr/bin/tee -a "$LOG"
status=${PIPESTATUS[0]}
exit "$status"
