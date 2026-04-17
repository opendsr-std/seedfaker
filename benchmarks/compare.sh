#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"
VENV="$DIR/.venv"
NODE_DIR="$DIR/node"

SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"
RUNS="${BENCH_RUNS:-5}"
OUT="${1:-$DIR/results/comparisons.md}"
SKIP_NODE="${BENCH_SKIP_NODE:-}"
SKIP_PYTHON="${BENCH_SKIP_PYTHON:-}"

PYTHON=python3
[ -f "$VENV/bin/python3" ] && PYTHON="$VENV/bin/python3"
export PYTHONPATH="${PYTHONPATH:-$ROOT/packages/pip}"

export NODE_PATH="$NODE_DIR/node_modules"
GOBIN="$(go env GOPATH 2>/dev/null)/bin"
export PATH="$NODE_DIR/node_modules/.bin:$GOBIN:$VENV/bin:$PATH"
export SEEDFAKER_NO_WARN=1

TIERS="3 5 10 15 20"
N="${BENCH_N:-100000}"
N_LIB="${BENCH_N_LIB:-10000}"

# Field tiers — PII/anonymization use-case
SF_T3="name email phone"
SF_T5="name email phone city birthdate"
SF_T10="name email phone city birthdate country username postal-code ssn credit-card"
SF_T15="name email phone city birthdate country username postal-code ssn credit-card address company-name job-title iban password"
SF_T20="name email phone city birthdate country username postal-code ssn credit-card address company-name job-title iban password ip uuid timestamp passport national-id"

# fakedata CLI field tiers (closest equivalents)
FD_T3="name email phone"
FD_T5="name email phone city date"
FD_T10="name email phone city date country username state int int"
FD_T15="name email phone city date country username state int int sentence industry occupation int domain"
FD_T20="name email phone city date country username state int int sentence industry occupation int domain ipv4 uuidv4 date int int"

# ===================================================================
# Preflight
# ===================================================================

echo "Preflight checks..." >&2
[ -x "$SF" ] || { echo "FAIL: seedfaker binary not found at $SF" >&2; exit 1; }
command -v fakedata >/dev/null || { echo "FAIL: fakedata not found" >&2; exit 1; }

if [ -z "$SKIP_PYTHON" ]; then
    $PYTHON -c "from seedfaker import SeedFaker; SeedFaker(seed='check')" || { echo "FAIL: seedfaker PyO3 not installed" >&2; exit 1; }
    $PYTHON -c "import faker" || { echo "FAIL: faker not installed" >&2; exit 1; }
    $PYTHON -c "import mimesis" || { echo "FAIL: mimesis not installed" >&2; exit 1; }
    $PYTHON -c "import polyfactory" || { echo "FAIL: polyfactory not installed" >&2; exit 1; }
fi

if [ -z "$SKIP_NODE" ]; then
    node -e "const{SeedFaker}=require('$ROOT/packages/npm/index.js');new SeedFaker({seed:'check'})" || { echo "FAIL: seedfaker NAPI not available" >&2; exit 1; }
    node -e "require('@faker-js/faker')" || { echo "FAIL: @faker-js/faker not installed" >&2; exit 1; }
    node -e "require('chance')" || { echo "FAIL: chance not installed" >&2; exit 1; }
    node -e "require('@ngneat/falso').randFullName()" || { echo "FAIL: @ngneat/falso not installed" >&2; exit 1; }
    node -e "require('json-schema-faker')" || { echo "FAIL: json-schema-faker not installed" >&2; exit 1; }
fi

echo "  ok" >&2

# ===================================================================
# Helpers
# ===================================================================

median() { sort -n | awk '{a[NR]=$1} END {print a[int((NR+1)/2)]}'; }

# bench LABEL CMD... — warm-up + RUNS timed, return median
bench() {
    local _label="$1"; shift
    "$@" > /dev/null 2>&1 || { echo "FAIL: warmup failed: $*" >&2; exit 1; }
    local times="" t
    for _ in $(seq 1 "$RUNS"); do
        t=$(perl -MTime::HiRes=time -e '$s=time; open STDOUT, ">/dev/null"; open STDERR, ">/dev/null"; system(@ARGV); open STDOUT, ">&", 3; printf "%.3f\n", time-$s' -- "$@" 3>&1)
        if [ -z "$t" ]; then
            echo "FAIL: no timing from: $*" >&2; exit 1
        fi
        times="${times}${t}\n"
    done
    printf '%b' "$times" | median
}

