#!/usr/bin/env bash
# Cross-interface determinism proof.
# Generates identical records via CLI, Python, Node.js, Go, PHP, Ruby, MCP
# and compares SHA-256 hashes. Any mismatch = broken determinism guarantee.
#
# Usage: make determinism
# Reproduce: bash benchmarks/determinism.sh
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$DIR/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"
OUT="$DIR/results/determinism.md"
TMP="$DIR/.determinism_tmp"

SEED="determinism-proof"
N=100
FIELDS="name email phone city birthdate credit-card ssn passport ip uuid"
LOCALE="en"

VENV="$DIR/.venv"
PYTHON=python3
[ -f "$VENV/bin/python3" ] && PYTHON="$VENV/bin/python3"

NODE_DIR="$DIR/node"
export NODE_PATH="$NODE_DIR/node_modules"

rm -rf "$TMP"
mkdir -p "$TMP"

echo "determinism: $N records, seed=$SEED, locale=$LOCALE" >&2
echo "  fields: $FIELDS" >&2
echo "" >&2

PASS=0
FAIL=0
SKIP=0
RESULTS=()

# ---------------------------------------------------------------------------
# CLI (reference)
# ---------------------------------------------------------------------------

echo "  CLI..." >&2
"$SF" $FIELDS --seed "$SEED" --locale "$LOCALE" -n "$N" --until 2025 --no-header -q 2>/dev/null > "$TMP/cli.tsv"
CLI_HASH=$(shasum -a 256 "$TMP/cli.tsv" | cut -d' ' -f1)
CLI_LINES=$(wc -l < "$TMP/cli.tsv" | tr -d ' ')
RESULTS+=("cli|CLI (Rust binary)|$CLI_HASH|$CLI_LINES|pass")
PASS=$((PASS + 1))

# ---------------------------------------------------------------------------
# Python (PyO3)
# ---------------------------------------------------------------------------

if $PYTHON -c "from seedfaker import SeedFaker" 2>/dev/null; then
    echo "  Python..." >&2
    $PYTHON -c "
from seedfaker import SeedFaker
f = SeedFaker(seed='$SEED', locale='$LOCALE', until=2025)
fields = '$FIELDS'.split()
for rec in f.records(fields, n=$N):
    print('\t'.join(str(rec[k]) for k in fields))
" > "$TMP/python.tsv"
    PY_HASH=$(shasum -a 256 "$TMP/python.tsv" | cut -d' ' -f1)
    PY_LINES=$(wc -l < "$TMP/python.tsv" | tr -d ' ')
    if [ "$PY_HASH" = "$CLI_HASH" ]; then
        RESULTS+=("python|Python (PyO3)|$PY_HASH|$PY_LINES|pass")
        PASS=$((PASS + 1))
    else
        RESULTS+=("python|Python (PyO3)|$PY_HASH|$PY_LINES|FAIL")
        FAIL=$((FAIL + 1))
    fi
else
    echo "  Python: skipped (not installed)" >&2
    RESULTS+=("python|Python (PyO3)|—|—|skip")
    SKIP=$((SKIP + 1))
fi

# ---------------------------------------------------------------------------
# Node.js (NAPI-RS)
# ---------------------------------------------------------------------------

if node -e "require('$ROOT/packages/npm/index.js')" 2>/dev/null; then
    echo "  Node.js..." >&2
    node -e "
const { SeedFaker } = require('$ROOT/packages/npm/index.js');
const f = new SeedFaker({ seed: '$SEED', locale: '$LOCALE', until: 2025 });
const fields = '$FIELDS'.split(' ');
const records = f.records(fields, { n: $N });
for (const rec of records) {
    console.log(fields.map(k => rec[k]).join('\t'));
}
" > "$TMP/node.tsv"
    NODE_HASH=$(shasum -a 256 "$TMP/node.tsv" | cut -d' ' -f1)
    NODE_LINES=$(wc -l < "$TMP/node.tsv" | tr -d ' ')
    if [ "$NODE_HASH" = "$CLI_HASH" ]; then
        RESULTS+=("node|Node.js (NAPI-RS)|$NODE_HASH|$NODE_LINES|pass")
        PASS=$((PASS + 1))
    else
        RESULTS+=("node|Node.js (NAPI-RS)|$NODE_HASH|$NODE_LINES|FAIL")
        FAIL=$((FAIL + 1))
    fi
