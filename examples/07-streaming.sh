#!/usr/bin/env bash
# -n 0 = unlimited stream (constant memory). --rate N = cap emit to N/sec.
# Both combine with any field, format, or config.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# Head of an unlimited stream.
${SF} email -n 0 --seed stream --until 2025 | head -5

echo
# 5 records at ~2/s takes ~2.5s — observable rate limit.
time ${SF} email -n 5 --rate 2 --seed rate --until 2025

echo
# Uniqueness at scale: default fields have enough entropy for millions.
${SF} email -n 10000 --seed uniq --until 2025 | sort -u | wc -l
