#!/usr/bin/env bash
# -n 0 = unlimited stream. --rate N = cap N/sec.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# Unlimited stream, take 5.
${SF} email -n 0 --seed stream --until 2025 | head -5

echo
# 5 records at 2/s → ~2.5s.
time ${SF} email -n 5 --rate 2 --seed rate --until 2025

echo
# uniqueness sanity: 10k emails ≥ 9990 distinct (entropy floor).
UNIQ=$(${SF} email -n 10000 --seed uniq --until 2025 | sort -u | wc -l | tr -d ' ')
echo "unique: $UNIQ / 10000"
[ "$UNIQ" -ge 9990 ] || { echo "FAIL: low uniqueness"; exit 1; }
