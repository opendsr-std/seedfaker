#!/usr/bin/env bash
# sha256 proof: same seed → identical bytes; different seed → different bytes.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

A=$(${SF} name email phone -n 100 --seed demo  --until 2025 | shasum -a 256 | awk '{print $1}')
B=$(${SF} name email phone -n 100 --seed demo  --until 2025 | shasum -a 256 | awk '{print $1}')
C=$(${SF} name email phone -n 100 --seed other --until 2025 | shasum -a 256 | awk '{print $1}')

echo "demo  run 1: $A"
echo "demo  run 2: $B"
echo "other run 3: $C"
[ "$A" = "$B" ]  || { echo "FAIL: same seed gave different bytes"; exit 1; }
[ "$A" != "$C" ] || { echo "FAIL: different seed gave same bytes";  exit 1; }
echo "OK"

# Algorithm fingerprint changes only when generation logic changes — pin in CI.
echo
${SF} --fingerprint
