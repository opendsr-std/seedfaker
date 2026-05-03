#!/usr/bin/env bash
# Formats: tsv, csv, jsonl, sql=TABLE, -t inline template. Same 3 records.
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
