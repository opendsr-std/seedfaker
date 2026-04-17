#!/usr/bin/env bash
# Inject native module SHA256 checksums into wrapper source code.
# Called by CI before publishing. Replaces placeholders between @checksum markers.
#
# Usage: bash tools/inject-checksums.sh <staging-dir>
set -euo pipefail

STAGING="${1:?Usage: inject-checksums.sh <staging-dir>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Cross-platform sed -i
sedi() {
  if sed --version >/dev/null 2>&1; then sed -i "$@"; else sed -i '' "$@"; fi
}

echo "Injecting checksums from $STAGING..."

# --- npm: per-platform checksums into index.js ---
NPM_CHECKSUMS="{"
first=true
for platform_dir in "$STAGING"/*/; do
  [ -d "$platform_dir" ] || continue
  platform=$(basename "$platform_dir")
  node_file="$platform_dir/npm/seedfaker_napi.node"
  if [ -f "$node_file" ]; then
    hash=$(sha256sum "$node_file" | cut -d' ' -f1)
    if [ "$first" = true ]; then first=false; else NPM_CHECKSUMS+=","; fi
    NPM_CHECKSUMS+="\"$platform\":\"$hash\""
    echo "  npm $platform: ${hash:0:16}..."
  fi
done
NPM_CHECKSUMS+="}"

# Replace between @checksums-start and @checksums-end in index.js
sedi "/@checksums-start/,/@checksums-end/c\\
// @checksums-start\\
const CHECKSUMS = ${NPM_CHECKSUMS};\\
// @checksums-end" "$ROOT/packages/npm/index.js"

# --- pip: single checksum into __init__.py ---
# Find the .so for the current platform (CI builds for specific targets)
PY_HASH=""
for f in "$STAGING"/*/pyo3.so "$STAGING"/*/pyo3.dylib; do
  if [ -f "$f" ]; then
    PY_HASH=$(sha256sum "$f" | cut -d' ' -f1)
    echo "  pip: ${PY_HASH:0:16}..."
    break
  fi
done
if [ -n "$PY_HASH" ]; then
  sedi "/@checksum-start/,/@checksum-end/c\\
# @checksum-start\\
_NATIVE_CHECKSUM = \"${PY_HASH}\"\\
# @checksum-end" "$ROOT/packages/pip/seedfaker/__init__.py"
fi

# --- PHP: per-platform checksums into SeedFaker.php ---
# Map staging dir name → (php-target, ffi-filename).
php_target_for() {
  case "$1" in
    linux-x64)    echo "linux-x86_64 libseedfaker_ffi.so" ;;
    linux-arm64)  echo "linux-arm64 libseedfaker_ffi.so" ;;
    darwin-x64)   echo "darwin-x86_64 libseedfaker_ffi.dylib" ;;
    darwin-arm64) echo "darwin-arm64 libseedfaker_ffi.dylib" ;;
    *) return 1 ;;
  esac
}

PHP_REPLACEMENT=$(mktemp)
echo "        // @checksums-start" > "$PHP_REPLACEMENT"
for platform_dir in "$STAGING"/*/; do
  [ -d "$platform_dir" ] || continue
  platform=$(basename "$platform_dir")
  mapping=$(php_target_for "$platform") || continue
  target=${mapping%% *}
  lib=${mapping##* }
  ffi_file="$platform_dir/$lib"
  if [ -f "$ffi_file" ]; then
    h=$(sha256sum "$ffi_file" | cut -d' ' -f1)
    printf "        '%s' => '%s',\n" "$target" "$h" >> "$PHP_REPLACEMENT"
    echo "  php $target: ${h:0:16}..."
  fi
done
echo "        // @checksums-end" >> "$PHP_REPLACEMENT"

# Replace section between @checksums-start and @checksums-end in SeedFaker.php
PHP_FILE="$ROOT/packages/php/src/SeedFaker.php"
awk -v repl="$PHP_REPLACEMENT" '
  /@checksums-start/ {
    while ((getline line < repl) > 0) print line
    close(repl)
    skip = 1
    next
  }
  /@checksums-end/ {
    skip = 0
    next
  }
  !skip { print }
' "$PHP_FILE" > "$PHP_FILE.new" && mv "$PHP_FILE.new" "$PHP_FILE"
rm -f "$PHP_REPLACEMENT"

# --- Ruby: single checksum into seedfaker.rb (single arch — gem flat layout) ---
# Picks first available FFI binary; Ruby gem currently single-arch.
RUBY_HASH=""
for f in "$STAGING"/*/libseedfaker_ffi.so "$STAGING"/*/libseedfaker_ffi.dylib; do
  if [ -f "$f" ]; then
    RUBY_HASH=$(sha256sum "$f" | cut -d' ' -f1)
    echo "  ruby: ${RUBY_HASH:0:16}..."
    break
  fi
done
if [ -n "$RUBY_HASH" ]; then
  sedi "/@checksum-start/,/@checksum-end/c\\
  # @checksum-start\\
  NATIVE_CHECKSUM = \"${RUBY_HASH}\"\\
  # @checksum-end" "$ROOT/packages/ruby/lib/seedfaker.rb"
fi

echo "Done."
