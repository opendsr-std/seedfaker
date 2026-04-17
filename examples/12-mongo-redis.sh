#!/usr/bin/env bash
# MongoDB + Redis: seed documents and session cache
# Requires: docker compose up -d (from examples/ directory)
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

MONGO="docker compose exec -T mongo"
REDIS="docker compose exec -T redis"

echo "=== Starting MongoDB + Redis ==="
docker compose up -d --wait mongo redis 2>/dev/null

echo ""
echo "=== MongoDB: import 500 user documents ==="
${SF} name email phone address --format jsonl -n 500 --until 2025 --seed mongo-demo \
    | $MONGO mongoimport --db demo --collection users --drop 2>/dev/null
$MONGO mongosh --quiet --eval \
    "print('documents:', db.getSiblingDB('demo').users.countDocuments())"

echo ""
echo "=== MongoDB: sample documents ==="
$MONGO mongosh --quiet --eval \
    "db.getSiblingDB('demo').users.find().limit(3).forEach(d => printjson(d))"

echo ""
echo "=== Redis: seed 50 session tokens ==="
${SF} email jwt -n 50 --until 2025 --seed redis-demo | while IFS=$'\t' read -r email token; do
    $REDIS redis-cli SET "session:${email}" "${token}" EX 3600 > /dev/null 2>&1
done
echo "keys: $($REDIS redis-cli DBSIZE 2>/dev/null | awk '{print $2}')"

echo ""
echo "=== Redis: sample keys ==="
$REDIS redis-cli KEYS 'session:*' 2>/dev/null | head -5

echo ""
echo "=== Stopping MongoDB + Redis ==="
docker compose stop mongo redis 2>/dev/null
