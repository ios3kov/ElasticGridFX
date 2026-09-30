#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
if [[ "$(uname -s)" != "Darwin" ]]; then
  echo 'Эта диагностика запускается на Mac. Ничего не изменено.' >&2
  exit 2
fi
if ! command -v python3 >/dev/null 2>&1; then
  echo 'Python 3 не найден. Ничего не установлено; пришли это сообщение в чат.' >&2
  exit 2
fi
echo 'Проверяется загруженная версия ElasticGridFX. Установки и изменений проектов не будет.'
exec python3 "$ROOT/tools/live_identity.py" "$@"
