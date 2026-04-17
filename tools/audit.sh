#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
export PATH="$HOME/.cargo/bin:$PATH"

FAIL=0
fail() { echo "FAIL: $1"; FAIL=1; }

# Rust: advisories, licenses, bans, sources
(cd rust && cargo deny check) || fail "cargo deny"

# Rust: no deprecated crates
DEP=$(cd rust && cargo tree --workspace --prefix none --format '{p}' 2>/dev/null | grep -i deprecated || true)
[ -n "$DEP" ] && fail "deprecated: $DEP"

# Rust: lockfile in sync
(cd rust && cargo update --locked --dry-run >/dev/null 2>&1) || fail "Cargo.lock out of sync"

# pnpm: vulnerabilities
pnpm audit || fail "pnpm audit"

[ "$FAIL" -ne 0 ] && exit 1
echo "audit ok"