# lib_bench CMD... — warm-up + RUNS timed, return median elapsed
lib_bench() {
    "$@" > /dev/null 2>&1 || { echo "FAIL: warmup failed: $*" >&2; exit 1; }
    local times="" out t
    for _ in $(seq 1 "$RUNS"); do
        out=$("$@" 2>&1) || { echo "FAIL: benchmark failed: $*" >&2; exit 1; }
        t=$(echo "$out" | jval elapsed)
        if [ -z "$t" ] || [ "$t" = "None" ]; then
            echo "FAIL: invalid result from: $*" >&2
            echo "  output: $out" >&2
            exit 1
        fi
        times="${times}${t}\n"
    done
    printf '%b' "$times" | median
}

rps() {
    awk "BEGIN { t=$2+0; if(t<0.001) t=0.001; r=$1/t;
      if(r>=1e6) printf \"%.1fM\",r/1e6; else if(r>=1e3) printf \"%.0fK\",r/1e3; else printf \"%.0f\",r }"
}

pct() {
    awk "BEGIN { b=$1+0; v=$2+0; if(b<0.001) b=0.001; d=(v/b-1)*100;
      if(d>-1&&d<1) printf \"~0%%\"; else if(d>0) printf \"+%.0f%%\",d; else printf \"%.0f%%\",d }"
}

jval() { $PYTHON -c "import sys,json; print(json.load(sys.stdin)['$1'])"; }

fmt_cell() {
    local n="$1" t="$2"
    printf "%ss (%s/s)" "$t" "$(rps "$n" "$t")"
}

# Format cell with comparison to seedfaker time
fmt_vs() {
    local n="$1" t="$2" sf_t="$3"
    local base
    base=$(printf "%ss (%s/s)" "$t" "$(rps "$n" "$t")")
    local tag
    tag=$(awk "BEGIN { s=$sf_t+0; o=$t+0;
      if(s<0.001) s=0.001; if(o<0.001) o=0.001;
      r=o/s;
      if(r>1.05) printf \"%.1fx slower\",r;
      else if(r<0.95) printf \"%.1fx faster\",1/r;
      else printf \"~same\" }")
    printf "%s · *%s*" "$base" "$tag"
}

# ===================================================================
# Output
# ===================================================================

