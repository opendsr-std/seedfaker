#!/usr/bin/env bash
# Run all benchmarks: CLI tiers + per-field + comparisons.
# Prerequisites: benchmarks/install.sh
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"

echo "=== fast (CLI tiers) ===" >&2
bash "$DIR/fast.sh"

echo "" >&2
echo "=== fields (per-field perf) ===" >&2
bash "$DIR/fields.sh"

echo "" >&2
echo "=== comparisons (vs competitors) ===" >&2
bash "$DIR/compare.sh"

echo "" >&2
echo "All benchmarks complete. Results in benchmarks/results/" >&2
