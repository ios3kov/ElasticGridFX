#!/usr/bin/env python3
import pathlib
import sys

if len(sys.argv) != 2:
    raise SystemExit("usage: verify_clean_installer.py INSTALL.command")

text = pathlib.Path(sys.argv[1]).read_text()

required = [
    'MATCH_NAME="com.elasticgrid.fx.warp"',
    "bundle_contains_match_name()",
    'grep -a -F -q -- "$MATCH_NAME"',
    'find "$bundle/Contents" -type f -print0',
    'found_id" == "com.elasticgrid.fx"',
    'bundle_contains_match_name "$found"',
    'find "$root" -type d -name "*.plugin"',
    'if (( ${#matches[@]} != 1 ))',
    'pgrep -x "After Effects"',
]
for item in required:
    if item not in text:
        raise SystemExit(f"clean installer contract missing: {item}")

print("clean installer contract: PASS filename + bundle-id + AE match-name duplicate scan")
