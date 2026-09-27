#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."

MISSING=0
need() {
  if command -v "$1" >/dev/null 2>&1; then
    printf "  ok       %s\n" "$1"
  else
    printf "  MISSING  %s — %s\n" "$1" "$2"
    MISSING=1
  fi
}

echo "--- toolchains"
need rustup "https://rustup.rs (version pinned in rust/rust-toolchain.toml)"
need node "nvm install (version in .nvmrc)"
need pnpm "corepack enable"
need php "brew install php (FFI extension required)"
need ruby "brew install ruby"
need go "brew install go"
need shellcheck "brew install shellcheck"
need taplo "cargo install taplo-cli --locked"
need cargo-deny "cargo install cargo-deny --locked"
need prettier "pnpm add -g prettier"
need eslint "pnpm add -g eslint@9"

NODE_MAJOR=$(node -p 'process.versions.node.split(".")[0]' 2>/dev/null || echo 0)
if [ "$NODE_MAJOR" -lt 22 ]; then
  echo "  WARN     node $(node --version 2>/dev/null || echo none) < 22 — run: nvm use"
  MISSING=1
fi
if ! grep -qx FFI <<<"$(php -m 2>/dev/null)"; then
  echo "  WARN     php FFI extension is not enabled"
  MISSING=1
fi

echo "--- rust toolchain"
(cd rust && rustup show active-toolchain)

echo "--- python venv (.venv)"
PY="${PYTHON:-python3.12}"
command -v "$PY" >/dev/null 2>&1 || PY=python3
[ -x .venv/bin/python3 ] || "$PY" -m venv .venv
.venv/bin/pip install --quiet --upgrade pip
.venv/bin/pip install --quiet -r requirements-dev.txt
echo "  ok       $(.venv/bin/python3 --version), ruff $(.venv/bin/ruff --version | cut -d' ' -f2)"

echo ""
if [ "$MISSING" -ne 0 ]; then
  echo "Some tools are missing — see above."
  exit 1
fi
echo "Ready. In each shell: nvm use && source .venv/bin/activate"
