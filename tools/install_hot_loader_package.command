#!/bin/zsh
set -euo pipefail
setopt null_glob

HERE="${0:A:h}"
BUNDLE="$HERE/ElasticGrid.plugin"

SYSTEM_ROOT="/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore"
USER_ROOT="$HOME/Library/Application Support/Adobe/Common/Plug-ins/7.0/MediaCore"
TARGET="$USER_ROOT/FSTR FX/ElasticGrid.plugin"
STAGED_DIR="$HOME/Library/Application Support/AE Hot Loader/implementations/elasticgrid"

STAMP="$(date +%Y%m%dT%H%M%S)-$$"
USER_BACKUP_ROOT="$HOME/Library/Application Support/AE Hot Loader/backups/elasticgrid-clean/$STAMP"
SYSTEM_BACKUP_ROOT="/Library/Application Support/AE Hot Loader Legacy Backup/elasticgrid/$STAMP"

[[ -d "$BUNDLE" ]] || { echo "ERROR: missing $BUNDLE"; exit 2; }

if pgrep -x "After Effects" >/dev/null 2>&1; then
  echo "ERROR: After Effects is running."
  echo "Fully quit AE before installing the ElasticGrid hot-reload shell."
  exit 3
fi

echo "Validating packaged ElasticGrid build..."
codesign --verify --deep --strict "$BUNDLE"
bundle_archs="$(lipo -archs "$BUNDLE/Contents/MacOS/ElasticGrid" 2>/dev/null || true)"
[[ "$bundle_archs" == *arm64* ]] || { echo "ERROR: packaged ElasticGrid shell is not arm64."; exit 4; }

bundle_id="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$BUNDLE/Contents/Info.plist" 2>/dev/null || true)"
[[ "$bundle_id" == "com.elasticgrid.fx" ]] || {
  echo "ERROR: unexpected packaged ElasticGrid bundle id: $bundle_id"
  exit 4
}

typeset -a search_roots
search_roots=(
  "$SYSTEM_ROOT"
  "$USER_ROOT"
  "/Library/Application Support/Adobe/Plug-Ins/CC"
  "$HOME/Library/Application Support/Adobe/Plug-Ins/CC"
)
for app_plugins in /Applications/Adobe\ After\ Effects*.app/Contents/Plug-ins; do
  [[ -d "$app_plugins" ]] && search_roots+=("$app_plugins")
done

