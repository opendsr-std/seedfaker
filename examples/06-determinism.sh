#!/usr/bin/env bash
# Same seed + same --until = byte-identical output across runs and machines.
# sha256sum proves the equality without printing two long outputs.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

A=$(${SF} name email phone -n 100 --seed demo --until 2025 | shasum -a 256 | awk '{print $1}')
B=$(${SF} name email phone -n 100 --seed demo --until 2025 | shasum -a 256 | awk '{print $1}')
C=$(${SF} name email phone -n 100 --seed other --until 2025 | shasum -a 256 | awk '{print $1}')

echo "run 1 (seed=demo):  $A"
echo "run 2 (seed=demo):  $B   $([ "$A" = "$B" ] && echo OK || echo DIFF)"
echo "run 3 (seed=other): $C   $([ "$A" != "$C" ] && echo '(different seed → different bytes)' || echo UNEXPECTED_SAME)"

# The algorithm fingerprint changes only when the generation algorithm changes.
# Pin it in CI to catch drift between seedfaker versions.
echo
${SF} --fingerprint
