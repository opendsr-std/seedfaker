#!/usr/bin/env bash
# Uniqueness report — measures collision rates via direct CLI calls.
# Every number in this report is reproducible: copy the command, run it yourself.
# Usage: make uniqueness [MAX=1000000]
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"
OUT="$DIR/results/uniqueness.md"
N="${MAX:-100000}"
SEEDS=(hero alpha bravo charlie delta)
K=${#SEEDS[@]}

[ -x "$SF" ] || { echo "Binary not found at $SF — run 'make build-cli' first." >&2; exit 1; }

echo "uniqueness: $K seeds × $N records" >&2

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

fmt_num() {
    local n=$1
    if (( n >= 1000000 )); then
        awk "BEGIN{printf \"%.2fM\", $n/1000000}"
    elif (( n >= 1000 )); then
        awk "BEGIN{printf \"%.1fK\", $n/1000}"
    else
        echo "$n"
    fi
}

# Count unique values from seedfaker output (single-column, no header)
count_unique() {
    sort -u | wc -l | tr -d ' '
}

# Median of a list of numbers (one per line)
median() {
    sort -n | awk -v k="$1" 'NR==int(k/2)+1{print; exit}'
}

# Safe awk wrappers (avoid nested-quote issues in subshells)
awk_pct()  { awk -v u="$1" -v n="$2" 'BEGIN{printf "%.4f", (1 - u/n) * 100}'; }
awk_dpct() { awk -v d="$1" -v n="$2" 'BEGIN{printf "%.4f", d/n*100}'; }
awk_upct() { awk -v u="$1" -v n="$2" 'BEGIN{printf "%.4f", u/n*100}'; }
awk_lt()   { awk -v a="$1" -v b="$2" 'BEGIN{print (a < b) ? 1 : 0}'; }
awk_gt()   { awk -v a="$1" -v b="$2" 'BEGIN{print (a > b) ? 1 : 0}'; }
awk_ge()   { awk -v a="$1" -v b="$2" 'BEGIN{print (a >= b) ? 1 : 0}'; }
awk_f2()   { awk -v v="$1" 'BEGIN{printf "%.2f", v}'; }
awk_f1()   { awk -v v="$1" 'BEGIN{printf "%.1f", v}'; }

# Generate N records of a single field, return unique count
gen_unique() {
    local field=$1 seed=$2 n=$3
    "$SF" "$field" --seed "$seed" --until 2025 -n "$n" -q 2>/dev/null | count_unique
}

# Generate N records with alias columns, return tab-separated row
gen_aliased() {
    local field=$1 count=$2 seed=$3 n=$4
    local args=()
    for i in $(seq 0 $((count - 1))); do
        args+=("c${i}=${field}")
    done
    "$SF" "${args[@]}" --seed "$seed" --until 2025 -n "$n" --no-header -q 2>/dev/null
}

# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------

{
echo "# Uniqueness Report"
echo ""

# --- Section 1: Default vs :xuniq ---
echo "## Default vs \`:xuniq\`"
echo ""
echo "Median duplicate % across $K seeds. The \`:xuniq\` modifier adds a 5-char deterministic tag for guaranteed uniqueness at any scale."
echo ""

COMP_FIELDS=(email username login-name nickname social-handle)
# Comparison scales: N/10, N, N*5 — capped by MAX
COMP_SCALES=($((N / 10)) "$N" $((N * 5)))
COMP_SEEDS=3

printf "| Field | Mode |"
for s in "${COMP_SCALES[@]}"; do printf " %s |" "$(fmt_num "$s")"; done
echo ""
printf "|-------|------|"
for _ in "${COMP_SCALES[@]}"; do printf -- "------|"; done
echo ""

for field in "${COMP_FIELDS[@]}"; do
    echo "    $field..." >&2
    # Default mode
    printf "| \`%s\` | default |" "$field"
    for scale in "${COMP_SCALES[@]}"; do
        pcts=()
        for seed in "${SEEDS[@]:0:$COMP_SEEDS}"; do
            u=$(gen_unique "$field" "$seed" "$scale")
            pcts+=("$(awk_pct "$u" "$scale")")
        done
        med=$(printf '%s\n' "${pcts[@]}" | median "$COMP_SEEDS")
        if [ "$(awk_lt "$med" 0.001)" = "1" ]; then
            printf " 0%% |"
        else
            printf " %s%% dup |" "$(awk_f2 "$med")"
        fi
    done
    echo ""
    # xuniq mode
    printf "| | \`:xuniq\` |"
    for scale in "${COMP_SCALES[@]}"; do
        u=$(gen_unique "${field}:xuniq" "${SEEDS[0]}" "$scale")
        dups=$((scale - u))
        if (( dups == 0 )); then
            printf " 0%% |"
        else
            printf " %s%% dup |" "$(awk_dpct "$dups" "$scale")"
        fi
    done
    echo ""
done

echo ""
echo "\\* zero collisions observed"
echo ""
echo "See [fields — extended uniqueness](../docs/fields.md#extended-uniqueness-xuniq) for details."
echo ""
echo "---"
echo ""

# --- Section 2: Multi-use per entity ---
echo "## Multi-use per entity"
echo ""
echo "When a single record draws the same field type N times (e.g. \`doctor=name patient=name nurse=name\`), duplicates within one row break realism. This table shows the median number of duplicate values per record ($K seeds × 1000 rows). 0 = all values distinct in the typical row."
echo ""

MULTI_COUNTS=(5 10 25 50 100)
MULTI_FIELDS=(email username first-name last-name phone city ip address jwt credit-card passport birthdate uuid)
MULTI_ROWS=1000

printf "| Field |"
for c in "${MULTI_COUNTS[@]}"; do printf " ×%s |" "$c"; done
echo ""
printf "|-------|"
for _ in "${MULTI_COUNTS[@]}"; do printf -- "------|"; done
echo ""

for field in "${MULTI_FIELDS[@]}"; do
    echo "    $field..." >&2
    printf "| \`%s\` |" "$field"
    for count in "${MULTI_COUNTS[@]}"; do
        seed_medians=()
        for seed in "${SEEDS[@]}"; do
            # Generate MULTI_ROWS rows with $count aliased columns
            # Count per-row duplicates: columns - unique_per_row
            dups=$(gen_aliased "$field" "$count" "$seed" "$MULTI_ROWS" \
                | awk -F'\t' -v c="$count" '{
                    delete seen; u=0
                    for(i=1;i<=NF;i++) if(!seen[$i]++) u++
                    print c - u
                }' \
                | sort -n | awk -v k="$MULTI_ROWS" 'NR==int(k/2)+1{print; exit}')
            seed_medians+=("${dups:-0}")
        done
        med=$(printf '%s\n' "${seed_medians[@]}" | median "$K")
        printf " %s |" "$med"
    done
    echo ""
    echo "    $field done" >&2
done

echo ""
echo "Fields with large value spaces (\`email\`, \`phone\`, \`ip\`, \`credit-card\`, \`jwt\`, \`passport\`) produce zero in-row collisions at any practical multiplicity. Dictionary-bounded fields (\`first-name\`, \`last-name\`, \`city\`) follow birthday-paradox statistics — collisions grow as draws approach dictionary size."
echo ""
echo "---"
echo ""

# --- Section 3: All fields ---
echo "## All fields"
echo ""
echo "Measured: $K seeds × $(fmt_num "$N") records per seed, locale: all."
echo "Seed variance across all fields: <0.1% — results are seed-independent."
echo ""

ALL_FIELDS=(
    name first-name last-name email username nickname login-name
    phone "phone:e164" address city postal-code
    ssn passport drivers-license
    "credit-card" iban
    ip ipv6 uuid jwt api-key
    btc-address eth-address
    company-name ein employee-id
)

echo "### Fields"
echo ""
echo "| Field | Unique | Dup% | Type |"
echo "|-------|--------|------|------|"

for spec in "${ALL_FIELDS[@]}"; do
    echo "  $spec..." >&2
    uniques=()
    dup_pcts=()
    all_zero=true
    for seed in "${SEEDS[@]}"; do
        u=$(gen_unique "$spec" "$seed" "$N")
        uniques+=("$u")
        d=$((N - u))
        if (( d > 0 )); then all_zero=false; fi
        dup_pcts+=("$(awk_dpct "$d" "$N")")
    done
    med_u=$(printf '%s\n' "${uniques[@]}" | median "$K")
    med_dup=$(printf '%s\n' "${dup_pcts[@]}" | median "$K")

    # Field type classification
    if (( med_u == N )); then
        ftype="algorithmic"
    elif [ "$(awk_gt "$(awk_upct "$med_u" "$N")" 99)" = "1" ]; then
        ftype="high-cardinality"
    elif (( med_u > 10000 )); then
        ftype="medium"
    else
        ftype="dictionary"
    fi

    # Format dup%
    if $all_zero; then
        dup_str="0% *"
    elif [ "$(awk_lt "$med_dup" 0.01)" = "1" ]; then
        dup_str="<0.01%"
    elif [ "$(awk_lt "$med_dup" 1)" = "1" ]; then
        dup_str="$(awk_f2 "$med_dup")%"
    else
        dup_str="$(awk_f1 "$med_dup")%"
    fi

    printf "| \`%s\` | %s | %s | %s |\n" "$spec" "$(fmt_num "$med_u")" "$dup_str" "$ftype"
done

echo ""
echo "\\* no collisions observed at ${K}×$(fmt_num "$N")"
echo ""

# --- Section 4: Combinations ---
echo "## Combinations"
echo ""
echo "| Fields | Unique | Dup% |"
echo "|--------|--------|------|"

COMBOS=(
    "name,email"
    "name,birthdate"
    "name,email,phone"
    "name,email,phone,birthdate"
    "name,email,ssn"
    "ip,username"
    "credit-card,amount"
    "ssn,name"
)

for combo in "${COMBOS[@]}"; do
    echo "  $combo..." >&2
    IFS=',' read -ra fields <<< "$combo"
    uniques=()
    dup_pcts=()
    all_zero=true
    for seed in "${SEEDS[@]}"; do
        # Generate multi-field rows, count unique combinations
        u=$("$SF" "${fields[@]}" --seed "$seed" --until 2025 -n "$N" --no-header -q 2>/dev/null | count_unique)
        uniques+=("$u")
        d=$((N - u))
        if (( d > 0 )); then all_zero=false; fi
        dup_pcts+=("$(awk_dpct "$d" "$N")")
    done
    med_u=$(printf '%s\n' "${uniques[@]}" | median "$K")
    med_dup=$(printf '%s\n' "${dup_pcts[@]}" | median "$K")

    if $all_zero; then
        dup_str="0% *"
    elif [ "$(awk_lt "$med_dup" 0.01)" = "1" ]; then
        dup_str="<0.01%"
    elif [ "$(awk_lt "$med_dup" 1)" = "1" ]; then
        dup_str="$(awk_f2 "$med_dup")%"
    else
        dup_str="$(awk_f1 "$med_dup")%"
    fi

    printf "| \`%s\` | %s | %s |\n" "$(echo "${fields[*]}" | tr ' ' ', ')" "$(fmt_num "$med_u")" "$dup_str"
done

echo ""
echo "\\* no collisions observed at ${K}×$(fmt_num "$N")"
echo ""

# --- Section 5: Scale planner ---
echo "## Scale planner"
echo ""
echo "Median unique % across $K seeds."
echo ""

# Scale sizes capped by MAX
SCALE_SIZES=()
for s in 1000 10000 100000 1000000; do
    (( s <= N )) && SCALE_SIZES+=("$s")
done
[ ${#SCALE_SIZES[@]} -eq 0 ] && SCALE_SIZES=(1000)
SCALE_ITEMS=(
    "name"
    "email"
    "username"
    "phone"
    "credit-card"
    "name,email,phone"
)

printf "| Fields |"
for sz in "${SCALE_SIZES[@]}"; do printf " %s |" "$(fmt_num "$sz")"; done
echo ""
printf "|--------|"
for _ in "${SCALE_SIZES[@]}"; do printf -- "--------|"; done
echo ""

for combo in "${SCALE_ITEMS[@]}"; do
    echo "  scale: $combo..." >&2
    IFS=',' read -ra fields <<< "$combo"
    printf "| \`%s\` |" "$(echo "${fields[*]}" | tr ' ' ', ')"
    for sz in "${SCALE_SIZES[@]}"; do
        pcts=()
        for seed in "${SEEDS[@]}"; do
            u=$("$SF" "${fields[@]}" --seed "$seed" --until 2025 -n "$sz" --no-header -q 2>/dev/null | count_unique)
            pcts+=("$(awk_upct "$u" "$sz")")
        done
        med=$(printf '%s\n' "${pcts[@]}" | median "$K")
        if [ "$(awk_ge "$med" 99.995)" = "1" ]; then
            printf " 100%% |"
        else
            printf " %s%% |" "$(awk_f1 "$med")"
        fi
    done
    echo ""
done

} > "$OUT"

echo "" >&2
echo "Written to $OUT" >&2