is_inside_search_root() {
  local candidate="$1"
  local root
  for root in "${search_roots[@]}"; do
    case "$candidate" in
      "$root"|"$root"/*) return 0 ;;
    esac
  done
  return 1
}

typeset -a matches
typeset -A match_seen

MATCH_NAME="com.elasticgrid.fx.warp"

bundle_contains_match_name() {
  local bundle="$1"
  local payload

  [[ -d "$bundle/Contents" ]] || return 1

  while IFS= read -r -d '' payload; do
    if LC_ALL=C grep -a -F -q -- "$MATCH_NAME" "$payload" 2>/dev/null; then
      return 0
    fi
  done < <(find "$bundle/Contents" -type f -print0 2>/dev/null)

  return 1
}

record_match() {
  local found="$1"
  [[ -n "$found" ]] || return
  is_inside_search_root "$found" || {
    echo "ERROR: refusing to touch path outside known Adobe plug-in roots:"
    echo "  $found"
    exit 5
  }
  if [[ -z "${match_seen[$found]-}" ]]; then
    match_seen[$found]=1
    matches+=("$found")
  fi
}

scan_elasticgrid_copies() {
  local root found plist found_id
  for root in "${search_roots[@]}"; do
    [[ -d "$root" ]] || continue

    while IFS= read -r found; do
      record_match "$found"
    done < <(find "$root" -type d -name "ElasticGrid.plugin" -prune -print 2>/dev/null)

    while IFS= read -r found; do
      plist="$found/Contents/Info.plist"
      [[ -f "$plist" ]] || continue
      found_id="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist" 2>/dev/null || true)"
      if [[ "$found_id" == "com.elasticgrid.fx" ]] || bundle_contains_match_name "$found"; then
        record_match "$found"
      fi
    done < <(find "$root" -type d -name "*.plugin" -prune -print 2>/dev/null)
  done
}

scan_elasticgrid_copies

typeset -a app_bundle_matches
for found in "${matches[@]}"; do
  case "$found" in
    /Applications/*.app/Contents/Plug-ins/*)
      app_bundle_matches+=("$found")
      ;;
  esac
done

if (( ${#app_bundle_matches[@]} > 0 )); then
  echo "ERROR: ElasticGrid copy found inside a signed After Effects application bundle:"
  for found in "${app_bundle_matches[@]}"; do
    echo "  $found"
  done
  echo "The clean installer will not modify the Adobe app bundle."
  exit 6
fi

mkdir -p "$USER_BACKUP_ROOT/legacy" "$USER_BACKUP_ROOT/state"

needs_sudo=0
for found in "${matches[@]}"; do
  case "$found" in
    /Library/*) needs_sudo=1 ;;
  esac
done

if (( needs_sudo == 1 )); then
  echo
  echo "macOS may ask for your password once to move old system-wide ElasticGrid copies to backup."
  sudo -v
  sudo mkdir -p "$SYSTEM_BACKUP_ROOT"
fi

typeset -a moved_originals
typeset -a moved_backups
typeset -a moved_modes

restore_on_error() {
  local rc=$?
  if (( rc != 0 )); then
    echo
    echo "Install failed; restoring previous ElasticGrid copies..."

    rm -rf "$TARGET" 2>/dev/null || true

    local i original backup mode
    for (( i=${#moved_originals[@]}; i>=1; i-- )); do
      original="${moved_originals[$i]}"
      backup="${moved_backups[$i]}"
      mode="${moved_modes[$i]}"

      if [[ "$mode" == "sudo" ]]; then
        sudo mkdir -p "${original:h}"
        [[ -e "$backup" ]] && sudo mv "$backup" "$original"
      else
        mkdir -p "${original:h}"
        [[ -e "$backup" ]] && mv "$backup" "$original"
      fi
    done
  fi
  exit $rc
}
trap restore_on_error EXIT

if (( ${#matches[@]} > 0 )); then
  echo
  echo "Moving old ElasticGrid copies out of Adobe plug-in folders:"
fi

legacy_index=0
for found in "${matches[@]}"; do
  is_inside_search_root "$found" || {
    echo "ERROR: cleanup safety check failed: $found"
    exit 7
  }

  legacy_index=$((legacy_index + 1))
  base="${found:t}"

  case "$found" in
    /Library/*)
      backup="$SYSTEM_BACKUP_ROOT/${legacy_index}-$base"
      echo "  $found"
      sudo mv "$found" "$backup"
      moved_originals+=("$found")
      moved_backups+=("$backup")
      moved_modes+=("sudo")
      ;;
    *)
      backup="$USER_BACKUP_ROOT/legacy/${legacy_index}-$base"
      echo "  $found"
      mv "$found" "$backup"
      moved_originals+=("$found")
      moved_backups+=("$backup")
      moved_modes+=("user")
      ;;
  esac
done

# Clear only ElasticGrid hot-reload state. Preserve it in the install backup.
state_index=0
for stale in   "$STAGED_DIR/current.dylib"   "$STAGED_DIR/current.tmp.dylib"   /tmp/ae-hot-loader-elasticgrid-shell.log   /tmp/ae-hot-loader-shell-reloader.log   /tmp/ae-hot-loader-agent.log \
  /tmp/elasticgrid-fx.log; do
  if [[ -e "$stale" ]]; then
    state_index=$((state_index + 1))
    mv "$stale" "$USER_BACKUP_ROOT/state/${state_index}-${stale:t}"
  fi
done

# Remove stale ElasticGrid runtime images only; do not touch other shells.
runtime_root="${TMPDIR:-/tmp}/AEHotLoaderShell"
if [[ -d "$runtime_root" ]]; then
  while IFS= read -r stale_runtime; do
    state_index=$((state_index + 1))
    mv "$stale_runtime" "$USER_BACKUP_ROOT/state/${state_index}-${stale_runtime:t}"
  done < <(find "$runtime_root" -type f -name 'elasticgrid-*.dylib' -print 2>/dev/null)
fi

echo
echo "Installing one fresh ElasticGrid shell:"
echo "  $TARGET"
mkdir -p "${TARGET:h}"
cp -R "$BUNDLE" "$TARGET"
xattr -dr com.apple.quarantine "$TARGET" 2>/dev/null || true

codesign --verify --deep --strict "$TARGET"

IDENTITY_FILE="$TARGET/Contents/Resources/BuildIdentity.txt"
[[ -f "$IDENTITY_FILE" ]] || { echo "ERROR: installed BuildIdentity.txt is missing."; exit 8; }
grep -q '^build_id=' "$IDENTITY_FILE" || { echo "ERROR: installed build identity is invalid."; exit 8; }
grep -q '^git_commit=' "$IDENTITY_FILE" || { echo "ERROR: installed git commit identity is missing."; exit 8; }

installed_archs="$(lipo -archs "$TARGET/Contents/MacOS/ElasticGrid" 2>/dev/null || true)"
[[ "$installed_archs" == *arm64* ]] || { echo "ERROR: installed ElasticGrid shell is not arm64."; exit 8; }

installed_id="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$TARGET/Contents/Info.plist" 2>/dev/null || true)"
[[ "$installed_id" == "com.elasticgrid.fx" ]] || {
  echo "ERROR: installed ElasticGrid bundle id mismatch: $installed_id"
  exit 8
}

# Post-install verification: exactly one host-visible ElasticGrid copy must remain,
# including differently named bundles that claim the same AE match name.
matches=()
match_seen=()
scan_elasticgrid_copies

if (( ${#matches[@]} != 1 )) || [[ "${matches[1]-}" != "$TARGET" ]]; then
  echo
  echo "ERROR: clean-install verification failed. Expected exactly:"
  echo "  $TARGET"
  echo "Found:"
  for found in "${matches[@]}"; do
    echo "  $found"
  done
  exit 9
fi

trap - EXIT

echo
echo "CLEAN ELASTICGRID ORIGINAL-PARITY INSTALL COMPLETE"
echo
echo "Installed exactly one active copy:"
echo "  $TARGET"
echo
echo "Build Identity:"
cat "$IDENTITY_FILE"
echo
echo "Previous/test ElasticGrid copies were moved to backup, not deleted."
echo "User backup:"
echo "  $USER_BACKUP_ROOT"
if (( needs_sudo == 1 )); then
  echo "System backup:"
  echo "  $SYSTEM_BACKUP_ROOT"
fi
echo
echo "Next:"
echo "  1. Start After Effects once."
echo "  2. Verify ElasticGrid FX is present and deforms the image."
echo "  3. Keep AE open for STAGE_CANDIDATE.command and Reload Plugins."
echo "See ELASTICGRID_LIVE_TEST.md and ORIGINAL_GRIDWARP_AUDIT.md in this package."
