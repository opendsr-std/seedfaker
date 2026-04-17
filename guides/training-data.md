# Training and evaluation datasets

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Labeled, reproducible synthetic data for NER/PII training, LLM fine-tuning, evals with ground truth, red-team sets, multi-turn conversations, RAG, multilingual training, and privacy-preserving fine-tuning on real data.

Every value has a known label and exact byte position. Every run is byte-reproducible from `(seed, --until)`. Algorithm drift is caught at load time via `--fingerprint`.

## Contents

- [NER and PII detection](#ner-and-pii-detection)
- [LLM fine-tuning (prompt → completion)](#llm-fine-tuning-prompt--completion)
- [Evaluation sets](#evaluation-sets)
- [Red-team and robustness](#red-team-and-robustness)
- [Multi-turn conversations](#multi-turn-conversations)
- [RAG corpus](#rag-corpus)
- [Multilingual](#multilingual)
- [Privacy-preserving fine-tuning](#privacy-preserving-fine-tuning)
- [Scaling up](#scaling-up)

## NER and PII detection

`--annotated` emits JSONL with `text` and a `spans` array (`s`, `e`, `f`, `v`).

```bash
seedfaker name email ssn phone --annotated --seed train -n 100_000 --until 2025 > ner.jsonl
```

Byte offsets (`v` = `text[s..e]`). With `--corrupt`, spans carry `o` = pre-corruption original.

For natural-language text, wrap in an inline template or use the `pii-leak` preset:

```bash
seedfaker name email ssn --annotated --seed train --until 2025 \
  -t 'Dear {{name}}, we verified your account at {{email}}. Reference: {{ssn}}.'

seedfaker run pii-leak --annotated --seed train -n 10_000 --until 2025 > pii_leak.jsonl
```

Details: [docs/annotated](../docs/annotated.md). Prodigy / doccano / Label Studio consume the JSONL directly.

## LLM fine-tuning (prompt → completion)

Templates + per-column expressions produce paired prompts and completions in one pass.

```yaml
# ft.yaml
columns:
  name: name
  email: email
  last4: integer:1000..9999
  prompt: >
    {{name}} emailed about card ending {{last4}}. Reply at {{email}}.
  completion: >
    {"name":"{{name}}","email":"{{email}}","card_last4":"{{last4}}"}

options:
  seed: ft-2026
  ctx: strict
  count: 50000
  format: jsonl
```

```bash
seedfaker run ft.yaml > extraction.jsonl
```

`ctx: strict` locks name / email to one identity per row. Swap the template for summarisation, classification, or entity-linking.

## Evaluation sets

Pin both seed and algorithm fingerprint — algorithm drift fails loudly at load time, so eval numbers never shift silently across upgrades.

```bash
seedfaker run eval.yaml -n 5_000 --seed eval-v1 --fingerprint sf0-158dc9f79ce46b43 > eval.jsonl
```

Score model output against `spans`:

```python
for line in open("eval.jsonl"):
    rec = json.loads(line)
    truth = {(s["s"], s["e"], s["f"]) for s in rec["spans"]}
    pred  = {(m["start"], m["end"], m["type"]) for m in detect_pii(rec["text"])}
    tp, fn, fp = len(truth & pred), len(truth - pred), len(pred - truth)
```

Ground truth is exact — byte offsets from the generator, not human labels.

## Red-team and robustness

`--corrupt` applies 15 noise types at 4 levels. Deterministic — same seed, same corrupted bytes.

```bash
seedfaker name email ssn --annotated --corrupt extreme --seed redteam -n 10_000 > adversarial.jsonl
```

Each span carries `o` with the pre-corruption original, so detector recall is measured against the clean value.

| Level   | Noise                                                              |
| ------- | ------------------------------------------------------------------ |
| low     | extra spaces, invisible characters                                 |
| mid     | OCR-like substitutions, mojibake                                   |
| high    | truncation, partial masking, homoglyphs                            |
| extreme | stacked corruption, ~95 % of rows corrupted                        |

`high` for training augmentation, `extreme` for red-team eval. Details: [docs/corruption](../docs/corruption.md).

## Multi-turn conversations

Model `(user, ticket, message)` with FK + `ctx: strict`. One user has many messages in one thread.

```yaml
options: { seed: conv-2026 }

users:
  columns: { id: uuid, name: name, email: email }
  options: { count: 5000, ctx: strict }

tickets:
  columns:
    id: uuid
    user_id: users.id:zipf
    user_name: user_id->name
    subject: enum:billing=30,tech=40,account=20,cancel=10
    opened_at: timestamp:asc
  options: { count: 20000 }

messages:
  columns:
    id: uuid
    ticket_id: tickets.id:zipf
    role: enum:user=60,agent=40
    sent_at: timestamp:asc
    body_len: integer:50..400
  options: { count: 100000 }
```

```bash
seedfaker run conv.yaml --all --output-dir ./conv/ --format jsonl
```

Zipf on FKs gives a realistic power-law: a few threads with many messages, most with few. FK semantics: [docs/multi-table](../docs/multi-table.md).

## RAG corpus

Documents + queries with known-relevant targets. seedfaker owns the structure and ground-truth relevance; downstream LLM fills in natural text using the seed fields.

```yaml
documents:
  columns:
    id: uuid
    title: company-name
    body_seed: integer:1..1000000
  options: { count: 100000 }

queries:
  columns:
    id: serial
    target_doc_id: documents.id
    target_title: target_doc_id->title
    query_seed: integer:1..1000000
  options: { count: 10000 }
```

Recall @ K is measured against the FK-linked `target_doc_id`.

## Multilingual

```bash
seedfaker name email phone national-id --annotated --ctx strict \
  -l en=3,ja=2,zh=2,ar=1,de=1 --abc native \
  --seed ml --until 2025 -n 100_000 > multilingual.jsonl
```

`--abc native` keeps names in locale scripts (kanji, hanzi, Cyrillic, Arabic). `national-id` dispatches by locale: SSN (US), CPF (BR), NINO (UK), PESEL (PL), PAN (IN), shenfenzheng (CN). Label stays `national-id`.

Details: [docs/context](../docs/context.md).

## Privacy-preserving fine-tuning

`seedfaker replace` rewrites specific columns in existing CSV / JSONL; every other column passes through. Same value + same seed = same replacement, so joins across independently-masked files still match.

```bash
seedfaker replace email phone ssn --seed anon-v1 < prod_dump.csv > safe_train.csv
```

Fine-tune on `safe_train.csv`. A held-out eval masked with the same seed keeps cross-file references intact. Details: [docs/replace](../docs/replace.md), [guides/anonymize-data](anonymize-data.md).

## Scaling up

Combine `--threads` (in-process) with `--shard I/N` (cross-host):

```bash
seedfaker run corpus.yaml --seed v1 -n 1_000_000_000 --shard 0/3 --threads 8 \
  --format jsonl > part0.jsonl
# shard 1/3 and 2/3 on other hosts with --no-header
cat part{0,1,2}.jsonl > corpus.jsonl
```

Full semantics: [guides/distributed-generation](distributed-generation.md). CLI reference: [docs/cli § Sharding and threads](../docs/cli.md#sharding-and-threads).

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
