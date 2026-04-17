# Streaming

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Unlimited data streams with optional rate limiting. Pipe to databases, log processors, or files.

## Contents

- [Unlimited stream](#unlimited-stream)
- [Rate limiting](#rate-limiting)
- [Use cases](#use-cases) — load testing, pipelines, seeding
- [Determinism](#determinism)
- [Combining with other features](#combining-with-other-features)

## Unlimited stream

```bash
$ seedfaker name email phone --format jsonl -n 0 --seed demo --until 2025
{"name":"Paulina Laca","email":"im.ivana@eunet.rs","phone":"+90 557 223 7387"}
{"name":"Irene Michaelides","email":"sigitas.staniulis@protonmail.com","phone":"+91 70 36992930"}
{"name":"Elvira Castro Gonzalez","email":"imhannes@omv.com","phone":"+46 71 292 1619"}
...   (continues until Ctrl+C or pipe closes)
```

`-n 0` disables the record limit. Output continues until interrupted (Ctrl+C) or the pipe closes.

## Rate limiting

```bash
seedfaker name email phone --format jsonl -n 0 --rate 1000
```

`--rate N` limits output to N records per second.

## Use cases

### Load testing

```bash
seedfaker name email phone --format jsonl -n 0 --rate 5000 --seed load --until 2025 > load-test.jsonl
```

### Pipeline testing

```bash
seedfaker run nginx -n 0 --rate 100 --seed demo | your-log-processor
```

### Database seeding at controlled rate

```bash
seedfaker name email --format sql=users -n 0 --rate 500 | psql mydb
```

### File generation

```bash
seedfaker name email phone --format jsonl -n 1000000 --seed ci > large-dataset.jsonl
```

No rate limit needed — writes as fast as possible.

## Determinism

Streaming output is deterministic with `--seed`. Record N always produces the same values regardless of rate or interruption point.

## Combining with other features

```bash
# Streaming with corruption
seedfaker name email --format jsonl -n 0 --rate 500 --corrupt mid --seed demo

# Streaming with context
seedfaker name email phone --format jsonl -n 0 --rate 1000 --ctx strict --locale en

# Streaming config
seedfaker run nginx -n 0 --rate 200 --seed demo
```

## Related guides

- [API load testing](../guides/api-load-testing.md) — rate-limited streaming for load tests

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
