#!/usr/bin/env bash
# --annotated emits JSONL where every generated value carries a byte-offset
# span (s, e), a field label (f), and the value (v). Suitable as ground-truth
# for NER / PII training and for benchmarking detectors.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# 1. Structured rows, one record per line; spans cover each column.
${SF} name email phone:e164 --annotated --format csv -n 2 --seed ann --until 2025

echo
# 2. Natural-language ticket text (pii-leak preset). Spans still track byte
#    positions inside the rendered sentences.
${SF} run pii-leak --annotated -n 2 --seed ann --until 2025

echo
# 3. With --corrupt, each span also carries `o` — the pre-corruption original.
#    Train on text, evaluate recall against `o`.
${SF} run pii-leak --annotated --corrupt high -n 2 --seed ann --until 2025
