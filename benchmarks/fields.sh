#!/usr/bin/env bash
# Per-field performance benchmark — direct CLI calls, no internal binaries.
# Measures end-to-end throughput: generation + formatting + write to /dev/null.
# Output: benchmarks/results/fields.md
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"
OUT="$DIR/results/fields.md"
N="${BENCH_N_FIELD:-50000}"
RUNS=3

[ -x "$SF" ] || { echo "Binary not found at $SF — run 'make build-cli' first." >&2; exit 1; }

echo "bench-fields: $N records per field, median of $RUNS runs, locale=en" >&2

# Measure median time for a seedfaker command
bench() {
    local times=()
    # warm-up
    "$@" > /dev/null 2>&1 || true
    for _ in $(seq 1 "$RUNS"); do
        t=$(perl -MTime::HiRes=time -e '$s=time; open STDOUT, ">/dev/null"; open STDERR, ">/dev/null"; system(@ARGV); open STDOUT, ">&", 3; printf "%.6f\n", time-$s' -- "$@" 3>&1)
        times+=("$t")
    done
    printf '%s\n' "${times[@]}" | sort -n | sed -n "$((RUNS / 2 + 1))p"
}

# All fields from --list, one per line
FIELDS=$("$SF" --list 2>/dev/null | grep -E '^\s{4}[a-z]' | awk '{print $1}' | grep -v '^enum$')

{
echo "# Per-field Performance"
echo ""
echo "- **Date:** $(date -u +"%Y-%m-%d %H:%M UTC")"
echo "- **OS:** $(uname -s) $(uname -r) $(uname -m)"
echo "- **CPU:** $(sysctl -n machdep.cpu.brand_string 2>/dev/null || grep -m1 'model name' /proc/cpuinfo 2>/dev/null | sed 's/.*: //' || echo 'unknown')"
echo "- **Records:** $N per field"
echo "- **Locale:** en"
echo "- **Method:** median of $RUNS runs (1 warm-up), output to /dev/null"
echo ""
echo "| Field | ops/sec | ns/op |"
echo "|-------|---------|-------|"

total_ops=0
total_ns=0
count=0

while IFS= read -r field; do
    echo "  $field..." >&2
    t=$(bench "$SF" "$field" --locale en --seed bench --until 2025 -n "$N" -q)
    ops=$(awk "BEGIN{if($t>0) printf \"%.0f\", $N/$t; else print 0}")
    ns=$(awk "BEGIN{if($t>0) printf \"%.0f\", $t/$N*1000000000; else print 0}")

    if (( ops >= 1000000 )); then
        ops_str=$(awk -v v="$ops" 'BEGIN{printf "%.1fM/s", v/1000000}')
    else
        ops_str=$(awk -v v="$ops" 'BEGIN{printf "%.0fK/s", v/1000}')
    fi

    printf "| \`%s\` | %s | %s |\n" "$field" "$ops_str" "$ns"
    total_ops=$((total_ops + N))
    total_ns=$((total_ns + ns * N))
    count=$((count + 1))
done <<< "$FIELDS"

} > "$OUT"

avg_ns=$((total_ns / total_ops))
avg_ops=$((1000000000 / avg_ns))
echo "" >&2
echo "$count fields, avg $(awk -v v="$avg_ops" 'BEGIN{printf "%.1fM/s", v/1000000}') ($avg_ns ns/op)" >&2
echo "Written to $OUT" >&2
