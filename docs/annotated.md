# Annotated output

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

`--annotated` outputs JSONL where each record contains the formatted text and byte-offset spans for every generated value. Works with any output mode — structured (CSV, TSV, JSONL, SQL) and templates.

## Contents

- [Usage](#usage)
- [Output format](#output-format) — structured and template examples
- [Span keys](#span-keys)
- [What produces spans](#what-produces-spans)
- [Use cases](#use-cases) — scanner benchmarks, NER training, data quality

## Usage

```bash
# Structured output
seedfaker name email phone:e164 -n 1000 --annotated --seed demo --format csv
seedfaker name email -n 5000 --annotated --format jsonl

# Template output
seedfaker run pii-leak -n 1000 --annotated --seed demo
seedfaker run nginx --annotated -n 5000

# With corruption
seedfaker run pii-leak -n 100000 --annotated --corrupt mid --seed train

# Pipe to NER trainer
seedfaker run pii-leak -n 100000 --annotated --corrupt mid | python train_ner.py
```

Also available as config option: `annotated: true`.

## Output format

Each line is a JSON object with `text` and `spans`:

### Structured (CSV)

```bash
seedfaker name email phone:e164 -n 1 --annotated --seed demo --format csv
```

```json
{
  "text": "Paulina Laca,im.ivana@eunet.rs,+278458384682",
  "spans": [
    { "s": 0, "e": 12, "f": "name", "v": "Paulina Laca" },
    { "s": 13, "e": 30, "f": "email", "v": "im.ivana@eunet.rs" },
    { "s": 31, "e": 44, "f": "phone", "v": "+278458384682" }
  ]
}
```

### Template (pii-leak)

```bash
seedfaker run pii-leak -n 1 --annotated --seed demo
```

```json
{
  "text": "--- CRM Note | 2025-05-25T21:12:48Z | Agent: spyros_papageorghiou_vip ---\nContact: Spyros Papageorghiou <spyros@icloud.com>",
  "spans": [
    { "s": 15, "e": 35, "f": "timestamp", "v": "2025-05-25T21:12:48Z" },
    { "s": 45, "e": 69, "f": "username", "v": "spyros_papageorghiou_vip" },
    { "s": 83, "e": 103, "f": "name", "v": "Spyros Papageorghiou" },
    { "s": 105, "e": 122, "f": "email", "v": "spyros@icloud.com" }
  ]
}
```

### With corruption

Corrupted spans include the pre-corruption value in `o`:

```json
{ "s": 9, "e": 11, "f": "name", "v": "Sp", "o": "Spyros Papageorghiou" }
```

## Span keys

| Key | Description                                                                 |
| --- | --------------------------------------------------------------------------- |
| `s` | Start byte offset in `text` (inclusive)                                     |
| `e` | End byte offset in `text` (exclusive). `text[s..e] == v`                    |
| `f` | Field type from registry (`name`, `email`, `ssn`, `integer`, `serial`, ...) |
| `v` | Generated value — always matches `text[s..e]`                               |
| `o` | Pre-corruption value (only present if corrupted)                            |

## What produces spans

**Structured output:** every column value produces a span.

**Template output:** every `{{...}}` substitution — declared columns, inline fields, enums, serial. Template literal text does not produce spans.

With `--corrupt`, all generated values (including inline fields in templates) can be corrupted.

## Use cases

**PII scanner benchmarks.** Generate text with known PII locations, run your scanner, compare output spans against ground truth.

```bash
seedfaker run pii-leak -n 10000 --annotated --seed bench | python eval_scanner.py
```

**NER model training.** Output format is compatible with Prodigy/doccano annotation format — `text` + `spans` with character offsets.

```bash
seedfaker run pii-leak -n 100000 --annotated --corrupt mid --seed train > train.jsonl
```

**Data quality analysis.** Inspect what was generated and where corruption changed values.

```bash
seedfaker name email -n 100 --annotated --corrupt extreme --format csv --seed qa
```

## Examples

- [`examples/17-annotated.sh`](../examples/17-annotated.sh) — shell script with structured + template + corruption
- [`examples/output/annotated-pii-leak.jsonl`](../examples/output/annotated-pii-leak.jsonl) — pii-leak preset, 5 records
- [`examples/output/annotated-csv.jsonl`](../examples/output/annotated-csv.jsonl) — CSV annotated, 5 records

## Related guides

- [Training and evaluation datasets](../guides/training-data.md) — span-labeled NER/PII, LLM evaluation, red-team sets

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
