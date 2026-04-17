#!/usr/bin/env bash
# Regenerate test snapshot data after breaking changes to RNG or record numbering.
# Usage: make update-snapshots
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="$ROOT/rust/target/release/seedfaker"

if [ ! -f "$SF" ]; then
  echo "Binary not found. Run 'make build-local' first." >&2
  exit 1
fi

# --- golden.tsv ---
GOLDEN="$ROOT/rust/cli/tests/golden.tsv"
echo "=== golden.tsv ==="
TMP=$(mktemp)

# Extract field names from --list (strips modifiers and group headers)
"$SF" --list 2>/dev/null \
  | grep '^ ' \
  | awk '{print $1}' \
  | grep -v ':$' \
  | grep -v '^$' \
  | while IFS= read -r field; do
      val=$("$SF" "$field" --locale en --seed golden --until 2038 -n 1 2>/dev/null | head -1 || true)
      printf '%s\t%s\n' "$field" "$val"
    done > "$TMP"

mv "$TMP" "$GOLDEN"
echo "  $(wc -l < "$GOLDEN" | tr -d ' ') fields written."

# --- inline snapshots: print new expected values ---
echo ""
echo "=== Inline snapshot values (copy into tests if changed) ==="
echo ""

echo "--- snapshot_locale_de (cli.rs) ---"
"$SF" name --locale de -n 3 --seed snap2

echo "--- snapshot_abc_native_sr (cli.rs) ---"
"$SF" name --locale sr --abc native -n 3 --seed snap2

echo "--- snapshot_ctx_strict (cli.rs) ---"
"$SF" name email --ctx strict --locale de -n 3 --seed snap2

echo "--- snapshot_corrupt_extreme (corruption.rs) ---"
"$SF" name email phone --corrupt extreme -n 5 --seed snap2

echo "--- snapshot_corrupt_high (corruption.rs) ---"
"$SF" name email phone --corrupt high -n 5 --seed snap2

echo "--- snapshot_csv (formats.rs) ---"
"$SF" name email --format csv -n 3 --seed snap2

echo "--- snapshot_template (formats.rs) ---"
"$SF" -t '{{name}} <{{email}}>' -n 3 --seed snap2

echo "--- template_serial (formats.rs) ---"
"$SF" -t 'row-{{serial}}' -n 4 --seed ser

echo ""
echo "Done. Run 'make test-local' to verify."
