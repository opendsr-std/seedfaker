#!/usr/bin/env bash
set -euo pipefail

BENCH_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$BENCH_DIR/.." && pwd)"
VENV_DIR="$BENCH_DIR/.venv"

echo "=== Python venv ==="
python3 -m venv "$VENV_DIR"
source "$VENV_DIR/bin/activate"
pip install --upgrade pip
pip install faker mimesis polyfactory
pip install "$PROJECT_DIR/packages/pip"

echo ""
echo "=== Node.js ==="
cd "$BENCH_DIR/node"
pnpm install

echo ""
echo "=== CLI tools ==="
go install github.com/lucapette/fakedata@latest || brew install lucapette/tap/fakedata || echo "fakedata: install from https://github.com/lucapette/fakedata"

echo ""
echo "=== seedfaker ==="
cd "$PROJECT_DIR"
make build-local

echo ""
echo "Installed:"
echo "  venv:     $VENV_DIR"
echo "  node:     $BENCH_DIR/node/node_modules"
echo "  fakedata: $(which fakedata 2>/dev/null || echo 'not found')"
echo ""
echo "Run: make bench"
