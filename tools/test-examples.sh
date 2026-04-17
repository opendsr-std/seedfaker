#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SF="${SEEDFAKER:-$ROOT/rust/target/release/seedfaker}"

echo "--- shell examples ---"
SEEDFAKER="$SF" /bin/bash "$ROOT/examples/run-all.sh" --no-docker

echo "--- python ---"
PYTHONPATH="$ROOT/packages/pip" python3 "$ROOT/examples/python/basic.py" > /dev/null && echo "  ok: examples/python/basic.py"

echo "--- node ---"
mkdir -p "$ROOT/node_modules/@opendsr" && ln -sfn "$ROOT/packages/npm" "$ROOT/node_modules/@opendsr/seedfaker"
node "$ROOT/examples/nodejs/basic.cjs" > /dev/null && echo "  ok: examples/nodejs/basic.cjs"

echo "--- php ---"
php "$ROOT/examples/php/basic.php" > /dev/null && echo "  ok: examples/php/basic.php"

echo "--- ruby ---"
RUBYLIB="$ROOT/packages/ruby/lib" ruby "$ROOT/examples/ruby/basic.rb" > /dev/null && echo "  ok: examples/ruby/basic.rb"

echo "--- go ---"
cd "$ROOT/examples/go" && CGO_LDFLAGS="-L../../rust/target/release -lseedfaker_ffi" LD_LIBRARY_PATH="../../rust/target/release" go run main.go > /dev/null && echo "  ok: examples/go/main.go"
