#!/bin/bash
# Read-only precheck: never run/import the updater and never touch plugin state.
set -u
umask 077
SELF="$(cd "$(/usr/bin/dirname "$0")" && pwd)/$(/usr/bin/basename "$0")"
OUT=""
for base in "$HOME/Desktop" "$HOME/Library/Logs" "$HOME"; do
  if [[ -d "$base" && ! -L "$base" && -w "$base" ]]; then
    OUT="$(/usr/bin/mktemp -d "$base/EGFX-StartCheck.XXXXXX" 2>/dev/null)" && break
  fi
done
if [[ -z "$OUT" ]]; then
  printf 'Не удалось создать папку отчёта. Плагин не изменялся. Пришли это сообщение.\n' >&2
  exit 2
fi
LOG="$OUT/report.txt"
printf 'ElasticGridFX: проверка запуска, БЕЗ УСТАНОВКИ.\nНовый отчёт создан до запуска Python.\n' > "$LOG"
if command -v python3 >/dev/null 2>&1; then
  PYTHONDONTWRITEBYTECODE=1 python3 - "$SELF" "$@" >> "$LOG" 2>&1 <<'EGFX_PY'
import hashlib
import json
import os
from pathlib import Path
import platform
import subprocess
import sys

IDENTITY_SHA = '2b634ac2c289981b013179f30255b926d3c5d3101eed32d6f51ddb18b4849b27'
OLD_SHA = '4d21118301725178fbc6ba3ecea5e4ed053c9275accea10b6db0f51e3c657acf'
NEW_SHA = '4958d73bff702cbf21fa46d1aee670fa524cc99c57f2423c62a7d0cae32ebae7'
TARGET = 'Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore/FSTR FX/ElasticGrid.plugin'
MAX_FILE = 32 * 1024 * 1024
NEEDLES = ('after effects', 'aerender', 'premiere pro', 'adobe media encoder', 'dynamiclinkmanager')


def read_safe(path, limit=MAX_FILE):
    path = Path(path).absolute()
    for p in (path, *path.parents):
        if p.is_symlink():
            raise ValueError('SYMLINK_REFUSED')
    if not path.is_file() or path.stat().st_size > limit:
        raise ValueError('MISSING_OR_OVERSIZED_FILE: ' + path.name)
    with path.open('rb') as handle:
        data = handle.read(limit + 1)
    if len(data) > limit:
        raise ValueError('FILE_GREW_BEYOND_LIMIT')
    return data


def sha(data):
    return hashlib.sha256(data).hexdigest()


def inspect_package(root):
    result = {'path': str(root), 'issues': []}
    try:
        raw = read_safe(root / 'InstallToolIdentity.json', 16384)
        if sha(raw) != IDENTITY_SHA:
            raise ValueError('NOT_THE_DELIVERED_923f806_INSTALLER')
        meta = json.loads(raw)
        result['commit'] = meta['commit']
        for name, expected in meta['files'].items():
            p = root / name
            try:
                data = read_safe(p)
                if sha(data) != expected['sha256']:
                    result['issues'].append('CHANGED_BYTES: ' + name)
                if bool(p.stat().st_mode & 0o111) != expected['executable']:
                    result['issues'].append('CHANGED_EXECUTE_BIT: ' + name)
                if name.endswith('.py'):
                    compile(data, name, 'exec')  # Parse only; never execute imports/code.
            except (OSError, ValueError, SyntaxError) as error:
                result['issues'].append(type(error).__name__ + ': ' + str(error))
    except (OSError, ValueError, KeyError) as error:
        result['issues'].append(type(error).__name__ + ': ' + str(error))
    return result


def locate(roots, limit=400):
    # Only shallow explicit roots. Never traverse a .app/.plugin or symlink.
    found, errors, seen = [], [], set()
    pending = [(Path(p).absolute(), 0) for p in roots]
    while pending and len(seen) < limit and len(found) < 8:
        p, depth = pending.pop(0)
        if p in seen:
            continue
        seen.add(p)
        if p.is_symlink() or not p.is_dir():
            continue
        if (p / 'InstallToolIdentity.json').is_file():
            found.append(p)
            continue
        if depth == 3:
            continue
        try:
            with os.scandir(p) as entries:
                for i, entry in enumerate(entries):
                    if i >= 2000:
                        errors.append('DIRECTORY_ENTRY_LIMIT: ' + str(p)); break
                    if (not entry.name.startswith('.') and not entry.name.endswith(('.app', '.plugin'))
                            and not entry.name.startswith('EGFX-') and entry.is_dir(follow_symlinks=False)):
                        pending.append((Path(entry.path), depth + 1))
                        if len(pending) >= limit:
                            errors.append('SEARCH_QUEUE_LIMIT'); break
        except OSError as error:
            errors.append(type(error).__name__ + ': ' + str(p))
    if pending:
        errors.append('SEARCH_LIMIT: pass the updater file as an explicit argument')
    return found, errors


