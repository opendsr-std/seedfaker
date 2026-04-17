#!/usr/bin/env bash
# Unix composition: seedfaker writes to stdout, Unix tools take over.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# 1. Aggregate by HTTP status from synthetic nginx logs.
${SF} run nginx -n 1000 --seed demo --until 2025 \
  | grep -oE 'HTTP/1\.1" [0-9]+' \
  | sort | uniq -c | sort -rn | head -5

echo
# 2. Anonymise a CSV on its way through a pipeline.
printf 'name,email,phone\nAlice,alice@corp.com,555-1234\nBob,bob@corp.com,555-5678\n' \
  | ${SF} replace email phone --seed anon

echo
# 3. Sort JSONL by email, keep first 3.
${SF} name email --format jsonl -n 10 --seed demo --until 2025 \
  | sort -t\" -k4 \
  | head -3
