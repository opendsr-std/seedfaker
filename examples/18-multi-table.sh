#!/usr/bin/env bash
# Multi-table: FK anchor, dereference, zipf, expressions, aggregators
set -euo pipefail
SF="${SEEDFAKER:-seedfaker}"
TMPDIR=$(mktemp -d)

cat > "$TMPDIR/shop.yaml" << 'CONFIG'
options:
  seed: "shop"
  locale: [en]

users:
  columns:
    id: serial
    name: first-name
    email: email
    phone: phone:e164:omit=30
  options:
    count: 5

products:
  columns:
    id: serial
    title: company-name
    unit_price: amount:plain:10..500
  options:
    count: 4

orders:
  columns:
    id: serial
    customer_id: users.id:zipf
    customer_name: customer_id->name
    product_id: products.id
    product_title: product_id->title
    qty: integer:1..5
    unit_price: product_id->unit_price
    line_total: unit_price * qty
    user_order_num: unit_price:count=customer_id
    user_total_spent: line_total:sum=customer_id
  options:
    count: 15
CONFIG

echo "=== users ==="
${SF} run "$TMPDIR/shop.yaml" --table users --format csv

echo ""
echo "=== products ==="
${SF} run "$TMPDIR/shop.yaml" --table products --format csv

echo ""
echo "=== orders ==="
${SF} run "$TMPDIR/shop.yaml" --table orders --format csv

echo ""
echo "=== --all --output-dir ==="
${SF} run "$TMPDIR/shop.yaml" --all --output-dir "$TMPDIR/out" --format csv
wc -l "$TMPDIR/out"/*.csv

rm -rf "$TMPDIR"
