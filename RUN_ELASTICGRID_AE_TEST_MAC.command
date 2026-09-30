#!/bin/bash
set -u
umask 077
ROOT="$(cd "$(/usr/bin/dirname "$0")" && pwd)" || exit 2
OUT=""
for base in "$HOME/Desktop" "$HOME/Library/Logs" "$HOME"; do
  if [[ -d "$base" && ! -L "$base" && -w "$base" ]]; then
    OUT="$(/usr/bin/mktemp -d "$base/EGFX-AE-Test.XXXXXX" 2>/dev/null)" && break
  fi
done
[[ -n "$OUT" ]] || { echo 'Не удалось создать журнал. Ничего не запускалось.' >&2; exit 2; }
LOG="$OUT/terminal.log"
finish(){ status=$?; printf '\nEXIT_CODE=%s\nЖурнал: %s\n' "$status" "$LOG" | /usr/bin/tee -a "$LOG"; }
trap finish EXIT
printf 'ElasticGridFX: проверка реально загруженной fd69988 и Corner Pin.\n' | /usr/bin/tee "$LOG"
[[ "$(/usr/bin/uname -s)" == Darwin ]] || { echo 'Запусти на Mac.' | /usr/bin/tee -a "$LOG"; exit 2; }
command -v python3 >/dev/null 2>&1 || { echo 'Python 3 не найден. Ничего не изменено.' | /usr/bin/tee -a "$LOG"; exit 2; }
printf 'Требуется один открытый After Effects 2025 с НОВЫМ ПУСТЫМ несохранённым проектом.\n' | /usr/bin/tee -a "$LOG"
printf 'Скрипт ничего не устанавливает, не закрывает AE и не меняет preferences.\n' | /usr/bin/tee -a "$LOG"
export PYTHONDONTWRITEBYTECODE=1
python3 "$ROOT/tools/target_ae_acceptance.py" "$@" 2>&1 | /usr/bin/tee -a "$LOG"
status=${PIPESTATUS[0]}
exit "$status"