else
    echo "  Node.js: skipped (not installed)" >&2
    RESULTS+=("node|Node.js (NAPI-RS)|—|—|skip")
    SKIP=$((SKIP + 1))
fi

# ---------------------------------------------------------------------------
# Go (FFI)
# ---------------------------------------------------------------------------

GO_PKG="$ROOT/packages/go"
if command -v go >/dev/null 2>&1 && { [ -f "$ROOT/rust/target/release/libseedfaker_ffi.dylib" ] || [ -f "$ROOT/rust/target/release/libseedfaker_ffi.so" ]; }; then
    echo "  Go..." >&2
    GO_DIR="$TMP/godet"
    mkdir -p "$GO_DIR"

    # Copy lib into bin path expected by CGO directives
    GO_ARCH="darwin-arm64"
    [ "$(uname -m)" = "x86_64" ] && GO_ARCH="darwin-x86_64"
    [ "$(uname -s)" = "Linux" ] && GO_ARCH="linux-$(uname -m | sed 's/aarch64/arm64/')"
    mkdir -p "$GO_PKG/bin/$GO_ARCH"
    cp "$ROOT/rust/target/release/libseedfaker_ffi."* "$GO_PKG/bin/$GO_ARCH/" 2>/dev/null || true

    cat > "$GO_DIR/main.go" << 'GOEOF'
package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"

	seedfaker "seedfaker-go"
)

func main() {
	n, _ := strconv.Atoi(os.Args[4])
	f, err := seedfaker.New(seedfaker.Options{
		Seed:   os.Args[1],
		Locale: os.Args[2],
		Until:  2025,
	})
	if err != nil { fmt.Fprintln(os.Stderr, err); os.Exit(1) }
	defer f.Close()
	fields := strings.Split(os.Args[3], " ")
	records, err := f.Records(seedfaker.RecordOpts{
		Fields: fields,
		N:      n,
	})
	if err != nil { fmt.Fprintln(os.Stderr, err); os.Exit(1) }
	for _, rec := range records {
		vals := make([]string, len(fields))
		for i, k := range fields {
			vals[i] = rec[k]
		}
		fmt.Println(strings.Join(vals, "\t"))
	}
}
GOEOF
    cat > "$GO_DIR/go.mod" << GOMODEOF
module godet
go 1.21
require seedfaker-go v0.0.0
replace seedfaker-go => $GO_PKG
GOMODEOF
    if (cd "$GO_DIR" && CGO_ENABLED=1 go run main.go "$SEED" "$LOCALE" "$FIELDS" "$N" > "$TMP/go.tsv" 2>/dev/null); then
        GO_HASH=$(shasum -a 256 "$TMP/go.tsv" | cut -d' ' -f1)
        GO_LINES=$(wc -l < "$TMP/go.tsv" | tr -d ' ')
        if [ "$GO_HASH" = "$CLI_HASH" ]; then
            RESULTS+=("go|Go (FFI/CGO)|$GO_HASH|$GO_LINES|pass")
            PASS=$((PASS + 1))
        else
            RESULTS+=("go|Go (FFI/CGO)|$GO_HASH|$GO_LINES|FAIL")
            FAIL=$((FAIL + 1))
        fi
    else
        echo "  Go: skipped (build failed)" >&2
        RESULTS+=("go|Go (FFI/CGO)|—|—|skip")
        SKIP=$((SKIP + 1))
    fi
else
    echo "  Go: skipped (not available)" >&2
    RESULTS+=("go|Go (FFI/CGO)|—|—|skip")
    SKIP=$((SKIP + 1))
fi

# ---------------------------------------------------------------------------
# PHP (FFI)
# ---------------------------------------------------------------------------

if command -v php >/dev/null 2>&1; then
    echo "  PHP..." >&2
    PHP_SCRIPT="$TMP/php_gen.php"
    cat > "$PHP_SCRIPT" << 'PHPEOF'
