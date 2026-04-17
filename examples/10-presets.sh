#!/usr/bin/env bash
# Presets: ready-made configs embedded in the binary
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

echo "=== Available presets ==="
${SF} run --list

echo ""
echo "=== nginx access log ==="
${SF} run nginx -n 3 --until 2025 --seed demo

echo ""
echo "=== payment transactions (JSONL) ==="
${SF} run payment -n 3 --until 2025 --seed demo

echo ""
echo "=== auth.log events ==="
${SF} run auth -n 3 --until 2025 --seed demo
