#!/usr/bin/env bash
# Custom YAML configs: structured data, templates, ground truth
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
TMPDIR=$(mktemp -d)

echo "=== Structured output (JSONL) ==="
cat > "$TMPDIR/users.yaml" << 'CONFIG'
columns:
  name: name
  email: email
  phone: phone:e164
  role: enum:admin,user,viewer

options:
  ctx: strict
CONFIG
${SF} run "$TMPDIR/users.yaml" -n 3 --until 2025 --seed demo --format jsonl

echo ""
echo "=== Template output ==="
cat > "$TMPDIR/alert.yaml" << 'CONFIG'
columns:
  ts: timestamp:log
  host: dns-record
  level: enum:CRITICAL,ERROR,WARN
  msg: message

template: "[{{ts}}] {{level}} {{host}}: {{msg}}"
CONFIG
${SF} run "$TMPDIR/alert.yaml" -n 3 --until 2025 --seed demo

echo ""
echo "=== Annotated output (corruption + spans) ==="
cat > "$TMPDIR/annotated.yaml" << 'CONFIG'
columns:
  name: name
  email: email
  ssn: ssn

options:
  corrupt: high

template: "{{name}} <{{email}}> SSN:{{ssn}}"
CONFIG
${SF} run "$TMPDIR/annotated.yaml" -n 3 --until 2025 --seed demo --annotated

rm -rf "$TMPDIR"
