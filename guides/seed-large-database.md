# Seed a large database

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

GB–TB bulk load into PostgreSQL (adapts to MySQL, ClickHouse, SQLite). seedfaker on the host, DB in a throwaway container, data through `COPY FROM STDIN` — no intermediate files.

For sub-GB staging use [seed-database](seed-database.md).

## Contents

- [Two phases](#two-phases)
- [Phase 1: bulk load](#phase-1-bulk-load)
- [Phase 2: post-load](#phase-2-post-load)
- [Parallelism](#parallelism)
- [Postgres tuning](#postgres-tuning)
- [Reference benchmark](#reference-benchmark)
- [Other engines](#other-engines)

## Two phases

1. **Load into unconstrained heap tables.** No PK, FK, CHECK, UNIQUE, or indexes. seedfaker guarantees id uniqueness (`serial`/`uuid`), so server-side verification is wasted work.
2. **Add constraints and indexes against the populated heap.** Each scans once; incremental per-row maintenance during `COPY` is orders of magnitude slower.

## Phase 1: bulk load

```sql
CREATE UNLOGGED TABLE users (
    id UUID NOT NULL, first_name TEXT NOT NULL, last_name TEXT NOT NULL,
    email TEXT NOT NULL, created_at TIMESTAMPTZ NOT NULL
);
```

`UNLOGGED` skips the WAL. Revert with `ALTER TABLE ... SET LOGGED` in phase 2.

seedfaker emits columns in generation order (non-FK first, FK-derived after), not DDL order. Fetch the header once and pass it to `\COPY`:

```bash
hdr=$(seedfaker run users.yaml --table users --format csv -n 1 2>/dev/null | head -1)
seedfaker run users.yaml --table users --format csv \
  | psql "$PGURL" -q -c "\COPY users ($hdr) FROM STDIN WITH (FORMAT csv, HEADER true)"
```

## Phase 2: post-load

```sql
ALTER TABLE users SET LOGGED;
ALTER TABLE users ADD PRIMARY KEY (id);
CREATE INDEX ON orders (user_id);
ALTER TABLE orders ADD CONSTRAINT orders_user_fk
    FOREIGN KEY (user_id) REFERENCES users(id) NOT VALID;
ANALYZE;
```

`NOT VALID` accepts the FK without scanning existing rows; run `ALTER TABLE ... VALIDATE CONSTRAINT` off the hot path if the historical check is needed.

## Parallelism

Phase 1 tables are independent — run one `seedfaker | psql` pipeline per table concurrently.

```bash
for t in users accounts transactions authorizations ledger_entries; do
  ( seedfaker run schema.yaml --table "$t" --format csv \
      | psql "$PGURL" -q -c "\COPY $t ($(hdr $t)) FROM STDIN WITH (FORMAT csv, HEADER true)" \
  ) &
done
wait
```

When one child table dominates wall time, split it with `--shard I/N` — Postgres accepts concurrent `COPY` into one table (RowExclusive per backend, not Exclusive). Full semantics: [distributed-generation](distributed-generation.md), [docs/cli § Sharding and threads](../docs/cli.md#sharding-and-threads).

## Postgres tuning

Bench-only flags are flagged below. Never combine them on durable data.

| Setting                              | Value              | Gain         | Bench only? |
| ------------------------------------ | ------------------ | ------------ | ----------- |
| `UNLOGGED` tables                    | phase 1            | +50–100 %    |             |
| `tmpfs` datadir                      | container `tmpfs:` | +50–300 %    |             |
| `fsync`                              | `off`              | +50–100 %    | yes         |
| `full_page_writes`                   | `off`              | +20–50 %     | yes         |
| `synchronous_commit`                 | `off`              | +10–30 %     |             |
| `wal_level`                          | `minimal`          | less WAL     |             |
| `max_wal_size`                       | 16 GB+             | no mid-load checkpoint |   |
| `shared_buffers`                     | ~25 % RAM          | standard     |             |
| `maintenance_work_mem`               | 2 GB+              | faster phase 2 |           |
| `max_parallel_maintenance_workers`   | 4                  | parallel index build |     |

Throwaway `docker-compose.yml`:

```yaml
services:
  pg:
    image: postgres:17
    environment: { POSTGRES_DB: bench, POSTGRES_USER: bench, POSTGRES_PASSWORD: bench }
    ports: ["55432:5432"]
    tmpfs: ["/var/lib/postgresql/data"]
    command: >
      postgres -c shared_buffers=2GB -c maintenance_work_mem=2GB
      -c max_wal_size=16GB -c synchronous_commit=off -c fsync=off
      -c full_page_writes=off -c wal_level=minimal
      -c max_parallel_maintenance_workers=4
```

## Reference benchmark

[`benchmarks/payments_5gb.sh`](../benchmarks/payments_5gb.sh): 10-table payment system, Dockerised PG 17 with the tuning above, parallel `\COPY`, per-table timings, WAL / checkpoint / cache-hit counters.

```bash
./benchmarks/payments_5gb.sh                                  # ~100 MB
./benchmarks/payments_5gb.sh --scale 50 --jobs 10 --shards 3  # ~5 GB
./benchmarks/payments_5gb.sh --cleanup
```

`--shards N` applies only to the three biggest tables; small tables stay at 1. Optimum on a single host is `cores / 3`.

Scale × wall on an 8-core laptop with tmpfs + `fsync=off`:

| Scale | CSV size | Wall                 |
| ----- | -------- | -------------------- |
| 1     | 100 MB   | ~1 s                 |
| 10    | 1 GB     | ~9 s                 |
| 50    | 5 GB     | ~45 s                |
| 500   | 50 GB    | ~7 min               |
| 5000  | 500 GB   | ~70 min              |

Production (fsync on, durable disk) is roughly 2× slower.

## Other engines

**MySQL:**
```bash
seedfaker run schema.yaml --table users --format csv \
  | mysql -e "LOAD DATA LOCAL INFILE '/dev/stdin' INTO TABLE users
              FIELDS TERMINATED BY ',' ENCLOSED BY '\"' IGNORE 1 LINES"
```
Set `SET unique_checks=0; SET foreign_key_checks=0;` for phase 1.

**ClickHouse:**
```bash
seedfaker run schema.yaml --table users --format csv \
  | clickhouse-client --query "INSERT INTO users FORMAT CSVWithNames"
```

**SQLite** (expects a file, not stdin):
```bash
seedfaker run schema.yaml --table users --format csv > users.csv
sqlite3 db.sqlite <<SQL
PRAGMA journal_mode=OFF; PRAGMA synchronous=OFF;
.mode csv
.import --skip 1 users.csv users
PRAGMA journal_mode=WAL;
SQL
```

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
