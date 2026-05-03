#!/usr/bin/env bash
# Write to CSV, JSONL, SQL-INSERT files.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
OUT=$(mktemp -d) && trap 'rm -rf "$OUT"' EXIT

${SF} name email phone ssn --format csv     -n 1000 --seed files --until 2025 > "$OUT/users.csv"
${SF} name email phone      --format jsonl   -n 500  --seed files --until 2025 > "$OUT/users.jsonl"
${SF} name email phone      --format sql=users -n 100 --seed files --until 2025 > "$OUT/seed.sql"

wc -l "$OUT"/users.csv "$OUT"/users.jsonl "$OUT"/seed.sql
echo
echo "--- users.csv ---"
head -3 "$OUT/users.csv"
echo
echo "--- seed.sql ---"
head -2 "$OUT/seed.sql"

# assert: row counts match -n
[ "$(wc -l < "$OUT/users.csv")"    -eq 1001 ] || { echo "csv: row count mismatch"; exit 1; }
[ "$(wc -l < "$OUT/users.jsonl")"  -eq 500  ] || { echo "jsonl: row count mismatch"; exit 1; }
[ "$(wc -l < "$OUT/seed.sql")"     -eq 100  ] || { echo "sql: row count mismatch"; exit 1; }
