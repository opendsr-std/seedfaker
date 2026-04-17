#!/usr/bin/env bash
# Quick dev benchmark — regression check during development.
# Median of 5 runs. Warm-up run discarded.
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"
OUT="$DIR/results/fast.md"
N=150000
RUNS=5

[ -x "$SF" ] || { echo "Binary not found at $SF" >&2; exit 1; }

bench() {
    local _label="$1"; shift
    # warm-up (discarded)
    "$@" > /dev/null 2>&1
    local times=()
    for _ in $(seq 1 "$RUNS"); do
        t=$(perl -MTime::HiRes=time -e '$s=time; open STDOUT, ">/dev/null"; open STDERR, ">/dev/null"; system(@ARGV); open STDOUT, ">&", 3; printf "%.3f\n", time-$s' -- "$@" 3>&1)
        times+=("$t")
    done
    # Median: sort and pick middle
    printf '%s\n' "${times[@]}" | sort -n | sed -n "$((RUNS / 2 + 1))p"
}

rps() {
    # Records per second from time
    awk "BEGIN{t=$1; if(t>0) printf \"%.0fK/s\", $N/t/1000; else print \"—\"}"
}

echo "bench-fast: ${N} records, median of ${RUNS} runs" >&2

{
echo "# Quick Benchmark"
echo ""
echo "- **Date:** $(date -u +"%Y-%m-%d %H:%M UTC")"
echo "- **OS:** $(uname -s) $(uname -r) $(uname -m)"
echo "- **CPU:** $(sysctl -n machdep.cpu.brand_string 2>/dev/null || grep -m1 'model name' /proc/cpuinfo 2>/dev/null | sed 's/.*: //' || echo 'unknown')"
echo "- **Binary:** $($SF --version 2>/dev/null)"
echo "- **Records:** ${N}"
echo "- **Method:** median of ${RUNS} runs (1 warm-up discarded)"
echo ""

echo "## Field tiers"
echo ""
echo "| Tier | Fields | Time | Throughput |"
echo "|------|--------|------|------------|"
T3=$(bench "t3" "$SF" name email phone -n "$N" --seed bench --until 2025)
printf "| 3 | name, email, phone | %ss | %s |\n" "$T3" "$(rps "$T3")"
T5=$(bench "t5" "$SF" name email phone city birthdate -n "$N" --seed bench --until 2025)
printf "| 5 | + city, birthdate | %ss | %s |\n" "$T5" "$(rps "$T5")"
# shellcheck disable=SC2086
T10=$(bench "t10" "$SF" name email phone city birthdate country username postal-code ssn credit-card -n "$N" --seed bench --until 2025)
printf "| 10 | + country, username, postal-code, ssn, credit-card | %ss | %s |\n" "$T10" "$(rps "$T10")"
# shellcheck disable=SC2086
T20=$(bench "t20" "$SF" name email phone city birthdate country username postal-code ssn credit-card address company-name job-title iban password ip uuid timestamp passport national-id -n "$N" --seed bench --until 2025)
printf "| 20 | + address, iban, password, ip, uuid, timestamp, ... | %ss | %s |\n" "$T20" "$(rps "$T20")"
echo ""

echo "## Single fields (extremes)"
echo ""
echo "| Field | Time | Throughput | Note |"
echo "|-------|------|------------|------|"
T=$(bench "boolean" "$SF" boolean -n "$N" --seed bench --until 2025)
printf "| boolean | %ss | %s | fastest |\n" "$T" "$(rps "$T")"
T=$(bench "email" "$SF" email -n "$N" --seed bench --until 2025)
printf "| email | %ss | %s | PII, locale-aware |\n" "$T" "$(rps "$T")"
T=$(bench "credit-card" "$SF" credit-card -n "$N" --seed bench --until 2025)
printf "| credit-card | %ss | %s | Luhn checksum |\n" "$T" "$(rps "$T")"
T=$(bench "iban" "$SF" iban -n "$N" --seed bench --until 2025)
printf "| iban | %ss | %s | per-country format |\n" "$T" "$(rps "$T")"
T=$(bench "jwt" "$SF" jwt -n "$N" --seed bench --until 2025)
printf "| jwt | %ss | %s | base64 encoding |\n" "$T" "$(rps "$T")"
T=$(bench "ssh-key" "$SF" ssh-private-key -n "$N" --seed bench --until 2025)
printf "| ssh-private-key | %ss | %s | heaviest |\n" "$T" "$(rps "$T")"
echo ""

echo "## Templates"
echo ""
echo "| Type | Time | Throughput |"
echo "|------|------|------------|"
T=$(bench "tpl3" "$SF" name email phone -t '{{name}} <{{email}}> {{phone}}' -n "$N" --seed bench --until 2025)
printf "| inline (3 fields) | %ss | %s |\n" "$T" "$(rps "$T")"
T=$(bench "nginx" "$SF" run nginx -n "$N" --seed bench --until 2025)
printf "| nginx preset (8 fields, conditionals) | %ss | %s |\n" "$T" "$(rps "$T")"
T=$(bench "chaos" "$SF" run chaos -n "$N" --seed bench --until 2025)
printf "| chaos preset (9 fields, corruption) | %ss | %s |\n" "$T" "$(rps "$T")"
echo ""

echo "## Feature overhead"
echo ""
echo "Baseline: 3 fields (name, email, phone), ${N} records."
echo ""
echo "| Feature | Time | vs baseline |"
echo "|---------|------|-------------|"
T=$(bench "base" "$SF" name email phone -n "$N" --seed bench --until 2025)
printf "| baseline (TSV) | %ss | — |\n" "$T"
BASE="$T"
T2=$(bench "csv" "$SF" name email phone --format csv -n "$N" --seed bench --until 2025)
PCT=$(awk "BEGIN{printf \"+%.0f%%\", ($T2/$BASE - 1)*100}")
printf "| --format csv | %ss | %s |\n" "$T2" "$PCT"
T2=$(bench "ctx" "$SF" name email phone --ctx strict -n "$N" --seed bench --until 2025)
PCT=$(awk "BEGIN{printf \"+%.0f%%\", ($T2/$BASE - 1)*100}")
printf "| --ctx strict | %ss | %s |\n" "$T2" "$PCT"
T2=$(bench "cor" "$SF" name email phone --corrupt high -n "$N" --seed bench --until 2025)
PCT=$(awk "BEGIN{printf \"+%.0f%%\", ($T2/$BASE - 1)*100}")
printf "| --corrupt high | %ss | %s |\n" "$T2" "$PCT"

} > "$OUT"

echo "" >&2
cat "$OUT"
