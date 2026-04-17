#!/usr/bin/env bash
# MCP server demo — shows the JSON-RPC protocol exchange
#
# seedfaker mcp reads JSON-RPC requests from stdin, writes responses to stdout.
# This is the protocol AI tools (Claude Code, Cursor, etc.) use to call seedfaker.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

pj() { python3 -m json.tool --no-ensure-ascii; }
# Extract the text payload from a tools/call response
extract_text() { python3 -c "import sys,json; print(json.loads(sys.stdin.read())['result']['content'][0]['text'])"; }

echo "=== 1. Initialize (handshake) ==="
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  | ${SF} mcp | pj

echo ""
echo "=== 2. List available tools ==="
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' \
  | ${SF} mcp | pj

echo ""
echo "=== 3. Generate 3 correlated records ==="
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"field","arguments":{"fields":["name","email","phone:e164"],"n":3,"seed":"demo","ctx":"strict","locale":"en"}}}' \
  | ${SF} mcp | tail -1 | extract_text

echo ""
echo "=== 4. Generate with corruption ==="
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"field","arguments":{"fields":["name","email","ssn"],"n":3,"seed":"demo","corrupt":"high","locale":"en"}}}' \
  | ${SF} mcp | tail -1 | extract_text

echo ""
echo "=== 5. List fields (summary) ==="
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_fields","arguments":{}}}' \
  | ${SF} mcp | tail -1 | extract_text | python3 -c "
import sys, json
data = json.loads(sys.stdin.read())
for g in data['groups'][:3]:
    print(f\"{g['group']}: {', '.join(f['name'] for f in g['fields'])}\")
print(f\"... {data['total_fields']} fields total, {len(data['locales'])} locales\")
"
