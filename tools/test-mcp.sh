#!/usr/bin/env bash
set -euo pipefail
SF="${1:?Usage: test-mcp.sh <path-to-seedfaker-binary>}"
echo "--- MCP server ---"

INIT=$(printf '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}\n' | "$SF" mcp 2>/dev/null)
echo "$INIT" | grep -q '"protocolVersion"' || { echo "FAIL: initialize"; exit 1; }
echo "  ok: initialize"

TOOLS=$(printf '{"jsonrpc":"2.0","id":2,"method":"tools/list"}\n' | "$SF" mcp 2>/dev/null)
for tool in generate run_preset fingerprint list_fields list_presets validate; do
  echo "$TOOLS" | grep -q "\"$tool\"" || { echo "FAIL: tools/list missing $tool"; exit 1; }
done
echo "  ok: tools/list (generate, run_preset, validate, list_fields, list_presets, fingerprint)"

GEN=$(printf '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"generate","arguments":{"fields":["name","email"],"n":3,"seed":"mcp-test"}}}\n' | "$SF" mcp 2>/dev/null)
echo "$GEN" | grep -q '"text"' || { echo "FAIL: generate"; exit 1; }
echo "  ok: generate (3 records)"

FP=$(printf '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"fingerprint","arguments":{}}}\n' | "$SF" mcp 2>/dev/null)
echo "$FP" | grep -q 'sf0-' || { echo "FAIL: fingerprint"; exit 1; }
echo "  ok: fingerprint"

PRESET=$(printf '{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"run_preset","arguments":{"preset":"nginx","n":2,"seed":"mcp-test"}}}\n' | "$SF" mcp 2>/dev/null)
echo "$PRESET" | grep -q '"text"' || { echo "FAIL: run_preset"; exit 1; }
echo "  ok: run_preset nginx"

PING=$(printf '{"jsonrpc":"2.0","id":6,"method":"ping"}\n' | "$SF" mcp 2>/dev/null)
echo "$PING" | grep -q '"result"' || { echo "FAIL: ping"; exit 1; }
echo "  ok: ping"

VAL=$(printf '{"jsonrpc":"2.0","id":7,"method":"tools/call","params":{"name":"validate","arguments":{"fields":["name","email"]}}}\n' | "$SF" mcp 2>/dev/null)
echo "$VAL" | grep -q '"text"' || { echo "FAIL: validate"; exit 1; }
echo "  ok: validate"

PRESETS=$(printf '{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"list_presets","arguments":{}}}\n' | "$SF" mcp 2>/dev/null)
echo "$PRESETS" | grep -q 'nginx' || { echo "FAIL: list_presets"; exit 1; }
echo "  ok: list_presets"

# Fail-Fast: invalid ctx must error, not silent fallback
INVCTX=$(printf '{"jsonrpc":"2.0","id":9,"method":"tools/call","params":{"name":"generate","arguments":{"fields":["name"],"seed":"t","ctx":"typo"}}}\n' | "$SF" mcp 2>/dev/null)
echo "$INVCTX" | grep -q '"error"' || { echo "FAIL: invalid ctx should error"; exit 1; }
echo "  ok: invalid ctx rejected"

# Fail-Fast: missing seed must error
NOSEED=$(printf '{"jsonrpc":"2.0","id":10,"method":"tools/call","params":{"name":"generate","arguments":{"fields":["name"]}}}\n' | "$SF" mcp 2>/dev/null)
echo "$NOSEED" | grep -q '"error"' || { echo "FAIL: missing seed should error"; exit 1; }
echo "  ok: missing seed rejected"
