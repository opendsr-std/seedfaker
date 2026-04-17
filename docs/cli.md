# CLI reference

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

## Contents

- [Commands](#commands)
- [Options](#options)
- [Field syntax](#field-syntax) — modifiers, ranges, ordering, transforms
- [Column naming](#column-naming) — custom headers
- [Locales](#locales) — 68 locales, weights, native scripts
- [Run](#run) — configs and presets
- [Sharding and threads](#sharding-and-threads) — parallel generation

## Commands

```bash
seedfaker name email phone -n 5                                # quick records
seedfaker name email --format csv --seed demo --until 2025     # reproducible CSV
seedfaker run nginx -n 10000 --seed prod                       # realistic nginx logs
seedfaker replace email phone ssn --seed k < dump.csv          # anonymize a CSV
seedfaker mcp                                                  # MCP server for AI tools
```

## Options

All flags work with both `generate` and `run`. CLI flags override config `options:`.

| Flag                  | Description                                                                        | Default       |
| --------------------- | ---------------------------------------------------------------------------------- | ------------- |
| `-n COUNT`            | Record count (`0` = [unlimited stream](streaming.md))                              | 10            |
| `-s, --seed SEED`     | [Deterministic seed](determinism.md)                                               | Random        |
| `-f, --format FMT`    | `csv`, `tsv`, `jsonl`, `sql=TABLE`                                                 | Tab-separated |
| `-t TEMPLATE`         | Inline [template](templates.md) with `{{field}}`                                   | —             |
| `-l, --locale CODES`  | [Locale codes](#locales) with weights (`en=7,de=2`)                                | All 68        |
| `--ctx strict\|loose` | Lock fields to one identity per record. [Context](context.md)                      | Off           |
| `--corrupt LEVEL`     | `low`, `mid`, `high`, `extreme`. [Corruption](corruption.md)                       | Off           |
| `--abc native\|mixed` | Native scripts for non-Latin locales. [Context](context.md)                        | Latin         |
| `--rate N`            | Throttle to N records/sec. [Streaming](streaming.md)                               | Unlimited     |
| `--tz OFFSET`         | Timezone (`+0300`, `-05:00`, `Z`)                                                  | UTC           |
| `--since TEMPORAL`    | [Temporal range](#temporal-format) start                                           | 1900          |
| `--until TEMPORAL`    | [Temporal range](#temporal-format) end                                             | Now           |
| `-d, --delim DELIM`   | Field delimiter (supports `\t`, `\n`)                                              | Tab           |
| `--no-header`         | Skip header row                                                                    | Off           |
| `-q, --quiet`         | Suppress [determinism warnings](determinism.md)                                    | Off           |
| `--annotated`         | JSONL with [text + spans](annotated.md) for NER/PII training                       | Off           |
| `--shard I/N`         | Generate only shard I of N (see [Sharding and threads](#sharding-and-threads))     | Off           |
| `--threads N`         | In-process parallel generation (see [Sharding and threads](#sharding-and-threads)) | 1             |
| `--validate`          | Validate fields and options, then exit without generating data                     | Off           |
| `--list`              | List [fields](fields.md) or [presets](presets.md)                                  | —             |
| `--fingerprint`       | Algorithm fingerprint (changes when seeded output changes)                         | —             |

### Temporal format

`--since` and `--until` auto-detect format:

| Format        | Example            |
| ------------- | ------------------ |
| Year          | `2025`             |
| Date          | `2025-03-28`       |
| Datetime      | `2025-03-28T14:00` |
| Epoch seconds | `1711630800`       |

`--until 2025` means exclusive upper bound = Jan 1 2026. Same format works in configs, MCP, library bindings, and inline field ranges.

## Field syntax

Every field accepts `:` segments in any order. The parser classifies each automatically.

```bash
# Modifiers — field-specific formatting
seedfaker phone:e164                  # +14155551234
seedfaker amount:usd                  # $1,234.56
seedfaker timestamp:log               # 28/Mar/2025:14:30:00 +0000
seedfaker password:strong             # VncD+vfMp@?&873gd2sr

# Ranges — constrain output
seedfaker integer:1..100              # uniform 1–100
seedfaker amount:100..5000:usd        # $100–$5,000

# Zipf — power-law distribution over a range
seedfaker integer:1..50000:zipf       # most values cluster near 1

# Ordering — monotonic sequences
seedfaker timestamp:asc:log -n 1000   # chronological Apache log
seedfaker amount:desc:usd             # descending amounts

# Sequential IDs — use serial, not integer:asc
seedfaker id=serial name email -n 100  # 0, 1, 2, ...

# Transforms — case conversion on any field
seedfaker name:upper                  # HELEN WHITE

# Enums — custom value lists with weights
seedfaker enum:active=7,inactive=2,banned=1
```

`:omit=N` — skip generation for N% of rows. JSONL outputs `null`, SQL outputs `NULL`, CSV outputs empty cell.

See [fields](fields.md) for the full reference: [serial](fields.md#serial), [ranges](fields.md#range), [ordering](fields.md#ordering), [modifiers](fields.md#modifiers), [transforms](fields.md#transforms), [enums](fields.md#enums), [extended uniqueness](fields.md#extended-uniqueness-xuniq).

## Column naming

`name=field` sets the header in structured output:

```bash
seedfaker id=uuid created=timestamp:asc:log user=name mail=email --format csv --seed demo -n 3 --until 2025
# id,created,user,mail
# c1ecbbc7-fa34-48fc-b8f8-480831f08475,01/Jan/1970:00:00:59 +0000,Paulina Laca,im.ivana@eunet.rs
```

Without `=`, headers are derived: `amount:usd` → `amount_usd`. Expressions use `name=expr` syntax: `total=price*qty`.

See [expressions](expressions.md) for arithmetic between columns and running totals.

## Locales

68 locales across 10 regions. Weights control distribution:

```bash
seedfaker name -l en -n 5                        # US English only
seedfaker name -l en=7,es=2,de=1 -n 100           # 70/20/10 split
seedfaker name email --ctx strict -l ja,ko,zh -n 5  # East Asian, correlated
```

`--abc native` outputs names in native scripts (kanji, Cyrillic, Arabic). `--abc mixed` = ~50% Latin, ~50% native.

See [context](context.md) for full locale list, identity correlation, and gov-ID dispatch.

## Run

```bash
seedfaker run nginx -n 1000 --seed demo              # preset
seedfaker run ./orders.yaml -n 100 --format jsonl     # custom config
seedfaker run --list                                  # list presets
```

All [options](#options) apply. CLI flags override config values. See [configs](configs.md) for YAML syntax, [presets](presets.md) for all 13 embedded presets.

## Sharding and threads

Two orthogonal parallelism mechanisms. They compose. Both preserve determinism: row `k` derives from `(domain_hash, k)` regardless of which process, thread, or shard emits it.

| Flag          | Parallelism unit | Output topology                   |
| ------------- | ---------------- | --------------------------------- |
| `--shard I/N` | OS process       | N independent output streams      |
| `--threads N` | OS thread        | One merged stream in serial order |

### `--shard I/N`

N disjoint, contiguous serial ranges. `I` is 0-indexed; `N ≥ 1`; `N = 1` is a no-op. Requires `-n > 0`. Remainder rows go to the first `COUNT mod N` shards.

```bash
for i in 0 1 2 3; do
  seedfaker name email --seed demo -n 1_000_000 --shard $i/4 --format csv > part$i.csv
done
```

Each shard emits its own header. When concatenating shards into one stream, pass `--no-header` on shards ≥ 1.

### `--threads N`

N OS threads split the serial range, generate into per-thread buffers, main thread writes them in serial order. Single stdout, byte-identical to single-threaded.

```bash
seedfaker name email --seed demo -n 1_000_000 --format csv --threads 4 > all.csv
```

Requires `-n > 0`. Aggregator columns disable threading (order-sensitive state). Memory peak ≈ output size for the whole run — for multi-GB outputs prefer `--shard` or smaller `-n` batches. Composes with `--shard`: `--shard 0/2 --threads 4` = 8-way across 2 processes × 4 threads.

### When to use which

| Situation                                  | Use                        |
| ------------------------------------------ | -------------------------- |
| One stream to file / stdout                | `--threads N`              |
| `seedfaker \| psql "\COPY"` into one table | `--shard I/N` (N backends) |
| Across multiple hosts                      | `--shard I/N`              |
| Aggregator columns, or `-n 0` unlimited    | single-threaded            |
| Max throughput to Postgres on one host     | `--shard` + `--threads`    |

### Determinism

Shards and threads select disjoint subsets of serials; neither reseeds nor reorders. Row `k` is identical to a single-threaded non-sharded run. Concatenated shards (first with header, rest `--no-header`) equal `seedfaker -n COUNT` byte-for-byte.

## Related guides

- [Seed a database](../guides/seed-database.md) — pipe into Postgres/MySQL
- [Seed a large database](../guides/seed-large-database.md) — parallel COPY at GB/TB scale

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
