# Seed a database with realistic test data

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Pipe synthetic data directly into PostgreSQL or MySQL. No intermediate files, constant memory, works at any scale.

For GB/TB-scale bulk loads — parallel COPY, UNLOGGED tables, database tuning — see [Seed a large database](seed-large-database.md).

## Contents

- [Single table](#single-table)
- [Multi-table with FK](#multi-table-with-fk)
- [Pipe tables in order](#pipe-tables-in-order)
- [CSV + COPY for bulk load](#csv--copy-for-bulk-load)
- [Full example](#full-example)
- [Determinism](#determinism)

## Single table

Pipe SQL INSERT statements directly into PostgreSQL:

```bash
seedfaker name email phone --format sql=users -n 1000000 --seed staging --until 2025 | psql mydb
```

```
INSERT INTO users (name, email, phone) VALUES ('Atharva Bhat', 'im.marina@gmail.com', '+48 557 556 731');
INSERT INTO users (name, email, phone) VALUES ('Brooklyn Stewart', 'ohayonz6@mobileye.com', '+40 282 638202');
...
```

Streams directly, no temp files, constant memory.

For MySQL:

```bash
seedfaker name email phone --format sql=users -n 1000000 --seed staging --until 2025 | mysql mydb
```

## Multi-table with FK

Define related tables in a YAML config:

```yaml
# shop.yaml
options:
  seed: staging
  since: "2024-01-01"
  until: "2025"

users:
  columns:
    id: uuid
    name: name
    email: email:xuniq
    phone: phone:e164:omit=20
  options:
    count: 50000
    ctx: strict

orders:
  columns:
    id: serial
    user_id: users.id:zipf
    user_name: user_id->name
    user_email: user_id->email
    status: enum:completed=60,pending=25,shipped=10,cancelled=5
    amount: amount:plain:1..5000
  options:
    count: 500000

order_items:
  columns:
    id: serial
    order_id: orders.id:zipf
    product_name: company-name
    unit_price: amount:1..500:plain
    qty: integer:1..20
    line_total: unit_price * qty
  options:
    count: 1200000
```

- `users.id:zipf` — [FK anchor](../docs/multi-table.md#fk-anchor-tablecolumn) with [Zipf distribution](../docs/fields.md#zipf-distribution)
- `user_id->name` — [FK dereference](../docs/multi-table.md#fk-dereference-anchor-column): same parent row
- `unit_price * qty` — [expression](../docs/expressions.md)
- [`ctx: strict`](../docs/context.md) — name, email, phone belong to one identity
- [`:omit=20`](../docs/fields.md) — 20% of phone values NULL

## Pipe tables in order

Stream each table directly into the database — parents first, then children:

```bash
seedfaker run shop.yaml --table users --format sql=users | psql mydb
seedfaker run shop.yaml --table orders --format sql=orders | psql mydb
seedfaker run shop.yaml --table order_items --format sql=order_items | psql mydb
```

```
-- users:
INSERT INTO users (id, name, email, phone) VALUES ('0', 'Mason', 'stevenfraser@fastmail.com', '+17435404792');
INSERT INTO users (id, name, email, phone) VALUES ('1', 'Tariq', 'maureeny2@mail.com', '+14528956788');
...

-- orders (FK references valid user IDs):
INSERT INTO orders (id, amount, user_id, user_name, user_email, status) VALUES ('0', '295.81', '1', 'Tariq', 'maureeny2@mail.com', 'completed');
INSERT INTO orders (id, amount, user_id, user_name, user_email, status) VALUES ('1', '276.97', '2', 'Wei', 'gm05@icloud.com', 'pending');
...
```

No intermediate files, constant memory.

## CSV + COPY for bulk load

`\COPY` with CSV is faster than INSERT for large volumes:

```bash
seedfaker run shop.yaml --table users --format csv \
  | psql mydb -c "\COPY users(id, name, email, phone) FROM STDIN WITH (FORMAT csv, HEADER true)"
```

Same streaming approach — no temp files.

## Full example

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT
);

CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    user_name TEXT NOT NULL,
    user_email TEXT NOT NULL,
    status TEXT NOT NULL,
    amount NUMERIC(10,2) NOT NULL
);
```

```bash
seedfaker run shop.yaml --table users --format sql=users | psql mydb
seedfaker run shop.yaml --table orders --format sql=orders | psql mydb

psql mydb -c "SELECT count(*) FROM users"    -- 50000
psql mydb -c "SELECT count(*) FROM orders"   -- 500000
```

## Determinism

With `seed` and `until` pinned in the config:

- Every developer runs the same commands, gets identical rows
- Rebuild staging after schema changes — same data
- Adding a column does not change existing columns

Commit `shop.yaml` to version control.

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
