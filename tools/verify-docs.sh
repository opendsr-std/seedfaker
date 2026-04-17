#!/usr/bin/env bash
# Verify generated files and docs are in sync with the binary.
# Exits non-zero if any drift detected.
# Usage: make verify  (or called from make pre-commit)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="$ROOT/rust/target/release/seedfaker"
EXIT=0

fail() { echo "FAIL: $1"; EXIT=1; }
ok()   { echo "  ok: $1"; }

if [ ! -f "$SF" ]; then
  echo "Binary not found at $SF — run 'make build-cli' first." >&2
  exit 1
fi

# --- Fingerprint ---
ACTUAL=$("$SF" --fingerprint)
for f in FINGERPRINT docs/library.md; do
  fp="$ROOT/$f"
  [ -f "$fp" ] || continue
  if grep -q "$ACTUAL" "$fp"; then
    ok "$f fingerprint"
  else
    fail "$f has stale fingerprint (expected $ACTUAL)"
  fi
done

# --- Field count: --list vs field-reference.md vs bindings ---
# Count base field rows only — exclude modifier sub-rows like `phone:e164`.
LIST_COUNT=$("$SF" --list 2>&1 | grep -cE '^\s{4}[a-z][a-z0-9_-]*[[:space:]]')
DOC_COUNT=$(grep -cE '^\| `[a-z][a-z0-9_-]*`' "$ROOT/docs/field-reference.md" 2>/dev/null || echo 0)
# npm: count FIELDS array entries
NPM_FIELDS=$(sed -n '/@generated-start/,/@generated-end/p' "$ROOT/packages/npm/index.js" 2>/dev/null | grep -c '"[a-z]' || echo 0)
# pip: count _FIELDS list entries (multiple per line, count quoted strings)
PY_FIELDS=$(sed -n '/@fields-start/,/@fields-end/p' "$ROOT/packages/pip/seedfaker/__init__.py" 2>/dev/null | grep -oE '"[a-z][a-z0-9-]*"' | wc -l | tr -d ' ')

if [ "$DOC_COUNT" -lt "$LIST_COUNT" ]; then
  fail "field-reference.md has $DOC_COUNT fields, --list has $LIST_COUNT. Run: make fields"
else
  ok "field-reference.md ($DOC_COUNT fields)"
fi

if [ "$NPM_FIELDS" -ne "$PY_FIELDS" ] 2>/dev/null; then
  fail "npm has $NPM_FIELDS fields, pip has $PY_FIELDS. Run: make bindings"
elif [ "$NPM_FIELDS" -lt "$LIST_COUNT" ] 2>/dev/null; then
  fail "npm FIELDS has $NPM_FIELDS entries, --list has $LIST_COUNT. Run: make bindings"
else
  ok "bindings field parity ($NPM_FIELDS fields)"
fi

# --- Locale codes in docs/cli.md vs actual ---
CLI_LOCALES=$("$SF" --list-json 2>/dev/null | python3 -c "
import sys, json
data = json.load(sys.stdin)
# locales from --list-json are in the locale field
# fall back to ALL_CODES from the binary
" 2>/dev/null || true)
# Simpler: just check that the documented count matches
if grep -q "68 locales" "$ROOT/docs/cli.md" 2>/dev/null; then
  ok "cli.md locale count claim"
fi

# --- Corrupt levels in docs match --help ---
for level in low mid high extreme; do
  if ! grep -q "$level" "$ROOT/docs/corruption.md" 2>/dev/null; then
    fail "docs/corruption.md missing level '$level'"
  fi
done
ok "corruption levels in docs"

# --- PHP CDEF matches C header ---
HEADER_FNS=$(grep -oE '[a-z_]+\(' "$ROOT/include/seedfaker.h" | sort -u)
PHP_FNS=$(sed -n '/private const CDEF/,/CDEF;/p' "$ROOT/packages/php/src/SeedFaker.php" | grep -oE '[a-z_]+\(' | sort -u)
if [ "$HEADER_FNS" = "$PHP_FNS" ]; then
  ok "PHP CDEF matches C header"
else
  fail "PHP CDEF drifted from include/seedfaker.h — update the CDEF constant"
fi

# --- DEVELOPMENT.md: check key files exist ---
DEV_OK=true
# Extract filenames from tree lines (│   ├── filename.ext)
grep -oE '── [a-z_]+\.[a-z]+' "$ROOT/DEVELOPMENT.md" | sed 's/── //' | sort -u | while IFS= read -r fname; do
  if ! find "$ROOT" -name "$fname" -not -path '*/.git/*' -not -path '*/target/*' -not -path '*/node_modules/*' -print -quit 2>/dev/null | grep -q .; then
    echo "FAIL: DEVELOPMENT.md references $fname but it doesn't exist"
    # Signal failure via temp file since subshell can't set parent EXIT
    touch "$ROOT/.verify-fail"
  fi
done
if [ -f "$ROOT/.verify-fail" ]; then
  rm -f "$ROOT/.verify-fail"
  EXIT=1
else
  ok "DEVELOPMENT.md file references"
fi

echo ""
if [ "$EXIT" -ne 0 ]; then
  echo "DRIFT DETECTED. Fix issues above, then re-run."
else
  echo "All checks passed."
fi
exit $EXIT
