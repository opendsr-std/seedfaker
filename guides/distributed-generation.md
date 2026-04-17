# Distributed generation

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Generate one dataset across multiple machines without coordination. Each host emits a disjoint slice; concatenated slices are byte-identical to a single-host run.

## Contents

- [Why it works](#why-it-works)
- [Three-host example](#three-host-example)
- [Verify equivalence](#verify-equivalence)
- [Uneven hosts](#uneven-hosts)
- [Direct to Postgres](#direct-to-postgres)
- [Direct to object storage](#direct-to-object-storage)
- [Composing with threads](#composing-with-threads)
- [Failure and retry](#failure-and-retry)

## Why it works

Every value derives from `(seed, record_number, field_name)`. `--shard I/N` emits only rows with `record_number` in its slice. Two invariants:

1. **Disjoint** — different shards never emit the same row.
2. **Equivalent** — `cat` of all shards (header kept once, rest `--no-header`) equals a non-sharded run with the same seed and `-n`.

No coordinator, no network, no shared state between processes or hosts.

## Three-host example

```bash
# host-a
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 \
  --shard 0/3 --format csv > events.part0.csv

# host-b
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 \
  --shard 1/3 --format csv --no-header > events.part1.csv

# host-c
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 \
  --shard 2/3 --format csv --no-header > events.part2.csv

cat events.part{0,1,2}.csv > events.csv
```

`events.csv` has bytes identical to `seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 --format csv`.

## Verify equivalence

Before trusting a distributed setup, verify against a single-host reference on small `-n`:

```bash
seedfaker run shop.yaml --table events --seed prod -n 10_000 --format csv \
  | sha256sum > full.sha

{
  seedfaker run shop.yaml --table events --seed prod -n 10_000 --shard 0/3 --format csv
  seedfaker run shop.yaml --table events --seed prod -n 10_000 --shard 1/3 --format csv --no-header
  seedfaker run shop.yaml --table events --seed prod -n 10_000 --shard 2/3 --format csv --no-header
} | sha256sum > shards.sha

diff full.sha shards.sha && echo OK
```

Run this in CI whenever the schema changes or `seedfaker` upgrades. The algorithm fingerprint catches binary drift; this catches schema drift.

## Uneven hosts

`--shard` distributes evenly. For heterogeneous hardware, either give the faster host more shards (4-core host: `--shard 0..3/8`; 8-core host: `--shard 4..7/8`), or stack `--threads` inside each shard — see [Composing with threads](#composing-with-threads).

Remainder rows land on the first `count mod N` shards; schedule those on the fastest hosts if that matters.

## Direct to Postgres

Each shard pipes into its own `\COPY`. Postgres takes RowExclusive per backend, so concurrent `COPY` into one table is supported.

```bash
# host-a
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 \
  --shard 0/3 --format csv \
  | psql "$PGURL" -q -c "\COPY events (id,ts,user_id) FROM STDIN WITH (FORMAT csv, HEADER true)"
```

Same pattern for ClickHouse (`clickhouse-client ... FORMAT CSVWithNames`) and MySQL (`LOAD DATA LOCAL INFILE '/dev/stdin'`).

## Direct to object storage

For analytical pipelines, shards write to S3/GCS/Azure and a query engine reads them as one dataset.

```bash
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 \
  --shard 0/3 --format csv | aws s3 cp - s3://bucket/dataset/events.part0.csv
```

Consumers use wildcard paths (`SELECT FROM s3://bucket/dataset/events.*`). Shard boundaries are invisible to queries.

## Composing with threads

`--shard` partitions across processes and hosts. `--threads` partitions inside one process. Orthogonal and composable.

```bash
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 \
  --shard 0/3 --threads 8 --format csv > events.part0.csv
```

Three hosts × 8 threads = 24 concurrent generators producing one deterministic dataset.

## Failure and retry

A shard is pure: same input → same output. Kill and re-run with the same `--shard I/N` — surviving shards are unaffected, no state to reconcile.

- File output: on mid-write failure, delete the partial file and re-run that shard.
- Pipe to `\COPY`: Postgres rolls back the backend's ingest on pipe error. Re-run the shard.

Never retry with a different seed — the guarantee is seed-based.

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
