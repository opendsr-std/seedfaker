#!/usr/bin/env bash
# Credential and secret fields. Useful for testing secret scanners, log
# redaction, and vault integrations without exposing real credentials.
# Credit cards pass Luhn; SSH keys have valid OpenSSH framing; JWTs have
# parseable headers.
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

# One of each, full value.
for type in jwt aws-access-key stripe-key github-pat openai-key slack-bot-token \
            sentry-dsn connection-string; do
  printf "%-22s %s\n" "$type" "$(${SF} ${type} -n 1 --seed cred --until 2025)"
done

echo
echo "--- Luhn-valid credit cards (5) ---"
${SF} credit-card -n 5 --seed cards --until 2025

echo
# `auth` field group bundles the common secret shapes into one record.
echo "--- auth group (csv) ---"
${SF} auth --format csv -n 3 --seed scan --until 2025
