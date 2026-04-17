#!/usr/bin/env bash
# --corrupt injects 15 noise types into output text. Four levels. Deterministic:
# same seed + same level = same corrupted bytes.
#
# Use `high` for training augmentation, `extreme` for red-team / robustness evals.
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

# With --annotated, each span carries `o` with the pre-corruption original,
# so a detector can be scored on recall against the clean value.
echo "--- annotated + corrupt high (one record) ---"
${SF} name email ssn --annotated --corrupt high -n 1 --seed cr --until 2025
