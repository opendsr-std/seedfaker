#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="$ROOT/rust/target/release/seedfaker"

test -f "$SF" || { echo "Run 'make dev' first to build artifacts."; exit 1; }

echo "Installing seedfaker to host system..."
mkdir -p "$HOME/.local/bin"
cp "$SF" "$HOME/.local/bin/seedfaker"
echo "  CLI  → $HOME/.local/bin/seedfaker"

cd "$ROOT/packages/npm" && pnpm link --global 2>/dev/null \
  && echo "  npm  → @opendsr/seedfaker (linked)" \
  || echo "  npm  → skipped (pnpm not found)"
# pnpm link may overwrite seedfaker_napi.node with a platform-mismatched binary — rebuild.
make -C "$ROOT" build-napi --no-print-directory 2>/dev/null || true

cd "$ROOT/packages/pip" && python3 -m pip install -e . --break-system-packages --quiet 2>/dev/null \
  && echo "  pip  → seedfaker (editable)" \
  || { python3 -m pip install -e . --user --quiet 2>/dev/null \
       && echo "  pip  → seedfaker (--user)" \
       || echo "  pip  → skipped (pip not available)"; }

echo ""
echo "Installed:"
printf "  cli: "; "$HOME/.local/bin/seedfaker" --version 2>/dev/null || echo "not found"
printf "  npm: "; node -e "console.log(require('$ROOT/packages/npm/package.json').version)" 2>/dev/null || echo "not linked"
printf "  pip: "; python3 -c "import seedfaker;print(seedfaker.__version__)" 2>/dev/null || echo "not installed"
