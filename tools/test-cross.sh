#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"

echo "--- cross-package determinism (10 fields × 20 records) ---"
FIELDS="name email phone city birthdate credit-card ssn passport ip uuid"

CLI_HASH=$("$SF" $FIELDS --seed xpkg --locale en -n 20 --until 2025 --no-header -q 2>/dev/null | shasum -a 256 | cut -d' ' -f1)

NODE_HASH=$(node -e "
  const{SeedFaker}=require('$ROOT/packages/npm');
  const f=new SeedFaker({seed:'xpkg',locale:'en',until:2025});
  const fields='$FIELDS'.split(' ');
  f.records(fields,{n:20}).forEach(r=>console.log(fields.map(k=>r[k]||r[k.replace(/-/g,'_')]).join('\t')));
" | shasum -a 256 | cut -d' ' -f1)

PY_HASH=$(PYTHONPATH="$ROOT/packages/pip" python3 -c "
from seedfaker import SeedFaker
f=SeedFaker(seed='xpkg',locale='en',until=2025)
fields='$FIELDS'.split()
for r in f.records(fields,n=20):
    print('\t'.join(r.get(k,r.get(k.replace('-','_'),'')) for k in fields))
" | shasum -a 256 | cut -d' ' -f1)

if [ "$CLI_HASH" = "$NODE_HASH" ] && [ "$CLI_HASH" = "$PY_HASH" ]; then
  echo "  ok: CLI=npm=pip (sha256=$(printf '%.16s' "$CLI_HASH")...)"
else
  echo "FAIL: CLI=$CLI_HASH npm=$NODE_HASH pip=$PY_HASH"
  exit 1
fi
