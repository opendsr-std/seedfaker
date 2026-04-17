# Changelog

Format: [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.3.0-alpha.2]

### Removed

- Remove some examples, cleanup gh history by external reports

## [0.3.0-alpha.1]

### Added

- `--shard I/N` and `--threads N` for parallel / distributed generation. Both compose; both preserve determinism (concat of shards = threaded output = single-process output, bit-for-bit). See [docs/cli § Sharding and threads](docs/cli.md#sharding-and-threads).
- `Rng::zipf_from_cdf(&[f64])` — pre-built CDF reuse.
- Bulk-load benchmark [`benchmarks/payments_5gb.sh`](benchmarks/payments_5gb.sh) — 10-table payment dataset, Dockerised Postgres 17 on tmpfs, parallel `\COPY`, per-table timings and WAL / checkpoint diagnostics. Scales via `--scale N` (1 ≈ 100 MB).
- Guides: [Seed a large database](guides/seed-large-database.md), [Distributed generation](guides/distributed-generation.md), [Training and evaluation datasets](guides/training-data.md).
- [docs/README.md](docs/README.md) — docs index.
- Examples [`19-parallel.sh`](examples/19-parallel.sh), [`20-corruption.sh`](examples/20-corruption.sh).

### Changed

- Zipf CDF cached per FK column at compile time instead of rebuilt per child row. `transactions` at scale=10: 116 s → 2.4 s. Aggregate `payments_5gb.sh`: 0.5 MB/s → 66.9 MB/s at `--jobs 2`.
- Enum modifier parsed once per column instead of per row. `authorizations` at scale=10: 3.06 s → 1.94 s.
- Root [README.md](README.md) restructured: Highlights block, Distributed / Bulk-load sections, Packages table. Guides moved to the bottom.
- Guides consolidated by persona (10 guides, one use case each). Docs + guides share one canonical breadcrumb.
- Examples 01–17 simplified to one focused use case per file.

### Removed

- `guides/ner-training-data.md` + `guides/ml-and-llm-datasets.md` → merged into [`guides/training-data.md`](guides/training-data.md). No compat redirects.
- Parent-id memoisation experiment — measured net-negative, reverted.
- mimalloc allocator experiment — regressed cold-start on macOS ARM, reverted.

### Fixed

- `ctx:strict` + FK dereference: parent `Identity` is now reconstructed in `generate_parent_field`, so `user_id->email` and `user_id->first_name` belong to the same identity.
- `asc`/`desc` + FK dereference: parent column ordering is now propagated via new `parent_ordering` / `deref_ordering` fields on `ColumnGen::Fk` / `FkDeref`.

### Performance

8-core Apple M-series, Postgres 17 on tmpfs with `fsync=off`:

| Configuration                                | Wall   | Aggregate |
| -------------------------------------------- | ------ | --------- |
| Before session, `--jobs 2`                   | 125 s  | 0.5 MB/s  |
| After, `--jobs 6 --shards 3`, scale=10       | 10 s   | 66.9 MB/s |
| Pure generation, single table, `--threads 8` | 0.47 s | 403 MB/s  |

Projected 1 TB on the same host: ~4.3 h. Next bottleneck is psql CSV lexer; binary `COPY FORMAT binary` is on the roadmap.

## [0.2.0-alpha.6]

### Changed

- Docs update

## [0.2.0-alpha.5]

### Changed

- Docs update

## [0.2.0-alpha.4]

### Changed

- Docs update

## [0.2.0-alpha.3]

### Changed

- README restructured: 3 use-case sections (generate, anonymize, train PII models), comparison table
- 7 guides added: seed database, mock API, anonymize data, NER training, reproducible datasets, streaming, MCP
- Docs reorganized: multi-table extracted to separate page, unified footer navigation, guides cross-linked
- All package READMEs updated with consistent descriptions and guide links
- Field reference generator fixed (removed duplicate modifier entries)

## [0.2.0-alpha.1]

### Added

- **[Multi-table configs](docs/configs.md#multi-table-configs)** — define multiple tables in one YAML file with FK relationships
  - `table.column` syntax for FK anchors (e.g. `users.id`)
  - `anchor->column` syntax for FK dereference (e.g. `customer_id->name`)
  - [`:zipf`](docs/fields.md#zipf-distribution) / `:zipf=N` on FK anchors for power-law parent distribution
  - Automatic topological sort — tables generated in dependency order
  - [Self-referencing FK](docs/configs.md#self-referencing-fk) (table references itself)
  - Per-table [`template:`](docs/templates.md) support
  - Root → per-table → CLI [options merge](docs/configs.md#options-merge)
- **CLI flags** for multi-table: `--table TABLE`, `--all`, `--output-dir DIR`
- **[Expressions](docs/expressions.md) with FK deref** — arithmetic on dereferenced fields (e.g. `unit_price * qty`)
- FK deref columns now return raw numeric values, enabling [aggregators](docs/expressions.md) and expressions downstream

### Fixed

- FK domain hash did not account for column alias detection — dereferenced values could differ from parent table output
- FK sampling used inclusive range `[0, count]` instead of `[0, count-1]`, producing out-of-bounds row indices
- Grouped `count=column` aggregator used source column as group key instead of the specified group column
- FK/FkDeref columns reported `FieldType::Text` — expressions referencing them were rejected

### Changed

- `engine::run` accepts generic `W: Write` parameter instead of writing to stdout directly
- `config::load_config` returns `ConfigKind` enum (`Single` or `Multi`)
