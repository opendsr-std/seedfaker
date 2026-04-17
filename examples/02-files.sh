#!/usr/bin/env bash
# Write generated records to CSV, JSONL, and SQL-INSERT files.
# Determinism guarantee: same seed + same --until = same bytes on every run.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
OUT=$(mktemp -d) && trap 'rm -rf "$OUT"' EXIT

${SF} name email phone ssn --format csv     -n 1000 --seed files --until 2025 > "$OUT/users.csv"
${SF} name email phone      --format jsonl   -n 500  --seed files --until 2025 > "$OUT/users.jsonl"
${SF} name email phone      --format sql=users -n 100 --seed files --until 2025 > "$OUT/seed.sql"

wc -l "$OUT"/users.csv "$OUT"/users.jsonl "$OUT"/seed.sql
echo
echo "--- users.csv (head) ---"
head -3 "$OUT/users.csv"
echo
echo "--- seed.sql (head) ---"
head -2 "$OUT/seed.sql"
