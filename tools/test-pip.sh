#!/usr/bin/env bash
set -euo pipefail
echo "--- pip package ---"
PYTHONPATH=packages/pip python3 -c "
from seedfaker import SeedFaker
f=SeedFaker(seed='test-pkg',until=2025)
v=f.field('name'); assert v
n=len(SeedFaker.fields()); assert n>=200, n
fp=SeedFaker.fingerprint(); assert fp.startswith('sf0-'), fp
r=f.records(['name','email'],n=3,ctx='strict'); assert len(r)==3, len(r)
rec=f.record(['name','email'],ctx='strict'); assert 'name' in rec and 'email' in rec, rec
SeedFaker.validate(['name','email','phone:e164'])
u=f.field('name',upper=True); assert u==u.upper(), f'modifier upper: {u}'
xr=f.records(['name:upper'],n=1); assert xr[0]['name_upper']==xr[0]['name_upper'].upper()
print(f'  ok: name={v}, fields={n}, fp={fp}')
"
PYTHONPATH=packages/pip python3 -c "from seedfaker import SeedFaker; SeedFaker.validate(['name:e164'])" 2>&1 \
  && { echo "FAIL: validate should reject name:e164"; exit 1; } \
  || echo "  ok: validate rejects invalid"
