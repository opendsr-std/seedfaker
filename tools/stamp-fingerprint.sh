#!/usr/bin/env bash
# Update output fingerprint in all files that reference it.
# Usage: make stamp-fingerprint
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="$ROOT/rust/target/release/seedfaker"

if [ ! -f "$SF" ]; then
  echo "Binary not found. Run 'make dev' first." >&2
  exit 1
fi

HASH=$("$SF" --fingerprint)
echo "Current fingerprint: $HASH"

# Cross-platform sed -i
sedi() {
  if sed --version >/dev/null 2>&1; then
    sed -i "$@"
  else
    sed -i '' "$@"
  fi
}

# Match sf0-<hex> pattern
OLD_PAT='sf0-[0-9a-f]\{16\}'

stamp() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "  SKIP $file (not found)"
    return
  fi
  if grep -q "$OLD_PAT" "$file"; then
    sedi "s/$OLD_PAT/$HASH/g" "$file"
    echo "  OK   $file"
  else
    echo "  SKIP $file (no match)"
  fi
}

stamp "$ROOT/FINGERPRINT"
stamp "$ROOT/rust/cli/tests/determinism.rs"
stamp "$ROOT/README.md"
stamp "$ROOT/docs/library.md"
stamp "$ROOT/packages/pip/README.md"
stamp "$ROOT/packages/npm/README.md"
stamp "$ROOT/benchmarks/results/comparisons.md"

echo ""
echo "Done."
