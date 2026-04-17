# Quick start

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Get from zero to your first reproducible dataset in five minutes.

## Contents

- [Install](#install) — brew, cargo, npm, language libraries
- [Generate records](#generate-records) — first command, field syntax
- [Reproducible output](#reproducible-output) — `--seed` and `--until`
- [Correlate fields](#correlate-fields) — `--ctx strict` for one identity per row
- [Output formats](#output-formats) — CSV, JSONL, SQL, templates
- [Custom column names](#custom-column-names) — `name=field` headers
- [Add noise](#add-noise) — corruption levels for realism testing
- [Config files](#config-files) — declarative YAML datasets
- [Run a preset](#run-a-preset) — 13 ready-made schemas
- [Replace PII](#replace-pii) — anonymize existing CSV/JSONL
- [Pipes](#pipes) — streaming to psql, files, sort
- [Next](#next) — paths by use case + full docs index

## Install

```bash
brew install opendsr-std/tap/seedfaker         # macOS / Linux
cargo install seedfaker                        # from source
npm install -g @opendsr/seedfaker-cli          # npm
```

Also available as [Python, Node.js, Go, PHP, Ruby](library.md) native libraries.

## Generate records

```bash
$ seedfaker name email phone -n 5 --seed qs --until 2025
Sabina Constantin    marijaculo51@culo.xyz           +1 (327) 322-8718
Clara Silva Coelho   kukuh2011@antam.com             (985) 827-5440
Queenie Seah         agustin90@icloud.com            +1 (973) 635-9123
Oliver Lam           bgacinovic6229@orion.rs         +86 139 9757 9782
Ide OConnor          leonid.filippov1016@bk.ru       (809) 243-9964
```

[200+ fields](fields.md) — person, finance, auth, gov-id, internet, healthcare, and more. Fields accept [modifiers, ranges, and transforms](fields.md#field-syntax) via `:` segments:

```bash
$ seedfaker phone:e164 amount:usd credit-card:space -n 3 --seed readme --until 2025
+47412578114     $793.66   3715 236662 87984
+3118148237758   $123.30   4174 0785 8323 6433
+4901707888425   $473.87   3736 553912 88602
```

## Reproducible output

Add `--seed` — same output every time. Pin `--until` — without it, `--until` defaults to the current time and date fields will shift between runs:

```bash
$ seedfaker name email --seed qs --until 2025 -n 3
Sabina Constantin    marijaculo51@culo.xyz
Clara Silva Coelho   kukuh2011@antam.com
Queenie Seah         agustin90@icloud.com
```

Run it again — identical output. Change the seed — different data. See [determinism](determinism.md).

## Correlate fields

[`--ctx strict`](context.md) locks all fields to one identity per record — email follows name, phone matches locale:

```bash
$ seedfaker name email phone --ctx strict --locale en -n 3 --seed qs --until 2025
Jennifer Bennett   jennifertech05@outlook.com   +1 (278) 327-2383
Deborah Fields     deborahy2@mail.ru            834-995-8060
Jason Graham       jasonx5@yahoo.com            +1 (904) 983-5898
```

## Output formats

```bash
# CSV
$ seedfaker name email phone --format csv -n 3 --seed qs --until 2025
name,email,phone
Sabina Constantin,marijaculo51@culo.xyz,"+1 (327) 322-8718"
Clara Silva Coelho,kukuh2011@antam.com,(985) 827-5440
Queenie Seah,agustin90@icloud.com,"+1 (973) 635-9123"

# JSONL
$ seedfaker name email phone --format jsonl -n 2 --seed qs --until 2025
{"name":"Sabina Constantin","email":"marijaculo51@culo.xyz","phone":"+1 (327) 322-8718"}
{"name":"Clara Silva Coelho","email":"kukuh2011@antam.com","phone":"(985) 827-5440"}

# SQL
$ seedfaker name email --format sql=users -n 2 --seed qs --until 2025
INSERT INTO users (name, email) VALUES ('Sabina Constantin', 'marijaculo51@culo.xyz');
INSERT INTO users (name, email) VALUES ('Clara Silva Coelho', 'kukuh2011@antam.com');

# Template — free-form output
$ seedfaker name email -t '{{name}} <{{email}}>' -n 3 --seed qs --until 2025
Sabina Constantin <marijaculo51@culo.xyz>
Clara Silva Coelho <kukuh2011@antam.com>
Queenie Seah <agustin90@icloud.com>
```

All [formats](cli.md#options): `csv`, `tsv`, `jsonl`, `sql=TABLE`, `-t` [template](templates.md). Values are identical regardless of format. Add `--annotated` to any format for [JSONL with text + spans](annotated.md) — useful for NER training and PII scanner benchmarks.

## Custom column names

`name=field` sets the header:

```bash
$ seedfaker id=serial user=name mail=email --format csv --seed qs -n 3 --until 2025
id,user,mail
0,Sabina Constantin,marijaculo51@culo.xyz
1,Clara Silva Coelho,kukuh2011@antam.com
2,Queenie Seah,agustin90@icloud.com
```

See [column naming](cli.md#column-naming).

## Add noise

[`--corrupt`](corruption.md) corrupts values — OCR errors, mojibake, truncation, field swaps:

```bash
$ seedfaker name email --corrupt high -n 5 --seed qs --until 2025
Sabina Constantin        marijaculo51@culo.xyz
Clara Silva Coelho4Q5    kukuh2011@antam.com
Queenie Seah             agustin90@icloud.com
Oli ver Lam              bgacinovic6229@orion.rs
Id3 O                    leonid.filippov1016@bk.runt
```

Four levels: `low`, `mid`, `high`, `extreme`. Deterministic with `--seed`.

## Config files

Define datasets in YAML:

```yaml
# orders.yaml
columns:
  name: name
  price: amount:10..500:plain
  qty: integer:1..20
  total: price * qty

options:
  ctx: strict
  seed: shop
  until: "2025"
  format: csv
```

```bash
seedfaker run ./orders.yaml -n 100
```

See [configs](configs.md) for templates, expressions, aggregators, and presets.

## Run a preset

13 embedded presets for common formats:

```bash
$ seedfaker run nginx -n 1 --seed qs --until 2025
48.116.160.167 - kingmarian02 [28/May/2011:06:25:04 +0000] "GET https://cdn.assets.io/v1/upload HTTP/1.1" 403 ...
```

`nginx`, `auth`, `app-json`, `postgres`, `payment`, `pii-leak`, `user-table`, `email`, `stacktrace`, `chaos`, `llm-prompt`, `syslog`, `medical`. See [presets](presets.md) and [source files](https://github.com/opendsr-std/seedfaker/tree/main/rust/cli/src/presets).

## Replace PII

Anonymize existing CSV/JSONL — replace columns, keep structure:

```bash
$ printf 'name,email,phone\nAlice Chen,alice@corp.com,555-1234\nBob Wilson,bob@work.org,555-5678\n' \
  | seedfaker replace email phone --seed anon --until 2025
name,email,phone
Alice Chen,nolan.moreno.xxy@icloud.com,+1 (744) 555-2784
Bob Wilson,karterreid@ge.com,511-620-2275
```

Same input + same seed = same replacement. See [replace](replace.md).

## Pipes

```bash
seedfaker name email --format sql=users -n 10000 --seed ci --until 2025 | psql mydb
seedfaker run nginx -n 0 --rate 5000 --seed demo > access.log
seedfaker name email -n 100000 --seed ci --until 2025 | sort -t$'\t' -k2 | head -1000
```

`-n 0` = unlimited stream. `--rate N` = N records/sec. See [streaming](streaming.md).

## Next

Pick your path:

**Test fixtures and CI** — reproducible datasets for backends and QA:

[Fields](fields.md) → [Configs](configs.md) → [Determinism](determinism.md) → [Expressions](expressions.md)

**PII scanner benchmarks and NER training** — annotated data for security and ML:

[Annotated](annotated.md) → [Corruption](corruption.md) → [Presets](presets.md) → [Templates](templates.md)

**Library, browser, and API integration** — embed in your application:

[Library](library.md) (Python, Node.js, Go, PHP, Ruby, [Browser/WASM](library.md#browser-wasm)) → [MCP server](mcp.md)

Full docs index: [docs/](README.md). Workflow walkthroughs: [guides/](../guides/).

## Related guides

- [Library usage](../guides/library-usage.md) — library quick start
- [Seed a database](../guides/seed-database.md) — DB seeding quick start

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
