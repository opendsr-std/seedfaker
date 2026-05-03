#!/usr/bin/env bash
# --ctx strict: one identity + one locale per row (name, email, phone CC, gov ID match).
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

echo "--- without --ctx: fields drawn independently ---"
${SF} name email phone ssn --locale de,fr,ja -n 5 --seed ctx --until 2025

echo
echo "--- with --ctx strict: one identity + one locale per row ---"
${SF} name email phone ssn --locale de,fr,ja --ctx strict -n 5 --seed ctx --until 2025

echo
echo "--- --ctx strict + --abc native: locale-native scripts ---"
${SF} name phone country-code --locale de,ja,uk,zh --ctx strict --abc native \
  -n 5 --seed ctx --until 2025
