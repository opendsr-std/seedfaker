# Anonymize production data locally

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Replace PII in CSV and JSONL files with deterministic synthetic values. Everything runs locally — no data leaves the machine.

> [Guides](README.md) · [Quick start](../docs/quick-start.md) · [Replace reference](../docs/replace.md) · [Determinism](../docs/determinism.md)

## Contents

- [Quick example](#quick-example)
- [How the mapping works](#how-the-mapping-works)
- [How columns are replaced](#how-columns-are-replaced)
- [CSV files](#csv-files)
- [JSONL files](#jsonl-files)
- [PostgreSQL dump workflow](#postgresql-dump-workflow)
- [Multiple tables with consistent mapping](#multiple-tables-with-consistent-mapping)
- [Large files](#large-files)
- [Temporal columns](#temporal-columns)
- [CI pipeline](#ci-pipeline)

## Quick example

```bash
$ echo 'name,email,phone,ssn
Alice Chen,alice@corp.com,555-1234,123-45-6789
Bob Wilson,bob@work.org,555-5678,987-65-4321' | seedfaker replace email ssn --seed anon
```

```
name,email,phone,ssn
Alice Chen,nolan.moreno.xxy@icloud.com,555-1234,404-16-7659
Bob Wilson,karterreid@ge.com,555-5678,122-40-4526
```

`email` and `ssn` replaced. `name` and `phone` passed through unchanged.

## How the mapping works

Each original value is hashed together with the seed to produce a replacement. The mapping is deterministic:

- `alice@corp.com` with seed `anon` always produces the same fake email
- The same email in a different file with the same seed produces the same replacement
- Change the seed — get a completely different mapping

This means FK relationships survive: if `users.csv` has `alice@corp.com` and `orders.csv` references the same email, both get the same replacement.

## How columns are replaced

You specify which columns to replace by name. If the name matches a seedfaker field (`email`, `phone`, `ssn`, etc.) — the replacement is a realistic typed value. If not — the replacement is a random alphanumeric string of similar length.

Columns not listed in the command pass through unchanged.

## CSV files

```bash
seedfaker replace email phone ssn --seed anon < users.csv > users_safe.csv
```

- First line is treated as header
- Quoted fields and embedded commas are preserved
- Empty values are skipped (stay empty)

## JSONL files

Format is auto-detected from the first line. Force explicitly if needed:

```bash
seedfaker replace email phone --seed anon --input-format jsonl < events.jsonl > events_safe.jsonl
```

```
{"age":32,"email":"rohitquinn43@aol.com","name":"Alice"}
{"age":45,"email":"emiliorobe7944@aol.com","name":"Bob"}
```

Null values and empty strings are skipped. Non-matching keys pass through unchanged.

## PostgreSQL dump workflow

Export a production table, anonymize, load into staging:

```bash
# 1. Export from production
psql -c "COPY users TO STDOUT CSV HEADER" prod_db > users_raw.csv

# 2. Anonymize PII columns
seedfaker replace email phone ssn --seed anon < users_raw.csv > users_safe.csv

# 3. Load into staging
psql -c "\COPY users FROM 'users_safe.csv' CSV HEADER" staging_db
```

Or stream without intermediate files:

```bash
psql -c "COPY users TO STDOUT CSV HEADER" prod_db \
  | seedfaker replace email phone ssn --seed anon \
  | psql -c "\COPY users FROM STDIN CSV HEADER" staging_db
```

## Multiple tables with consistent mapping

When the same email appears in `users.csv` and `orders.csv`, use the same seed to get the same replacement in both:

```bash
seedfaker replace email --seed anon < users.csv > users_safe.csv
seedfaker replace email --seed anon < orders.csv > orders_safe.csv
```

In `users_safe.csv`, `alice@corp.com` maps to `nolan.moreno.xxy@icloud.com`.
In `orders_safe.csv`, `alice@corp.com` maps to the same `nolan.moreno.xxy@icloud.com`.

FK integrity is preserved without any special configuration — it follows from deterministic hashing.

Process an entire directory:

```bash
for f in dump/*.csv; do
  seedfaker replace email phone ssn --seed anon < "$f" > "safe/$(basename "$f")"
done
```

## Large files

`replace` processes stdin line by line. Memory usage is constant — a 30 GB file pipes the same way as 30 KB.

Stream directly without intermediate files:

```bash
psql -c "COPY users TO STDOUT CSV HEADER" prod_db \
  | seedfaker replace email phone ssn --seed anon \
  | psql -c "\COPY users FROM STDIN CSV HEADER" staging_db
```

Compress on the fly:

```bash
seedfaker replace email phone ssn --seed anon < dump.csv | gzip > safe.csv.gz
```

## Temporal columns

Constrain generated dates and timestamps:

```bash
seedfaker replace email birthdate --seed anon --since 1950 --until 2005 < users.csv > safe.csv
```

Accepts year (`2005`), date (`2005-12-31`), or datetime (`2005-12-31T23:59`).

## CI pipeline

Pin `--seed` and `--until` for reproducible anonymized fixtures:

```bash
seedfaker replace email phone ssn --seed ci-anon --until 2025 < dump.csv > fixtures/users.csv
```

Same input + same seed + same `--until` = byte-identical output on every run.

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
