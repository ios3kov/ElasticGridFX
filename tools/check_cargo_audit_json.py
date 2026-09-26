#!/usr/bin/env python3
import json,sys
from pathlib import Path
try: data=json.loads(Path(sys.argv[1]).read_text())
except Exception as e: print(f'ERROR: cannot parse cargo-audit JSON: {e}',file=sys.stderr); raise SystemExit(2)
v=data.get('vulnerabilities') or {}; count=int(v.get('count') or len(v.get('list') or []))
w=data.get('warnings') or {}; wc=sum(len(v) for v in w.values() if isinstance(v,list)) if isinstance(w,dict) else 0
print(f'RustSec vulnerabilities: {count}; informational/warning entries: {wc}')
if count: print('ERROR: RustSec vulnerability gate failed.',file=sys.stderr); raise SystemExit(1)
