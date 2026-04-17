#!/usr/bin/env bash
# Generate records from a list of field names. Output is tab-separated.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

${SF} email -n 5 --seed demo --until 2025

echo
${SF} name email phone -n 5 --seed demo --until 2025

# Field groups bundle common sets. `person` expands to name, email, phone,
# birthdate, gender, address, etc.
echo
${SF} person -n 3 --seed demo --until 2025