{
echo "# Benchmark Results"
echo ""
echo "## Environment"
echo ""
echo "- **Date:** $(date -u +"%Y-%m-%d %H:%M UTC")"
echo "- **OS:** $(uname -s) $(uname -r) $(uname -m)"
echo "- **CPU:** $(sysctl -n machdep.cpu.brand_string 2>/dev/null || grep -m1 'model name' /proc/cpuinfo 2>/dev/null | sed 's/.*: //' || echo 'unknown')"
echo "- **RAM:** $(sysctl -n hw.memsize 2>/dev/null | awk '{printf "%.0f GB", $1/1073741824}' || free -h 2>/dev/null | awk '/Mem:/{print $2}' || echo 'unknown')"
echo "- **Rust:** $(source "$HOME/.cargo/env" 2>/dev/null; rustc --version 2>/dev/null || echo 'N/A')"
echo "- **Python:** $($PYTHON --version 2>/dev/null || echo 'N/A')"
echo "- **Node:** $(node --version 2>/dev/null || echo 'N/A')"
echo "- **seedfaker:** $($SF --version 2>/dev/null || echo 'dev')"
if [ -z "$SKIP_PYTHON" ]; then
    echo "- **faker:** $($PYTHON -c 'import faker; print(faker.VERSION)')"
    echo "- **mimesis:** $($PYTHON -c 'import mimesis; print(mimesis.__version__)')"
    echo "- **polyfactory:** $($PYTHON -c 'from importlib.metadata import version; print(version("polyfactory"))')"
fi
if [ -z "$SKIP_NODE" ]; then
    echo "- **@faker-js/faker:** $(node -e "console.log(require('@faker-js/faker/package.json').version)")"
    echo "- **chance:** $(node -e "console.log(require('chance/package.json').version)")"
    echo "- **@ngneat/falso:** $(node -e "try{console.log(require('@ngneat/falso/package.json').version)}catch(e){console.log('unknown')}")"
    echo "- **json-schema-faker:** $(node -e "try{console.log(require('json-schema-faker/package.json').version)}catch(e){console.log('unknown')}")"
fi
echo "- **fakedata:** $(fakedata --version 2>/dev/null || echo 'N/A')"
echo "- **Method:** median of ${RUNS} runs (1 warm-up discarded)"
echo ""

# ===================================================================
# 1. CLI THROUGHPUT
# ===================================================================

echo "## 1. CLI throughput (${N} records, stdout > /dev/null)"
echo ""
echo "Both tools generate to /dev/null. seedfaker produces format-realistic PII (Luhn credit cards, IBAN check digits, locale-aware gov IDs). fakedata uses simpler generators — see [field substitutions](#field-substitutions) below."
echo ""
echo "| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |"
echo "|------|----------|----------|-----------|-----------|-----------|"

echo "  seedfaker CLI..." >&2
# shellcheck disable=SC2086
C3=$(bench sf3 "$SF" $SF_T3 -n "$N" --seed bench)
C5=$(bench sf5 "$SF" $SF_T5 -n "$N" --seed bench)
C10=$(bench sf10 "$SF" $SF_T10 -n "$N" --seed bench)
C15=$(bench sf15 "$SF" $SF_T15 -n "$N" --seed bench)
C20=$(bench sf20 "$SF" $SF_T20 -n "$N" --seed bench)
printf "| seedfaker | %s | %s | %s | %s | %s |\n" \
    "$(fmt_cell "$N" "$C3")" "$(fmt_cell "$N" "$C5")" "$(fmt_cell "$N" "$C10")" "$(fmt_cell "$N" "$C15")" "$(fmt_cell "$N" "$C20")"

echo "  fakedata CLI..." >&2
# shellcheck disable=SC2086
F3=$(bench fd3 fakedata --limit "$N" $FD_T3)
F5=$(bench fd5 fakedata --limit "$N" $FD_T5)
F10=$(bench fd10 fakedata --limit "$N" $FD_T10)
F15=$(bench fd15 fakedata --limit "$N" $FD_T15)
F20=$(bench fd20 fakedata --limit "$N" $FD_T20)
printf "| fakedata | %s | %s | %s | %s | %s |\n" \
    "$(fmt_vs "$N" "$F3" "$C3")" "$(fmt_vs "$N" "$F5" "$C5")" "$(fmt_vs "$N" "$F10" "$C10")" "$(fmt_vs "$N" "$F15" "$C15")" "$(fmt_vs "$N" "$F20" "$C20")"

echo ""

# ===================================================================
# 2. PYTHON LIBRARY
# ===================================================================

if [ -z "$SKIP_PYTHON" ]; then
    echo "## 2. Python library (${N_LIB} records, in-memory)"
    echo ""
    echo "seedfaker: PyO3 native extension. polyfactory: random strings (not structured PII)."
    echo ""
    echo "| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |"
    echo "|------|----------|----------|-----------|-----------|-----------|"

    echo "  python: seedfaker..." >&2
    PY_SF3=$(lib_bench $PYTHON "$DIR/python/bench_seedfaker.py" "$N_LIB" 3)
    PY_SF5=$(lib_bench $PYTHON "$DIR/python/bench_seedfaker.py" "$N_LIB" 5)
    PY_SF10=$(lib_bench $PYTHON "$DIR/python/bench_seedfaker.py" "$N_LIB" 10)
    PY_SF15=$(lib_bench $PYTHON "$DIR/python/bench_seedfaker.py" "$N_LIB" 15)
    PY_SF20=$(lib_bench $PYTHON "$DIR/python/bench_seedfaker.py" "$N_LIB" 20)
    printf "| seedfaker | %s | %s | %s | %s | %s |\n" \
        "$(fmt_cell "$N_LIB" "$PY_SF3")" "$(fmt_cell "$N_LIB" "$PY_SF5")" "$(fmt_cell "$N_LIB" "$PY_SF10")" "$(fmt_cell "$N_LIB" "$PY_SF15")" "$(fmt_cell "$N_LIB" "$PY_SF20")"

    for script in bench_faker.py bench_mimesis.py bench_polyfactory.py; do
        tool=$(echo "$script" | sed 's/bench_//;s/\.py//')
        echo "  python: $tool..." >&2
        P3=$(lib_bench $PYTHON "$DIR/python/$script" "$N_LIB" 3)
        P5=$(lib_bench $PYTHON "$DIR/python/$script" "$N_LIB" 5)
        P10=$(lib_bench $PYTHON "$DIR/python/$script" "$N_LIB" 10)
        P15=$(lib_bench $PYTHON "$DIR/python/$script" "$N_LIB" 15)
        P20=$(lib_bench $PYTHON "$DIR/python/$script" "$N_LIB" 20)
        printf "| %s | %s | %s | %s | %s | %s |\n" "$tool" \
            "$(fmt_vs "$N_LIB" "$P3" "$PY_SF3")" "$(fmt_vs "$N_LIB" "$P5" "$PY_SF5")" "$(fmt_vs "$N_LIB" "$P10" "$PY_SF10")" "$(fmt_vs "$N_LIB" "$P15" "$PY_SF15")" "$(fmt_vs "$N_LIB" "$P20" "$PY_SF20")"
    done
    echo ""
fi

# ===================================================================
# 3. NODE.JS LIBRARY
# ===================================================================

if [ -z "$SKIP_NODE" ]; then
    echo "## 3. Node.js library (${N_LIB} records, in-memory)"
    echo ""
    echo "seedfaker: NAPI-RS native extension."
    echo ""
    echo "| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |"
    echo "|------|----------|----------|-----------|-----------|-----------|"

    echo "  node: seedfaker..." >&2
    JS_SF3=$(lib_bench node "$NODE_DIR/bench_seedfaker.js" "$N_LIB" 3)
    JS_SF5=$(lib_bench node "$NODE_DIR/bench_seedfaker.js" "$N_LIB" 5)
    JS_SF10=$(lib_bench node "$NODE_DIR/bench_seedfaker.js" "$N_LIB" 10)
    JS_SF15=$(lib_bench node "$NODE_DIR/bench_seedfaker.js" "$N_LIB" 15)
    JS_SF20=$(lib_bench node "$NODE_DIR/bench_seedfaker.js" "$N_LIB" 20)
    printf "| seedfaker | %s | %s | %s | %s | %s |\n" \
        "$(fmt_cell "$N_LIB" "$JS_SF3")" "$(fmt_cell "$N_LIB" "$JS_SF5")" "$(fmt_cell "$N_LIB" "$JS_SF10")" "$(fmt_cell "$N_LIB" "$JS_SF15")" "$(fmt_cell "$N_LIB" "$JS_SF20")"

    for script in bench_fakerjs.js bench_chance.js bench_falso.js bench_jsf.js; do
        tool=$(echo "$script" | sed 's/bench_//;s/\.js//')
        echo "  node: $tool..." >&2
        J3=$(lib_bench node "$NODE_DIR/$script" "$N_LIB" 3)
        J5=$(lib_bench node "$NODE_DIR/$script" "$N_LIB" 5)
        J10=$(lib_bench node "$NODE_DIR/$script" "$N_LIB" 10)
        J15=$(lib_bench node "$NODE_DIR/$script" "$N_LIB" 15)
        J20=$(lib_bench node "$NODE_DIR/$script" "$N_LIB" 20)
        printf "| %s | %s | %s | %s | %s | %s |\n" "$tool" \
            "$(fmt_vs "$N_LIB" "$J3" "$JS_SF3")" "$(fmt_vs "$N_LIB" "$J5" "$JS_SF5")" "$(fmt_vs "$N_LIB" "$J10" "$JS_SF10")" "$(fmt_vs "$N_LIB" "$J15" "$JS_SF15")" "$(fmt_vs "$N_LIB" "$J20" "$JS_SF20")"
    done
    echo ""
fi

# ===================================================================
# 4. STARTUP OVERHEAD
# ===================================================================

echo "## 4. Startup overhead (1 record)"
echo ""
echo "| Tool | Time |"
echo "|------|------|"

echo "  startup..." >&2
T_START=$(bench sf-start "$SF" name -n 1 --seed x)
printf "| seedfaker CLI | %ss |\n" "$T_START"

if [ -z "$SKIP_PYTHON" ]; then
    T_FK=$(bench fk-start $PYTHON "$DIR/python/bench_faker.py" 1 3)
    T_MM=$(bench mm-start $PYTHON "$DIR/python/bench_mimesis.py" 1 3)
    printf "| faker.py (+ interpreter) | %ss |\n" "$T_FK"
    printf "| mimesis (+ interpreter) | %ss |\n" "$T_MM"
fi
echo ""

# ===================================================================
# 5. FEATURE OVERHEAD
# ===================================================================

echo "## 5. Feature overhead (seedfaker CLI, ${N} records)"
echo ""
echo "Baseline: 3 PII fields (name, email, phone), TSV to /dev/null."
echo ""
echo "| Feature | Time | Overhead |"
echo "|---------|------|----------|"

T_BASE="$C3"
echo "  features..." >&2
T_CSV=$(bench sf-csv "$SF" name email phone --format csv -n "$N" --seed bench --until 2025)
T_CTX=$(bench sf-ctx "$SF" name email phone -n "$N" --ctx strict --seed bench --until 2025)
T_CH=$(bench sf-ch "$SF" name email phone -n "$N" --corrupt high --seed bench --until 2025)

printf "| baseline (TSV) | %ss | — |\n" "$T_BASE"
printf "| --format csv | %ss | %s |\n" "$T_CSV" "$(pct "$T_BASE" "$T_CSV")"
printf "| --ctx strict | %ss | %s |\n" "$T_CTX" "$(pct "$T_BASE" "$T_CTX")"
printf "| --corrupt high | %ss | %s |\n" "$T_CH" "$(pct "$T_BASE" "$T_CH")"
echo ""

echo "### Template overhead (same fields: TSV vs inline template vs YAML config)" >&2
echo "### Template overhead (same fields: TSV vs inline template vs YAML config)"
echo ""
echo "| Fields | TSV | Inline \`-t\` | YAML config | TPL vs TSV |"
echo "|--------|-----|-------------|-----------|------------|"

BENCH_TMP="$DIR/.bench_tmp"
mkdir -p "$BENCH_TMP"

for tier in $TIERS; do
    echo "  template: ${tier} fields..." >&2
    eval "FIELDS=\$SF_T${tier}"

    # Build template string and config file from the same fields
    TPL=""
    VARS=""
    for f in $FIELDS; do
        TPL="${TPL}{{${f}}} "
        VARS="${VARS}  ${f}: ${f}\n"
    done

    CFG_FILE="$BENCH_TMP/t${tier}.yaml"
    printf "columns:\n%btemplate: '%s'\n" "$VARS" "$TPL" > "$CFG_FILE"

    # shellcheck disable=SC2086
    T_TSV=$(bench "sf-tsv${tier}" "$SF" $FIELDS -n "$N" --seed bench)
    # shellcheck disable=SC2086
    T_TPL=$(bench "sf-tpl${tier}" "$SF" $FIELDS -t "$TPL" -n "$N" --seed bench)
    T_CFG=$(bench "sf-cfg${tier}" "$SF" run "$CFG_FILE" -n "$N" --seed bench)

    printf "| %s | %ss | %ss | %ss | %s |\n" "$tier" "$T_TSV" "$T_TPL" "$T_CFG" "$(pct "$T_TSV" "$T_TPL")"
done

rm -rf "$BENCH_TMP"
echo ""

# ===================================================================
# 6. TEMPLATE ENGINE
# ===================================================================

# ===================================================================
# NOTES
# ===================================================================

echo "## Methodology"
echo ""
echo "- **Timing:** median of ${RUNS} runs, 1 warm-up discarded."
echo "- **CLI:** wall-clock via \`Time::HiRes\`, stdout to /dev/null."
echo "- **Library:** internal elapsed time reported by each script."
echo "- **Template engine:** criterion framework (statistical, outlier-aware)."
echo ""
echo "## Field tiers"
echo ""
echo "| Tier | seedfaker fields | Notes |"
echo "|------|------------------|-------|"
echo "| 3 | name, email, phone | Common PII |"
echo "| 5 | + city, birthdate | Demographic |"
echo "| 10 | + country, username, postal-code, ssn, credit-card | With checksum validation |"
echo "| 15 | + address, company-name, job-title, iban, password | Heavy formatting |"
echo "| 20 | + ip, uuid, timestamp, passport, national-id | Full PII set |"
echo ""
echo "## Field substitutions"
echo ""
echo "Not all tools support the same fields. Where a tool lacks an equivalent, the closest available generator is used. This affects comparisons at 10+ fields."
echo ""
echo "| seedfaker field | fakedata substitute | Impact |"
echo "|-----------------|---------------------|--------|"
echo "| ssn | \`int\` | No format validation |"
echo "| credit-card | \`int\` | No Luhn checksum |"
echo "| iban | \`domain\` | Different complexity |"
echo "| passport | \`int\` | No format rules |"
echo "| national-id | \`int\` | No locale dispatch |"
echo ""
echo "**polyfactory** generates unstructured random strings for all \`str\` fields. **@ngneat/falso** does not support deterministic seeding."
echo ""
echo "## Why native bindings are faster"
echo ""
echo "seedfaker Python and Node.js packages call the same compiled Rust core via native extensions (PyO3/NAPI-RS). Pure-Python and pure-JS libraries run interpreted code per field per record — the gap is inherent to the runtime, not a quality difference."
echo ""
echo "## Reproduce"
echo ""
echo "\`\`\`bash"
echo "benchmarks/install.sh"
echo "make bench-full"
echo "\`\`\`"

# ===================================================================
# APPENDIX
# ===================================================================

echo ""
echo "## Per-field performance"
echo ""
echo "See [results/fields.md](results/fields.md) (\`make bench-fields\`)."
echo ""

} > "${OUT}.tmp"

mv "${OUT}.tmp" "$OUT"
echo "" >&2
cat "$OUT"