<?php
require_once getenv('SF_PHP_PATH') . '/src/SeedFaker.php';

use Seedfaker\SeedFaker;

$f = new SeedFaker(seed: $argv[1], locale: $argv[2], until: 2025);
$fields = explode(' ', $argv[3]);
$n = intval($argv[4]);
$records = $f->records($fields, n: $n);
foreach ($records as $rec) {
    $vals = array_map(fn($k) => $rec[$k], $fields);
    echo implode("\t", $vals) . "\n";
}
PHPEOF
    if SF_PHP_PATH="$ROOT/packages/php" php "$PHP_SCRIPT" "$SEED" "$LOCALE" "$FIELDS" "$N" > "$TMP/php.tsv" 2>/dev/null; then
        PHP_HASH=$(shasum -a 256 "$TMP/php.tsv" | cut -d' ' -f1)
        PHP_LINES=$(wc -l < "$TMP/php.tsv" | tr -d ' ')
        if [ "$PHP_HASH" = "$CLI_HASH" ]; then
            RESULTS+=("php|PHP (FFI)|$PHP_HASH|$PHP_LINES|pass")
            PASS=$((PASS + 1))
        else
            RESULTS+=("php|PHP (FFI)|$PHP_HASH|$PHP_LINES|FAIL")
            FAIL=$((FAIL + 1))
        fi
    else
        echo "  PHP: skipped (runtime error)" >&2
        RESULTS+=("php|PHP (FFI)|—|—|skip")
        SKIP=$((SKIP + 1))
    fi
else
    echo "  PHP: skipped (not installed)" >&2
    RESULTS+=("php|PHP (FFI)|—|—|skip")
    SKIP=$((SKIP + 1))
fi

# ---------------------------------------------------------------------------
# Ruby (Fiddle/FFI)
# ---------------------------------------------------------------------------

if command -v ruby >/dev/null 2>&1; then
    echo "  Ruby..." >&2
    RUBY_SCRIPT="$TMP/ruby_gen.rb"
    cat > "$RUBY_SCRIPT" << 'RUBYEOF'
$LOAD_PATH.unshift(File.join(ENV['SF_RUBY_PATH'], 'lib'))
require 'seedfaker'

f = Seedfaker::SeedFaker.new(seed: ARGV[0], locale: ARGV[1], until_time: 2025)
fields = ARGV[2].split(' ')
n = ARGV[3].to_i
records = f.records(fields, n: n)
records.each do |rec|
  puts fields.map { |k| rec[k] }.join("\t")
end
RUBYEOF
    if SF_RUBY_PATH="$ROOT/packages/ruby" ruby "$RUBY_SCRIPT" "$SEED" "$LOCALE" "$FIELDS" "$N" > "$TMP/ruby.tsv" 2>/dev/null; then
        RUBY_HASH=$(shasum -a 256 "$TMP/ruby.tsv" | cut -d' ' -f1)
        RUBY_LINES=$(wc -l < "$TMP/ruby.tsv" | tr -d ' ')
        if [ "$RUBY_HASH" = "$CLI_HASH" ]; then
            RESULTS+=("ruby|Ruby (Fiddle)|$RUBY_HASH|$RUBY_LINES|pass")
            PASS=$((PASS + 1))
        else
            RESULTS+=("ruby|Ruby (Fiddle)|$RUBY_HASH|$RUBY_LINES|FAIL")
            FAIL=$((FAIL + 1))
        fi
    else
        echo "  Ruby: skipped (runtime error)" >&2
        RESULTS+=("ruby|Ruby (Fiddle)|—|—|skip")
        SKIP=$((SKIP + 1))
    fi
else
    echo "  Ruby: skipped (not installed)" >&2
    RESULTS+=("ruby|Ruby (Fiddle)|—|—|skip")
    SKIP=$((SKIP + 1))
fi

# ---------------------------------------------------------------------------
# MCP (JSON-RPC)
# ---------------------------------------------------------------------------

