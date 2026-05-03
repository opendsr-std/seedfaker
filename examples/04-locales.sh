#!/usr/bin/env bash
# Locale-aware fields. --abc native = non-Latin script when locale has one.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

${SF} name address phone --locale de -n 3 --seed loc --until 2025

echo
${SF} name address phone --locale ja --abc native -n 3 --seed loc --until 2025

echo
${SF} name address phone --locale zh --abc native -n 3 --seed loc --until 2025

echo
# Weighted locale mix.
${SF} name phone country-code --locale en=7,de=2,fr=1 -n 5 --seed loc --until 2025

echo
# national-id dispatches by locale.
for loc in en de fr pt-br hi zh gb; do
  printf "  %-6s %s\n" "$loc" "$(${SF} national-id --locale "$loc" -n 1 --seed gov --until 2025)"
done
