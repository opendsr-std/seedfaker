#!/usr/bin/env bash
set -euo pipefail
SF="${1:?Usage: test-mcp.sh <path-to-seedfaker-binary>}"
echo "--- MCP server ---"

INIT=$(printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}\n' | "$SF" mcp 2>/dev/null)
echo "$INIT" | grep -q '"protocolVersion"' || { echo "FAIL: initialize"; exit 1; }
echo "  ok: initialize"

TOOLS=$(printf '{"jsonrpc":"2.0","id":2,"method":"tools/list"}\n' | "$SF" mcp 2>/dev/null)
for tool in field run_preset fingerprint list_fields; do
  echo "$TOOLS" | grep -q "\"$tool\"" || { echo "FAIL: tools/list missing $tool"; exit 1; }
done
echo "  ok: tools/list (field, run_preset, fingerprint, list_fields)"

GEN=$(printf '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"field","arguments":{"fields":["name","email"],"n":3,"seed":"mcp-test"}}}\n' | "$SF" mcp 2>/dev/null)
echo "$GEN" | grep -q '"text"' || { echo "FAIL: field"; exit 1; }
echo "  ok: field (3 records)"

FP=$(printf '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"fingerprint","arguments":{}}}\n' | "$SF" mcp 2>/dev/null)
echo "$FP" | grep -q 'sf0-' || { echo "FAIL: fingerprint"; exit 1; }
echo "  ok: fingerprint"

PRESET=$(printf '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"run_preset","arguments":{"preset":"nginx","n":2,"seed":"mcp-test"}}}\n' | "$SF" mcp 2>/dev/null)
echo "$PRESET" | grep -q '"text"' || { echo "FAIL: run_preset"; exit 1; }
echo "  ok: run_preset nginx"
