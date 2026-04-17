# seedfaker

Deterministic synthetic data generator. Same seed, same output — across CLI, Python, Node.js, Go, PHP, Ruby, WASM.

200+ fields, 68 locales, multi-table FK, expressions, templates, streaming, `replace` for anonymising existing data.

## Highlights

- **Deterministic across 7 runtimes** — CLI, Python, Node, Go, PHP, Ruby, WASM. Same seed → byte-identical bytes. `--fingerprint` catches algorithm drift. [→](#determinism)
- **Multi-table FK** — anchors (`users.id:zipf`), dereference (`customer_id->email`), self-reference, `ctx:strict` identity correlation. [→](docs/multi-table.md)
- **Distributed** — `--shard I/N` on three hosts, concatenate, bit-identical to single-host. No coordinator. [→](#distributed-generation)
- **Database ingest** — `seedfaker | psql "\COPY"`, no files, constant memory. [→](guides/seed-database.md)
- **TB-scale** — 1 GB into Postgres in 9 s on 8-core ([benchmark](benchmarks/payments_5gb.sh)); 1 TB ≈ 4.3 h. [→](guides/seed-large-database.md)
- **Throughput** — ~90 MB/s per core (TPC-H dbgen parity), 403 MB/s on 8 threads. Reproducible in [`benchmarks/`](benchmarks/).
- **In-place anonymisation** — `seedfaker replace email ssn < dump.csv`. Same value + seed = same replacement; cross-file joins survive. [→](docs/replace.md)
- **ML/LLM datasets** — `--annotated` (byte-offset spans), `--corrupt` (15 noise types), templates (prompt/completion), multi-table FK (conversations, RAG). [→](guides/training-data.md)
- **Locale-aware PII** — Luhn credit cards, IBAN check digits, 48 gov-ID formats, 68 locales, native scripts. [→](docs/fields.md)

## Contents

- [Install](#install)
- [Library](#library)
- [CLI](#cli)
- [Multi-table and FK](#multi-table-and-fk)
- [Distributed generation](#distributed-generation)
- [Bulk load into a database](#bulk-load-into-a-database)
- [Anonymise existing data](#anonymise-existing-data)
- [Annotated output for ML](#annotated-output-for-ml)
- [Determinism](#determinism)
- [Packages and bindings](#packages-and-bindings)
- [Documentation](#documentation)
- [Quick start](#quick-start)
- [Guides](#guides)
- [Benchmarks](#benchmarks)
- [License](#license)

## Install

One of:

```bash
pip install seedfaker                          # Python
npm install @opendsr/seedfaker                 # Node.js
go get github.com/opendsr-std/seedfaker-go     # Go
composer require opendsr/seedfaker             # PHP
gem install seedfaker                          # Ruby
npm install @opendsr/seedfaker-wasm            # Browser (WASM)
brew install opendsr-std/tap/seedfaker         # CLI (macOS / Linux)
cargo install seedfaker                        # CLI (from source)
npm install -g @opendsr/seedfaker-cli          # CLI (npm)
```

All packages wrap the same Rust core and produce byte-identical output for a given seed. See [Packages and bindings](#packages-and-bindings) for per-package documentation.

## Library

One value:

```python
from seedfaker import SeedFaker
sf = SeedFaker(seed="test")
sf.field("email")                  # "janet.marsh@inbox.com"
sf.field("phone", e164=True)       # "+14155551234"
sf.field("credit-card", space=True) # "4174 0785 8323 6433"
```

One record, with `ctx="strict"` locking every field to one identity:

```python
sf.record(["name", "email", "phone"], ctx="strict")
# {"name": "Janet Marsh", "email": "janet.marsh@inbox.com", "phone": "+1 (957) 226-4272"}
```

Batch:

```python
sf.records(["name", "email", "phone"], n=1000, ctx="strict")
```

Locales, weighted mix, native script:

```python
SeedFaker(seed="test", locale="de").field("name")        # "Baldur Adler"
SeedFaker(seed="test", locale="ja").field("name")        # "石本 和彦"
SeedFaker(seed="test", locale="en=7,de=2,fr=1")          # weighted
```

Node.js API is identical:

```js
const sf = new SeedFaker({ seed: "test", locale: "en" });
sf.records(["name", "email"], { n: 1000, ctx: "strict" });
```

Full API: [docs/library](docs/library.md). Locale list: [docs/context](docs/context.md).

## CLI

```bash
seedfaker name email phone --seed test --until 2025 -n 1000
seedfaker name email phone --format csv --seed test --until 2025 -n 1000
seedfaker name email phone --format jsonl --seed test --until 2025 -n 1000
seedfaker name email --ctx strict -l ja,zh --abc native -n 10
```

Pipe directly into a database:

```bash
seedfaker name email phone --format sql=users -n 1000000 --seed staging --until 2025 | psql mydb
```

Arithmetic between columns:

```bash
seedfaker price=amount:1..500:plain qty=integer:1..20 "total=price*qty" \
  --format csv --seed ci -n 3 --until 2025
# price,qty,total
# 424.49,14,5942.86
# 459.67,3,1379.01
# 309.44,12,3713.28
```

Presets for common log/data shapes:

```bash
seedfaker run nginx   --rate 5000 --seed demo -n 0 > access.log
seedfaker run payment --format jsonl --seed bench -n 1000 --until 2025
```

All flags: [docs/cli](docs/cli.md). Field syntax: [docs/fields](docs/fields.md). Configs: [docs/configs](docs/configs.md). Presets: [docs/presets](docs/presets.md).

## Multi-table and FK

```yaml
# shop.yaml
users:
  columns:
    id: serial
    name: first-name
    email: email
  options: { count: 1000, ctx: strict }

orders:
  columns:
    id: serial
    customer_id: users.id:zipf
    customer_name: customer_id->name
    customer_email: customer_id->email
    total: amount:usd:1..5000
  options: { count: 50000 }
```

```bash
seedfaker run shop.yaml --all --output-dir ./data --format csv
```

- `users.id:zipf` — FK anchor with power-law distribution. `:zipf=N` for tunable exponent; omit for uniform.
- `customer_id->email` — FK dereference; resolves to the email of the same parent row selected by `customer_id`.
- Self-referencing FK supported (`employees.manager_id: employees.id`).

Details: [docs/multi-table](docs/multi-table.md), [docs/expressions](docs/expressions.md).

For bulk-loading a real database at GB/TB scale see [guides/seed-large-database](guides/seed-large-database.md).

## Distributed generation

Determinism enables horizontal scale without coordination. `--shard I/N` emits a disjoint, contiguous slice of the full `serial` range; the same seed on different hosts produces non-overlapping output. Concatenating all N shards (first shard's header retained, rest with `--no-header`) yields bytes bit-identical to an `N=1` run.

Three hosts, one dataset:

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
```

Collect and concatenate:

```bash
cat events.part0.csv events.part1.csv events.part2.csv > events.csv
# Same bytes, same SHA-256 as:
seedfaker run shop.yaml --table events --seed prod -n 1_000_000_000 --format csv
```

No shared state between hosts. No coordinator. No post-processing merge step. Each host is CPU-bound on its own slice and finishes independently.

Per-host generation can also use `--threads N` on top of `--shard`, stacking process and in-process parallelism:

```bash
seedfaker ... --shard 0/3 --threads 8 --format csv > events.part0.csv
```

Details on which mechanism to pick and how they compose: [docs/cli § Sharding and threads](docs/cli.md#sharding-and-threads), [guides/seed-large-database](guides/seed-large-database.md).

## Bulk load into a database

Pipe generated CSV straight into `COPY FROM STDIN` — no intermediate files, constant memory:

```bash
seedfaker run shop.yaml --table users --format csv \
  | psql "$PGURL" -q -c "\COPY users (id,name,email) FROM STDIN WITH (FORMAT csv, HEADER true)"
```

For GB/TB-scale loads: strip all constraints during phase 1, add them back afterwards.

```sql
CREATE UNLOGGED TABLE users (id UUID NOT NULL, name TEXT, email TEXT);
-- load rows with COPY FROM STDIN (no PK, no FK, no indexes)
ALTER TABLE users SET LOGGED;
ALTER TABLE users ADD PRIMARY KEY (id);
```

Reason: Postgres constraint and index maintenance is per-row during INSERT/COPY; deferring to a single post-load scan is dramatically faster. seedfaker guarantees id uniqueness by construction, so phase-1 validation is wasted work.

`--shard I/N` splits one table's generation into N disjoint serial ranges. Run multiple `seedfaker | psql` pipelines in parallel into the same table — Postgres takes a RowExclusive lock per backend, not Exclusive, so concurrent `COPY` into one table is supported.

```bash
# 4 shards into the same table, concurrent
for i in 0 1 2 3; do
  seedfaker run shop.yaml --table events --format csv --shard $i/4 \
    | psql "$PGURL" -q -c "\COPY events (id,ts,user_id) FROM STDIN WITH (FORMAT csv, HEADER true)" &
done
wait
```

The reference benchmark [`benchmarks/payments_5gb.sh`](benchmarks/payments_5gb.sh) implements this pattern end-to-end: 10-table payment dataset, Dockerised Postgres 17 with tuned settings, per-table shard pool, Postgres-side WAL / checkpoint / cache-hit counters.

```bash
./benchmarks/payments_5gb.sh                       # ~100 MB, default
./benchmarks/payments_5gb.sh --scale 50 --shards 3 # ~5 GB with 3-way sharding of the big tables
./benchmarks/payments_5gb.sh --cleanup
```

Full workflow, tuning rationale, per-knob cost table, cross-engine notes (MySQL, ClickHouse, SQLite): [guides/seed-large-database](guides/seed-large-database.md).

## Anonymise existing data

Replace specific columns in existing CSV or JSONL, keeping other columns untouched and preserving referential integrity across files:

```bash
$ echo 'name,email,ssn
Alice,alice@corp.com,123-45-6789' | seedfaker replace email ssn --seed anon
name,email,ssn
Alice,nolan.moreno.xxy@icloud.com,404-16-7659
```

Same value + same seed yields the same replacement every run, so joining `users.email` and `events.email` (after masking each independently) still matches. Details: [docs/replace](docs/replace.md).

## Annotated output for ML

`--annotated` emits JSONL with byte-offset spans, suitable for NER / PII training sets:

```bash
$ seedfaker name email ssn --annotated --seed demo -n 1 --until 2025
{"text":"Paulina Laca\tim.ivana@eunet.rs\t9580255797203","spans":[{"s":0,"e":12,"f":"name","v":"Paulina Laca"},{"s":13,"e":30,"f":"email","v":"im.ivana@eunet.rs"},{"s":31,"e":44,"f":"ssn","v":"9580255797203"}]}
```

Combine with `--corrupt low|mid|high|extreme` for noisy training data. Details: [docs/annotated](docs/annotated.md), [docs/corruption](docs/corruption.md).

## Determinism

Each value is derived from `(seed, record_number, field_name)`. Consequences:

- Adding a field does not change values of existing fields.
- Reordering fields in the config does not change values.
- The same seed produces byte-identical output across languages and versions within the same algorithm fingerprint.

Pin the fingerprint in CI to detect algorithm changes:

```bash
seedfaker --fingerprint
# sf0-158dc9f79ce46b43
```

Details: [docs/determinism](docs/determinism.md), [docs/context](docs/context.md) (identity correlation).

## Packages and bindings

| Language / runtime | Package                                                               | Local docs                            |
| ------------------ | --------------------------------------------------------------------- | ------------------------------------- |
| Python             | `pip install seedfaker`                                               | [packages/pip](packages/pip/)         |
| Node.js            | `npm install @opendsr/seedfaker`                                      | [packages/npm](packages/npm/)         |
| Go                 | `go get github.com/opendsr-std/seedfaker-go`                          | [packages/go](packages/go/)           |
| PHP                | `composer require opendsr/seedfaker`                                  | [packages/php](packages/php/)         |
| Ruby               | `gem install seedfaker`                                               | [packages/ruby](packages/ruby/)       |
| Browser (WASM)     | `npm install @opendsr/seedfaker-wasm`                                 | [packages/wasm](packages/wasm/)       |
| CLI (npm)          | `npm install -g @opendsr/seedfaker-cli`                               | [packages/npm-cli](packages/npm-cli/) |
| CLI (native)       | `brew install opendsr-std/tap/seedfaker` or `cargo install seedfaker` | [docs/cli](docs/cli.md)               |

All packages wrap the same Rust core. API surface is intentionally identical across languages except for idiomatic naming.

## Documentation

Reference: [docs/](docs/).

|                  |                                                                                                           |
| ---------------- | --------------------------------------------------------------------------------------------------------- |
| **Start here**   | [Quick start](docs/quick-start.md)                                                                        |
| **CLI**          | [Commands and flags](docs/cli.md) · [Determinism](docs/determinism.md)                                    |
| **Fields**       | [Syntax and modifiers](docs/fields.md) · [Field reference (200+)](docs/field-reference.md)                |
| **Configs**      | [YAML configs](docs/configs.md) · [Multi-table](docs/multi-table.md) · [Expressions](docs/expressions.md) |
| **Output**       | [Templates](docs/templates.md) · [Annotated](docs/annotated.md) · [Streaming](docs/streaming.md)          |
| **Data quality** | [Context](docs/context.md) · [Corruption](docs/corruption.md) · [Replace](docs/replace.md)                |
| **Presets**      | [Built-in presets](docs/presets.md) (nginx, payment, auth, postgres, syslog, medical, …)                  |
| **Integrations** | [Library API](docs/library.md) · [MCP](docs/mcp.md)                                                       |

Workflows: [guides/](guides/). Runnable examples: [examples/](examples/).

## Quick start

```bash
pip install seedfaker
python -c 'from seedfaker import SeedFaker; print(SeedFaker(seed="demo").record(["name","email"]))'
```

Or with the CLI:

```bash
brew install opendsr-std/tap/seedfaker
seedfaker name email phone --seed demo --until 2025 -n 5
```

Then: [docs/quick-start](docs/quick-start.md) for the 10-minute walkthrough, [docs/cli](docs/cli.md) for flags, [docs/fields](docs/fields.md) for field syntax.

## Guides

End-to-end workflows in [guides/](guides/):

|                                                             |                                                                          |
| ----------------------------------------------------------- | ------------------------------------------------------------------------ |
| [Seed a database](guides/seed-database.md)                  | Postgres/MySQL staging DB with multi-table FK                            |
| [Seed a large database](guides/seed-large-database.md)      | GB/TB bulk load — parallel COPY, UNLOGGED, tuning                        |
| [Distributed generation](guides/distributed-generation.md)  | Multi-host sharded generation without coordination                       |
| [Anonymise production data](guides/anonymize-data.md)       | `replace` on CSV/JSONL, FK integrity across files                        |
| [Training and evaluation datasets](guides/training-data.md) | NER/PII, LLM fine-tuning, eval with ground truth, red-team, multilingual |
| [Reproducible datasets](guides/reproducible-datasets.md)    | Deterministic fixtures, CI, fingerprint guard                            |
| [Library usage](guides/library-usage.md)                    | Python / Node.js SDK patterns                                            |
| [Mock API server](guides/mock-api-server.md)                | Express / FastAPI mock endpoint                                          |
| [API load testing](guides/api-load-testing.md)              | Rate-limited streaming, corruption                                       |
| [MCP for AI agents](guides/mcp-ai-agents.md)                | Claude / Cursor / VS Code integration                                    |

## Benchmarks

Reproducible throughput measurements, install scripts, per-field breakdowns, and an end-to-end Postgres load benchmark (`payments_5gb.sh`): [benchmarks/](benchmarks/).

## License

MIT

---

> [README](README.md) · [Docs](docs/) · [Guides](guides/) · [Packages](packages/)
