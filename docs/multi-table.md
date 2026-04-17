# Multi-table configs

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Define multiple tables in one YAML file with foreign key relationships, Zipf distribution, and computed fields.

## Contents

- [Example](#example)
- [FK anchor](#fk-anchor-tablecolumn) — reference a parent table column
- [FK dereference](#fk-dereference-anchor-column) — reuse the same parent row
- [Running](#running) — `--table`, `--all --output-dir`
- [Options merge](#options-merge) — root → per-table → CLI
- [Self-referencing FK](#self-referencing-fk) — hierarchical data within one table
- [Constraints](#constraints)

## Example

```yaml
options:
  seed: "shop"
  since: "2023-01-01"
  until: "2025-06-01"

users:
  columns:
    id: uuid
    first_name: first-name
    last_name: last-name
    email: email
    phone: phone:e164:omit=15
  options:
    count: 50000

orders:
  columns:
    id: serial
    customer_id: users.id:zipf
    customer_name: customer_id->first_name
    customer_email: customer_id->email
    total: amount:usd:1..5000
    status: enum:completed=65,pending=20,cancelled=15
  options:
    count: 500000
```

A config with multiple top-level keys (other than `columns`, `template`, `options`) defines multiple tables. Each table has its own `columns` and optional `options`. Root-level `options` apply as defaults; per-table options override them.

## FK anchor: `table.column`

`users.id` generates a value from the parent table's `id` column for a sampled parent row. The row index is chosen deterministically from the parent's `count`.

Add `:zipf` or `:zipf=N` for power-law distribution (most child rows reference a few popular parents). Without it, the distribution is uniform.

## FK dereference: `anchor->column`

`customer_id->first_name` reuses the same parent row selected by the anchor column and generates the parent's `first_name` for that row. Multiple dereferences from the same anchor always produce values from the same parent row.

FK dereference is pure recomputation: it derives the parent's RNG from the same seed + row index and generates the field value. No data is stored or looked up.

This is unrelated to `--ctx strict`. Context mode correlates fields _within a single row_ (name, email, username share one identity). FK dereference correlates fields _across tables_ (child row gets parent row's values). They operate independently and can be combined.

## Running

```bash
# One table to stdout
seedfaker run shop.yaml --table orders -n 100

# All tables to files
seedfaker run shop.yaml --all --output-dir ./data/ --format csv

# CLI overrides apply to the selected table
seedfaker run shop.yaml --table users --seed newseed --format jsonl
```

`--table TABLE` generates one table to stdout. `--all --output-dir DIR` generates all tables to files named `{table}.{ext}`. Tables are generated in dependency order (parents first).

### Parallel generation of one big table

`--shard I/N` splits a single table's row range into N disjoint slices. Run N `seedfaker --table X --shard i/N` in parallel and pipe each into its own `\COPY` — Postgres accepts concurrent `COPY` into one table. Details and examples: [CLI § Sharding](cli.md#sharding), [guides/seed-large-database](../guides/seed-large-database.md).

## Options merge

Options resolve in three layers: root config → per-table config → CLI flags. CLI always wins.

```yaml
options: # root defaults
  locale: [en, de]
  since: "2020-01-01"

users:
  columns: { ... }
  options: # per-table overrides
    count: 50000
    locale: [en] # overrides root locale for this table
```

## Self-referencing FK

A table can reference itself for hierarchical relationships:

```yaml
employees:
  columns:
    id: serial
    name: first-name
    email: email
    manager_id: employees.id
    manager_name: manager_id->name
  options:
    count: 100
```

Each `manager_id` points to a valid row in the same `employees` table. `manager_name` resolves to that row's `name` value.

The difference from `--ctx strict`: context mode links fields _within one row_ by sharing an identity seed. Self-referencing FK links fields _across rows_ by recomputing the referenced row's generation.

## Constraints

- FK anchor columns must reference non-FK columns in the parent table (no chained FKs)
- Parent table must have `count` in its options
- Circular FK dependencies between tables are rejected
- Self-referencing tables are allowed
- Reserved names (`columns`, `template`, `options`) cannot be used as table names

## Related guides

- [Seed a database](../guides/seed-database.md) — seed a Postgres/MySQL staging DB
- [Seed a large database](../guides/seed-large-database.md) — GB/TB-scale bulk load with parallel COPY

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
