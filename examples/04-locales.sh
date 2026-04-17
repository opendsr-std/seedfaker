#!/usr/bin/env bash
# Locale-aware fields: names, addresses, phones, and gov IDs all dispatch
# by the --locale flag. --abc native outputs non-Latin scripts where the
# locale has one (kanji, hanzi, Cyrillic, Arabic).
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

${SF} name address phone --locale de -n 3 --seed loc --until 2025

echo
${SF} name address phone --locale ja --abc native -n 3 --seed loc --until 2025

echo
${SF} name address phone --locale zh --abc native -n 3 --seed loc --until 2025

echo
# Weighted mix. Each row picks a locale from the weighted pool.
${SF} name phone country-code --locale en=7,de=2,fr=1 -n 5 --seed loc --until 2025

echo
# national-id dispatches by locale — SSN (en), CPF (pt-br), NINO (gb), etc.
echo "national-id by locale:"
for loc in en de fr pt-br hi zh gb; do
  printf "  %-6s %s\n" "$loc" "$(${SF} national-id --locale "$loc" -n 1 --seed gov --until 2025)"
done
