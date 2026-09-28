#!/bin/bash
# Record ordinary startup failures before Python/imports/host checks can exit.
set -u
umask 077
ROOT="$(cd "$(/usr/bin/dirname "$0")" && pwd)" || exit 2
OUT=""
for base in "$HOME/Desktop" "$HOME/Library/Logs" "$HOME"; do
  if [[ -d "$base" && ! -L "$base" && -w "$base" ]]; then
    OUT="$(/usr/bin/mktemp -d "$base/EGFX-Update.XXXXXX" 2>/dev/null)" && break
  fi
done
if [[ -z "$OUT" ]]; then
  printf 'Не удалось создать журнал. Установка не запускалась.\n' >&2
  exit 2
fi
LOG="$OUT/terminal.log"
finish() {
  status=$?
  printf '\nEXIT_CODE=%s\nЖурнал: %s\n' "$status" "$LOG" | /usr/bin/tee -a "$LOG"
}
trap finish EXIT
printf 'ElasticGridFX: разрешённая тестовая замена с сохранением оригинала.\n' | /usr/bin/tee "$LOG"
if [[ "$(/usr/bin/uname -s)" != Darwin ]]; then
  printf 'Запусти этот файл на своём Mac. Установка не запускалась.\n' | /usr/bin/tee -a "$LOG"
  exit 2
fi
if ! command -v python3 >/dev/null 2>&1; then
  printf 'Python 3 не найден. Установка не запускалась.\n' | /usr/bin/tee -a "$LOG"
  exit 2
fi
printf 'Проверяю процессы. AE и другие программы не будут закрыты автоматически.\n' | /usr/bin/tee -a "$LOG"
export PYTHONDONTWRITEBYTECODE=1
python3 "$ROOT/tools/authorized_update.py" --apply-authorized-replacement "$@" 2>&1 | /usr/bin/tee -a "$LOG"
status=${PIPESTATUS[0]}
exit "$status"
