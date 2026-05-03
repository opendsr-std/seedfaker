#!/usr/bin/env bash
# --corrupt: 4 levels (low|mid|high|extreme). Same seed + same level → same bytes.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
CMD="name email phone --format csv -n 4 --seed cr --until 2025"

for level in "" low mid high extreme; do
  label=${level:-off}
  echo "--- --corrupt $label ---"
  if [ -z "$level" ]; then
    ${SF} ${CMD}
  else
    ${SF} ${CMD} --corrupt ${level}
  fi
  echo
done

# --annotated: span carries `o` = pre-corruption original.
echo "--- annotated + corrupt high (one record) ---"
${SF} name email ssn --annotated --corrupt high -n 1 --seed cr --until 2025
