#!/usr/bin/env bash
# --annotated: JSONL with byte-spans (s,e), field label (f), value (v).
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# Structured rows.
${SF} name email phone:e164 --annotated --format csv -n 2 --seed ann --until 2025

echo
# Natural-language text (pii-leak preset). Spans track byte positions in rendered text.
${SF} run pii-leak --annotated -n 2 --seed ann --until 2025

echo
# --corrupt: each span carries `o` = pre-corruption original.
${SF} run pii-leak --annotated --corrupt high -n 2 --seed ann --until 2025
