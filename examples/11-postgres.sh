#!/usr/bin/env bash
# PostgreSQL: seed tables with CSV COPY and SQL INSERT
# Requires: docker compose up -d (from examples/ directory)
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"

PG="psql -h localhost -p 15432 -U test -d testdb"
export PGPASSWORD=test

echo "=== Starting PostgreSQL ==="
docker compose up -d --wait postgres 2>/dev/null

echo ""
echo "=== Create table ==="
$PG -c "DROP TABLE IF EXISTS users;
CREATE TABLE users (name TEXT, email TEXT, phone TEXT, ssn TEXT);"

echo ""
echo "=== Seed 1000 rows via CSV COPY ==="
${SF} name email phone ssn --format csv -n 1000 --until 2025 --seed pg-demo \
    | $PG -c "COPY users(name,email,phone,ssn) FROM STDIN CSV HEADER"
$PG -c "SELECT count(*) AS total_rows FROM users;"

echo ""
echo "=== Sample rows ==="
$PG -c "SELECT * FROM users LIMIT 5;"

echo ""
echo "=== Seed another table via SQL INSERT ==="
$PG -c "DROP TABLE IF EXISTS employees;
CREATE TABLE employees (name TEXT, email TEXT, job_title TEXT);"
${SF} name email job-title --format sql=employees -n 100 --until 2025 --seed pg-emp \
    | $PG 2>/dev/null
$PG -c "SELECT count(*) AS total_rows FROM employees;"
$PG -c "SELECT * FROM employees LIMIT 5;"

echo ""
echo "=== Stopping PostgreSQL ==="
docker compose stop postgres 2>/dev/null
