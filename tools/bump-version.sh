#!/usr/bin/env bash
# Usage: bash tools/bump-version.sh 0.1.0-alpha.5
# Prerelease tag converts to PEP 440 for pip: -alpha.N → aN

set -euo pipefail
cd "$(dirname "$0")/.."

# Cross-platform sed -i
sedi() {
  if sed --version 2>/dev/null | grep -q GNU; then
    sed -i "$@"
  else
    sed -i '' "$@"
  fi
}

VERSION="${1:-}"
if [ -z "$VERSION" ]; then
  echo "Usage: $0 VERSION"
  echo "  e.g. $0 0.1.0-alpha.2"
  exit 1
fi

# Detect current version from core Cargo.toml
OLD=$(grep '^version' rust/core/Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
echo "Current: $OLD"
echo "New:     $VERSION"

# PEP 440 conversion: 0.1.0-alpha.2 → 0.1.0a2, 0.1.0-beta.1 → 0.1.0b1, 0.1.0-rc.1 → 0.1.0rc1
PEP440=$(echo "$VERSION" | sed 's/-alpha\.\([0-9]*\)/a\1/' | sed 's/-beta\.\([0-9]*\)/b\1/' | sed 's/-rc\.\([0-9]*\)/rc\1/')
OLD_PEP440=$(echo "$OLD" | sed 's/-alpha\.\([0-9]*\)/a\1/' | sed 's/-beta\.\([0-9]*\)/b\1/' | sed 's/-rc\.\([0-9]*\)/rc\1/')

echo "PEP 440: $PEP440"
echo ""

# ── Cargo.toml (6 crates: core, cli, napi, pyo3, ffi, wasm) ──
for crate in core cli napi pyo3 ffi wasm; do
  f="rust/$crate/Cargo.toml"
  sedi "s/^version = \"$OLD\"/version = \"$VERSION\"/" "$f"
  # Also update dependency versions pointing to old version
  sedi "s/version = \"$OLD\"/version = \"$VERSION\"/g" "$f"
  echo "  $f"
done

# ── npm package.json (main + 5 platforms + wasm) — replace ALL version occurrences ──
for f in packages/npm/package.json \
         packages/npm/platforms/darwin-arm64/package.json \
         packages/npm/platforms/darwin-x64/package.json \
         packages/npm/platforms/linux-x64/package.json \
         packages/npm/platforms/linux-arm64/package.json \
         packages/npm/platforms/win32-x64/package.json \
         packages/wasm/package.json \
         packages/npm-cli/package.json; do
  sedi "s/\"$OLD\"/\"$VERSION\"/g" "$f"
  echo "  $f"
done

# ── pip pyproject.toml (main + pyo3) ──
for f in packages/pip/pyproject.toml rust/pyo3/pyproject.toml; do
  sedi "s/^version = \"$OLD_PEP440\"/version = \"$PEP440\"/" "$f"
  echo "  $f"
done

# ── PHP composer.json ──
f="packages/php/composer.json"
sedi "s/\"version\": \"$OLD\"/\"version\": \"$VERSION\"/" "$f"
echo "  $f"

# ── README.md version line ──
f="README.md"
sedi "s/^\`$OLD\`/\`$VERSION\`/" "$f"
echo "  $f"

# ── Python __version__ ──
f="packages/pip/seedfaker/__init__.py"
sedi "s/__version__ = \"$OLD_PEP440\"/__version__ = \"$PEP440\"/" "$f"
echo "  $f"

# ── Ruby gemspec ──
RUBY_OLD=$(echo "$OLD" | sed 's/-/.pre./')
RUBY_NEW=$(echo "$VERSION" | sed 's/-/.pre./')
f="packages/ruby/seedfaker.gemspec"
sedi "s/s\.version     = \"$RUBY_OLD\"/s.version     = \"$RUBY_NEW\"/" "$f"
echo "  $f"

echo ""

# ── Self-check: no file should still contain the old version ──
STALE=$(grep -rn \
  --include="Cargo.toml" \
  --include="package.json" \
  --include="pyproject.toml" \
  --include="composer.json" \
  --include="__init__.py" \
  --include="*.gemspec" \
  --include="README.md" \
  -e "\"$OLD\"" -e "\"$OLD_PEP440\"" -e "version = \"$OLD\"" -e "version = \"$OLD_PEP440\"" \
  rust/ packages/ README.md 2>/dev/null \
  | grep -v "Cargo.lock" || true)

if [ -n "$STALE" ]; then
  echo "ERROR: old version still present in:"
  echo "$STALE"
  exit 1
fi

echo "Done. All files updated to $VERSION."
echo "Then run: make pre-commit"
