# Configs

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Config files define datasets declaratively in YAML.

## Contents

- [Minimal config](#minimal-config)
- [Options](#options) — every CLI flag as config key, validate, fingerprint guard
- [Column syntax](#column-syntax) — resolution rules, expressions, aggregators
- [Structured output](#structured-output) — CSV, JSONL, SQL
- [Multi-table configs](#multi-table-configs) → separate page

## Minimal config

Single table:

```yaml
columns:
  name: name
  email: email
  phone: phone:e164:omit=15
```

```bash
seedfaker run ./users.yaml -n 5
```

Multi-table:

```yaml
users:
  columns:
    id: uuid
    name: name
    email: email
  options:
    count: 1000

orders:
  columns:
    id: serial
    customer_id: users.id:zipf
    customer_name: customer_id->name
    total: amount:usd:1..5000
  options:
    count: 10000
```

```bash
seedfaker run ./shop.yaml --table orders -n 100
seedfaker run ./shop.yaml --all --output-dir ./data/
```

## Options

Almost every [CLI flag](cli.md#options) has a config equivalent; CLI flags override config values. Runtime-parallelism flags (`--shard`, `--threads`) are CLI-only — they control the invocation, not the dataset.

```yaml
options:
  seed: ci-fixtures
  count: 1000
  locale: [en=7, de=2]
  ctx: strict
  format: csv
  fingerprint: sf0-6d405deafbc76730
```

| Option        | Type                               | Default   | Description                                                  |
| ------------- | ---------------------------------- | --------- | ------------------------------------------------------------ |
| `seed`        | string                             | random    | [Deterministic seed](determinism.md)                         |
| `count`       | integer                            | 10        | Record count (`0` = unlimited)                               |
| `locale`      | list                               | all       | Locale codes, with optional weights (`[en=7, de=2]`)         |
| `ctx`         | `strict`/`loose`                   | off       | Record correlation. [Context](context.md)                    |
| `corrupt`     | level name                         | off       | Data corruption. [Corruption](corruption.md)                 |
| `abc`         | `native`/`mixed`                   | latin     | Script selection                                             |
| `format`      | `csv`/`tsv`/`jsonl`/`sql=TABLE`    | tsv       | Output format (structured only)                              |
| `rate`        | integer                            | unlimited | Records per second                                           |
| `tz`          | offset                             | UTC       | Timezone offset (`+0300`, `-08:00`, `Z`)                     |
| `since`       | [temporal](cli.md#temporal-format) | 1900      | Range start                                                  |
| `until`       | [temporal](cli.md#temporal-format) | now       | Range end                                                    |
| `delim`       | string                             | tab       | Field delimiter. Any string; `\t`, `\n` for escapes          |
| `no_header`   | bool                               | false     | Skip CSV/TSV header                                          |
| `annotated`   | bool                               | false     | JSONL with [text + spans](annotated.md) for NER/PII training |
| `validate`    | bool                               | false     | Validate config without generating data                      |
| `fingerprint` | string                             | —         | Expected algorithm fingerprint. Rejects run on mismatch      |

### Fingerprint guard

`fingerprint` pins a config to a specific version of the generation algorithm. If present, seedfaker compares it against the current fingerprint at load time and rejects the run on mismatch:

```
error: config fingerprint sf0-aaa does not match current sf0-bbb;
output would differ — update or remove fingerprint from config
```

Without `fingerprint`, configs run against any version. Check the current value with `seedfaker --fingerprint`.

## Column syntax

Column values are plain strings. The parser resolves each value through a fixed priority chain:

1. **Expression** — `+`, `-`, or `*` with valid operands → arithmetic expression
2. **Aggregator** — `source:sum`, `source:count` → running aggregate
3. **Column reference** — matches another declared column → copies its value. With modifier (`col:usd`), reformats the raw value
4. **Field registry** — known field name with modifiers → generator
5. **Error** — none of the above match

Columns can be declared in any order — dependency graph resolved at compile time. Output columns appear in YAML declaration order.

```yaml
columns:
  id: serial # sequential counter (0, 1, 2, ...)
  name: name # field generator
  phone: phone:e164 # field with modifier
  role: enum:admin,user # enum
  base: amount:30000..80000 # field with range
  bonus: amount:1000..5000 # field with range
  total: base + bonus # expression
  total_usd: total:usd # column reference with modifier (reformats total as $1,234.56)
  salary: base:sum # aggregator (running sum)
```

See [expressions](expressions.md) for arithmetic, aggregators, and chaining. See [fields](fields.md) for ranges, ordering, modifiers, and transforms.

## Structured output

Format controlled by `--format` or `options.format`:

```yaml
columns:
  name: name
  email: email
  phone: phone:e164
  role: enum:admin,user,viewer

options:
  ctx: strict
  seed: ci-fixtures
  format: csv
```

```bash
seedfaker run ./users.yaml -n 1000
seedfaker run ./users.yaml --format jsonl -n 1000
seedfaker run ./users.yaml --format sql=users -n 1000
```

For free-form text output, add `template:` — see [templates](templates.md).

## Multi-table configs

For FK relationships, Zipf distribution, and cross-table computed fields — see [multi-table configs](multi-table.md).

## Related guides

- [Seed a database](../guides/seed-database.md) — multi-table configs → Postgres
- [Reproducible datasets](../guides/reproducible-datasets.md) — pinned fixtures for CI

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
