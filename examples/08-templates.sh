#!/usr/bin/env bash
# Templates: .env, SQL UPDATE, log lines, terraform, API payloads
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

echo "=== .env file ==="
${SF} -t 'DB_PASSWORD={{password}}
AWS_ACCESS_KEY_ID={{aws-access-key}}
AWS_SECRET_ACCESS_KEY={{aws-secret-key}}
API_KEY={{api-key}}' -n 1 --until 2025 --seed env

echo ""
echo "=== SQL UPDATE statements ==="
${SF} -t "UPDATE users SET email='{{email}}' WHERE id={{serial}};" -n 5 --until 2025 --seed upd

echo ""
echo "=== Access log lines ==="
${SF} -t '{{ip}} - {{username}} [01/Jan/2025:12:00:00 +0000] "GET /api/users HTTP/1.1" 200' -n 5 --until 2025 --seed log

echo ""
echo "=== JSON payloads ==="
${SF} -t '{"name":"{{name}}","email":"{{email}}","phone":"{{phone}}"}' -n 3 --until 2025 --seed api

echo ""
echo "=== Terraform resources ==="
${SF} -t 'resource "aws_iam_user" "user_{{serial}}" {
  name = "{{username}}"
  tags = { email = "{{email}}" }
}' -n 2 --until 2025 --seed tf
