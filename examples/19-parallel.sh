#!/usr/bin/env bash
# Parallel generation. Two orthogonal mechanisms; they compose.
#
#   --threads N   in-process OS threads, single stdout, serial order
#   --shard I/N   external slice of the serial range, each shard its own process
#
# Both preserve determinism: same seed, row k produces identical bytes in
# every shard/thread configuration.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# 1. In-process threading into one output. Bigger -n = bigger win.
${SF} name email phone -n 100000 --threads 4 --format csv \
  --seed par --until 2025 > /tmp/sf_threads.csv
wc -l /tmp/sf_threads.csv

echo
# 2. External sharding — 3 processes each writing a slice, then concat.
#    Worker 0 keeps the header; later workers use --no-header so the
#    concatenated file has exactly one header line.
${SF} name email phone -n 100000 --shard 0/3 --format csv \
  --seed par --until 2025 > /tmp/sf_shard0.csv
${SF} name email phone -n 100000 --shard 1/3 --format csv --no-header \
  --seed par --until 2025 > /tmp/sf_shard1.csv
${SF} name email phone -n 100000 --shard 2/3 --format csv --no-header \
  --seed par --until 2025 > /tmp/sf_shard2.csv
cat /tmp/sf_shard{0,1,2}.csv > /tmp/sf_shards_concat.csv

# 3. Byte-identical to a single run: sha256 of threaded output == sha256 of
#    concatenated shards == sha256 of a plain single-process run.
SINGLE=$(${SF} name email phone -n 100000 --format csv --seed par --until 2025 | shasum -a 256 | awk '{print $1}')
THREADS=$(shasum -a 256 /tmp/sf_threads.csv | awk '{print $1}')
SHARDS=$(shasum -a 256 /tmp/sf_shards_concat.csv | awk '{print $1}')
echo "single-process:    $SINGLE"
echo "--threads 4:       $THREADS   $([ "$SINGLE" = "$THREADS" ] && echo OK || echo DIFF)"
echo "--shard 0..2/3:    $SHARDS    $([ "$SINGLE" = "$SHARDS" ] && echo OK || echo DIFF)"

rm -f /tmp/sf_threads.csv /tmp/sf_shard{0,1,2}.csv /tmp/sf_shards_concat.csv
