#!/usr/bin/env bash
#   --threads N   in-process threads, single stdout, serial order
#   --shard I/N   external slice, each shard its own process
# Same seed: row k → identical bytes in every config.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
T=$(mktemp -d) && trap 'rm -rf "$T"' EXIT

${SF} name email phone -n 100000 --threads 4 --format csv \
  --seed par --until 2025 > "$T/threads.csv"
wc -l "$T/threads.csv"

# Worker 0 keeps header, later workers --no-header → exactly one header after concat.
${SF} name email phone -n 100000 --shard 0/3 --format csv \
  --seed par --until 2025 > "$T/shard0.csv"
${SF} name email phone -n 100000 --shard 1/3 --format csv --no-header \
  --seed par --until 2025 > "$T/shard1.csv"
${SF} name email phone -n 100000 --shard 2/3 --format csv --no-header \
  --seed par --until 2025 > "$T/shard2.csv"
cat "$T"/shard{0,1,2}.csv > "$T/shards.csv"

SINGLE=$(${SF} name email phone -n 100000 --format csv --seed par --until 2025 | shasum -a 256 | awk '{print $1}')
THREADS=$(shasum -a 256 "$T/threads.csv" | awk '{print $1}')
SHARDS=$(shasum -a 256 "$T/shards.csv"  | awk '{print $1}')
echo
echo "single:     $SINGLE"
echo "threads 4:  $THREADS"
echo "shards 3:   $SHARDS"
[ "$SINGLE" = "$THREADS" ] || { echo "FAIL: threads != single"; exit 1; }
[ "$SINGLE" = "$SHARDS"  ] || { echo "FAIL: shards != single";  exit 1; }
echo "OK"
