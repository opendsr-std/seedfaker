#!/usr/bin/env bash
# Output formats: tsv (default), csv, jsonl, sql=TABLE, inline template (-t).
# Same 3 records in each format so you can compare shapes.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
COMMON="name email phone -n 3 --seed demo --until 2025"

echo "--- tsv (default) ---"
${SF} ${COMMON}

echo
echo "--- csv ---"
${SF} ${COMMON} --format csv

echo
echo "--- jsonl ---"
${SF} ${COMMON} --format jsonl

echo
echo "--- sql=users ---"
${SF} ${COMMON} --format sql=users

echo
echo "--- inline template (-t) ---"
${SF} name email -n 3 --seed demo --until 2025 \
  -t '<user name="{{name}}" contact="{{email}}"/>'
