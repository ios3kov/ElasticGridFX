#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")" && pwd)"
[[ "$(uname -s)" == Darwin ]] || { echo 'Запусти этот файл на своём Mac.' >&2; exit 2; }
command -v python3 >/dev/null || { echo 'Python 3 не найден. Ничего не изменено.' >&2; exit 2; }
echo 'Закрой After Effects и другие Adobe-программы. Они не будут закрыты автоматически.'
echo 'Заменяется только разрешённая копия ElasticGrid. Исходная папка сохраняется для отката.'
export PYTHONDONTWRITEBYTECODE=1
exec python3 "$ROOT/tools/authorized_update.py" --apply-authorized-replacement "$@"