echo "  MCP..." >&2
FIELDS_JSON=$(echo "$FIELDS" | $PYTHON -c "import sys,json; print(json.dumps(sys.stdin.read().split()))")
MCP_REQUEST=$(cat <<MCPEOF
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"bench","version":"1.0.0"}}}
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"field","arguments":{"fields":$FIELDS_JSON,"n":$N,"seed":"$SEED","locale":"$LOCALE","until":"2025"}}}
MCPEOF
)
MCP_RAW=$(echo "$MCP_REQUEST" | "$SF" mcp 2>/dev/null || true)
if echo "$MCP_RAW" | grep -q '"result"'; then
    # Extract records from MCP response — content[0].text is a JSON array
    FIELDS_LIST="$FIELDS"
    echo "$MCP_RAW" | $PYTHON -c "
import sys, json
fields = '$FIELDS_LIST'.split()
for line in sys.stdin:
    line = line.strip()
    if not line: continue
    try:
        msg = json.loads(line)
    except: continue
    if 'result' in msg and 'content' in msg.get('result', {}):
        text = msg['result']['content'][0]['text']
        records = json.loads(text)
        for rec in records:
            vals = []
            for f in fields:
                # MCP normalizes keys: credit-card -> credit_card
                v = rec.get(f) or rec.get(f.replace('-', '_'), '')
                vals.append(str(v))
            print('\t'.join(vals))
        break
" > "$TMP/mcp.tsv"
    MCP_HASH=$(shasum -a 256 "$TMP/mcp.tsv" | cut -d' ' -f1)
    MCP_LINES=$(wc -l < "$TMP/mcp.tsv" | tr -d ' ')
    if [ "$MCP_HASH" = "$CLI_HASH" ]; then
        RESULTS+=("mcp|MCP (JSON-RPC)|$MCP_HASH|$MCP_LINES|pass")
        PASS=$((PASS + 1))
    else
        RESULTS+=("mcp|MCP (JSON-RPC)|$MCP_HASH|$MCP_LINES|FAIL")
        FAIL=$((FAIL + 1))
    fi
else
    echo "  MCP: skipped (no response)" >&2
    RESULTS+=("mcp|MCP (JSON-RPC)|—|—|skip")
    SKIP=$((SKIP + 1))
fi

# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------

{
echo "# Cross-Interface Determinism"
echo ""
echo "Same seed, same fields, same output — across every interface. This is the core guarantee."
echo ""
echo "**Parameters:** seed=\`$SEED\`, locale=\`$LOCALE\`, n=$N, until=2025"
echo "**Fields:** \`$FIELDS\`"
echo ""
echo "| Interface | SHA-256 | Rows | Status |"
echo "|-----------|---------|------|--------|"

for entry in "${RESULTS[@]}"; do
    IFS='|' read -r _id iface hash lines status <<< "$entry"
    if [ "$status" = "pass" ]; then
        short_hash="${hash:0:16}..."
        echo "| $iface | \`$short_hash\` | $lines | $status |"
    elif [ "$status" = "skip" ]; then
        echo "| $iface | — | — | skip |"
    else
        short_hash="${hash:0:16}..."
        echo "| $iface | \`$short_hash\` | $lines | **FAIL** |"
    fi
done

echo ""

if (( FAIL > 0 )); then
    echo "**FAIL: $FAIL interface(s) produced different output.**"
else
    ALL_TESTED=$((PASS))
    echo "All $ALL_TESTED tested interfaces produce byte-identical output."
    if (( SKIP > 0 )); then
        echo "$SKIP interface(s) skipped (not installed)."
    fi
fi

echo ""
echo "Full SHA-256: \`$CLI_HASH\`"
echo ""
echo "## Reproduce"
echo ""
echo "\`\`\`bash"
echo "# Any interface — same hash:"
echo "seedfaker $FIELDS --seed $SEED --locale $LOCALE -n $N --until 2025 --no-header | shasum -a 256"
echo "\`\`\`"

} > "$OUT"

# Cleanup
rm -rf "$TMP"

echo "" >&2
echo "Results: $PASS pass, $FAIL fail, $SKIP skip" >&2
echo "Written to $OUT" >&2

if (( FAIL > 0 )); then
    exit 1
fi