def hosts(text):
    result = []
    for line in text.splitlines():
        pair = line.strip().split(None, 1)
        if len(pair) == 2 and pair[0].isdigit() and any(n in pair[1].lower() for n in NEEDLES):
            result.append({'pid': int(pair[0]), 'executable': pair[1]})
    return result


def run(self_path, explicit=None):
    home = Path.home()
    result = {'schema': 1, 'tool_sha256': sha(read_safe(self_path)),
              'scope': 'current read-only startup checks; not a reconstruction of the previous launch',
              'installation_attempted': False, 'historical_failure_reason': 'UNKNOWN',
              'python': sys.version.split()[0], 'python_executable': sys.executable,
              'os': platform.system(), 'architecture': platform.machine(), 'blockers': []}
    if platform.system() != 'Darwin':
        result['blockers'].append('MACOS_REQUIRED')
    if platform.machine() != 'arm64':
        result['blockers'].append('ARM64_REQUIRED_FOR_THIS_CANDIDATE')
    if sys.version_info < (3, 9):
        result['blockers'].append('UPDATER_REQUIRES_PYTHON_3_9_OR_NEWER')
    if os.geteuid() == 0:
        result['blockers'].append('DO_NOT_RUN_INSTALLER_WITH_SUDO')
    try:
        text = subprocess.check_output(['/bin/ps', '-axo', 'pid=,comm='], text=True, timeout=5)
        result['adobe_processes'] = hosts(text)
        if result['adobe_processes']:
            result['blockers'].append('ADOBE_HOST_OR_RENDER_HELPER_IS_STILL_RUNNING')
    except (OSError, subprocess.SubprocessError) as error:
        result['blockers'].append('PROCESS_CHECK_FAILED: ' + type(error).__name__)
    binary = home / TARGET / 'Contents/MacOS/ElasticGrid'
    try:
        value = sha(read_safe(binary))
        result['installed_binary_sha256'] = value
        result['installed_state'] = 'OLD' if value == OLD_SHA else 'CANDIDATE' if value == NEW_SHA else 'OTHER'
        result['plugin_parent_writable'] = os.access(binary.parents[2], os.W_OK)
    except (OSError, ValueError) as error:
        result['blockers'].append('TARGET_READ_FAILED: ' + str(error))
    result['backup_directory_exists'] = (home / 'Library/Application Support/ElasticGridFX/Test Backups').exists()
    if explicit:
        item = Path(explicit).expanduser().absolute()
        roots = [item.parent if item.name == 'UPDATE_ELASTICGRID_MAC.command' else item]
    else:
        roots = [Path(self_path).parent, home / 'Downloads', home / 'Desktop']
    found, errors = locate(roots)
    result['search_warnings'] = errors
    result['installers'] = [inspect_package(p) for p in found]
    if not found:
        result['blockers'].append('UNPACKED_INSTALLER_NOT_FOUND_IN_SEARCH_SCOPE')
    elif not any(not p['issues'] for p in result['installers']):
        result['blockers'].append('DELIVERED_INSTALLER_INTEGRITY_OR_SYNTAX_FAILED')
    result['status'] = 'BLOCKED' if result['blockers'] else 'NO_CURRENT_EARLY_BLOCKER_FOUND'
    result['note_ru'] = 'Установщик не запускался. Плагин и резервные копии не менялись. Причина прежнего запуска без его вывода неизвестна.'
    return result


if __name__ == '__main__':
    result = run(Path(sys.argv[1]), sys.argv[2] if len(sys.argv) > 2 else None)
    print(json.dumps(result, ensure_ascii=False, indent=2).replace(str(Path.home()), '~'))
EGFX_PY
  status=$?
  printf '\nPYTHON_EXIT_CODE=%s\n' "$status" >> "$LOG"
else
  printf 'PYTHON_NOT_FOUND: Python 3 не найден в PATH. Установщик не запускался.\n' >> "$LOG"
  status=2
fi
/bin/cat "$LOG"
printf '\nПришли файл report.txt отсюда: %s\n' "$OUT"
# Finder reveal only for the user's interactive terminal; never launch Adobe.
if [[ -t 1 && "$(/usr/bin/uname -s)" == Darwin ]]; then
  /usr/bin/open -R "$LOG" >/dev/null 2>&1 || true
fi
exit "$status"
