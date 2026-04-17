#!/usr/bin/env bash
# replace: rewrite PII columns of an existing CSV or JSONL stream while
# keeping every other column intact. Same input + same seed = same output,
# so independently-masked files still join correctly on the replaced values.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
OUT=$(mktemp -d) && trap 'rm -rf "$OUT"' EXIT

${SF} name email phone ssn --format csv -n 5 --seed prod --until 2025 > "$OUT/source.csv"
echo "--- source ---"
cat "$OUT/source.csv"

echo
echo "--- replace email + ssn ---"
${SF} replace email ssn --seed anon < "$OUT/source.csv"

# Determinism: the same (input, seed) pair produces the same masked output
# byte-for-byte, so cross-file joins on email survive anonymisation.
A=$(${SF} replace email ssn --seed anon < "$OUT/source.csv" | shasum -a 256 | awk '{print $1}')
B=$(${SF} replace email ssn --seed anon < "$OUT/source.csv" | shasum -a 256 | awk '{print $1}')
echo
echo "two runs: $A / $B — $([ "$A" = "$B" ] && echo OK || echo DIFF)"
