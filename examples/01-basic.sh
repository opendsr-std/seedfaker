#!/usr/bin/env bash
# Generate from a list of field names.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

${SF} email -n 5 --seed demo --until 2025

echo
${SF} name email phone -n 5 --seed demo --until 2025

echo
# `person` field group → name, email, phone, birthdate, gender, address, ...
${SF} person -n 3 --seed demo --until 2025
