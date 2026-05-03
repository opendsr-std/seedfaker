#!/usr/bin/env bash
# replace: rewrite named columns of a CSV/JSONL stream, keep others intact.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
OUT=$(mktemp -d) && trap 'rm -rf "$OUT"' EXIT

${SF} name email phone ssn --format csv -n 5 --seed prod --until 2025 > "$OUT/source.csv"
echo "--- source ---"
cat "$OUT/source.csv"

echo
echo "--- replace email + ssn ---"
${SF} replace email ssn --seed anon < "$OUT/source.csv"

# Same (input, seed) → same masked output → cross-file joins on email survive.
A=$(${SF} replace email ssn --seed anon < "$OUT/source.csv" | shasum -a 256 | awk '{print $1}')
B=$(${SF} replace email ssn --seed anon < "$OUT/source.csv" | shasum -a 256 | awk '{print $1}')
echo
echo "two runs: $A / $B"
[ "$A" = "$B" ] || { echo "replace not deterministic"; exit 1; }
echo "OK"
