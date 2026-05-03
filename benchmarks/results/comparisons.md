# Benchmark Results

## Environment

- **Date:** 2026-05-03 16:34 UTC
- **OS:** Darwin 23.5.0 arm64
- **CPU:** Apple M1 Pro
- **RAM:** 16 GB
- **Rust:** rustc 1.95.0 (59807616e 2026-04-14)
- **Python:** Python 3.13.2
- **Node:** v22.22.2
- **seedfaker:** seedfaker 0.3.0-alpha.2 (sf0-158dc9f79ce46b43)
- **faker:** 40.11.0
- **mimesis:** 19.1.0
- **polyfactory:** 3.3.0
- **@faker-js/faker:** 9.9.0
- **chance:** 1.1.13
- **@ngneat/falso:** 7.4.0
- **json-schema-faker:** unknown
- **fakedata:** main
- **Method:** median of 5 runs (1 warm-up discarded)

## 1. CLI throughput (100000 records, stdout > /dev/null)

Both tools generate to /dev/null. seedfaker produces format-realistic PII (Luhn credit cards, IBAN check digits, locale-aware gov IDs). fakedata uses simpler generators — see [field substitutions](#field-substitutions) below.

| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |
|------|----------|----------|-----------|-----------|-----------|
| seedfaker | 0.045s (2.2M/s) | 0.058s (1.7M/s) | 0.108s (926K/s) | 0.158s (633K/s) | 0.195s (513K/s) |
| fakedata | 0.054s (1.9M/s) · *1.2x slower* | 0.068s (1.5M/s) · *1.2x slower* | 0.085s (1.2M/s) · *1.3x faster* | 0.106s (943K/s) · *1.5x faster* | 0.188s (532K/s) · *~same* |

## 2. Python library (10000 records, in-memory)

seedfaker: PyO3 native extension. polyfactory: random strings (not structured PII).

| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |
|------|----------|----------|-----------|-----------|-----------|
| seedfaker | 0.010s (1.0M/s) | 0.014s (714K/s) | 0.026s (385K/s) | 0.040s (250K/s) | 0.051s (196K/s) |
| faker | 1.375s (7K/s) · *137.5x slower* | 1.772s (6K/s) · *126.6x slower* | 2.554s (4K/s) · *98.2x slower* | 4.585s (2K/s) · *114.6x slower* | 5.082s (2K/s) · *99.6x slower* |
| mimesis | 0.066s (152K/s) · *6.6x slower* | 0.098s (102K/s) · *7.0x slower* | 0.222s (45K/s) · *8.5x slower* | 0.341s (29K/s) · *8.5x slower* | 0.434s (23K/s) · *8.5x slower* |
| polyfactory | 0.775s (13K/s) · *77.5x slower* | 1.256s (8K/s) · *89.7x slower* | 2.461s (4K/s) · *94.7x slower* | 3.733s (3K/s) · *93.3x slower* | 4.897s (2K/s) · *96.0x slower* |

## 3. Node.js library (10000 records, in-memory)

seedfaker: NAPI-RS native extension.

| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |
|------|----------|----------|-----------|-----------|-----------|
| seedfaker | 0.021s (476K/s) | 0.027s (370K/s) | 0.067s (149K/s) | 0.124s (81K/s) | 0.198s (51K/s) |
| fakerjs | 0.079s (127K/s) · *3.8x slower* | 0.119s (84K/s) · *4.4x slower* | 0.187s (53K/s) · *2.8x slower* | 0.262s (38K/s) · *2.1x slower* | 0.354s (28K/s) · *1.8x slower* |
| chance | 0.043s (233K/s) · *2.0x slower* | 0.076s (132K/s) · *2.8x slower* | 0.123s (81K/s) · *1.8x slower* | 0.191s (52K/s) · *1.5x slower* | 0.328s (30K/s) · *1.7x slower* |
| falso | 0.058s (172K/s) · *2.8x slower* | 0.078s (128K/s) · *2.9x slower* | 0.122s (82K/s) · *1.8x slower* | 0.172s (58K/s) · *1.4x slower* | 0.252s (40K/s) · *1.3x slower* |
| jsf | 0.374s (27K/s) · *17.8x slower* | 0.438s (23K/s) · *16.2x slower* | 0.526s (19K/s) · *7.9x slower* | 0.691s (14K/s) · *5.6x slower* | 1.090s (9K/s) · *5.5x slower* |

## 4. Startup overhead (1 record)

| Tool | Time |
|------|------|
| seedfaker CLI | 0.003s |
| faker.py (+ interpreter) | 0.100s |
| mimesis (+ interpreter) | 0.077s |

## 5. Feature overhead (seedfaker CLI, 100000 records)

Baseline: 3 PII fields (name, email, phone), TSV to /dev/null.

| Feature | Time | Overhead |
|---------|------|----------|
| baseline (TSV) | 0.045s | — |
| --format csv | 0.060s | +33% |
| --ctx strict | 0.080s | +78% |
| --corrupt high | 0.096s | +113% |

### Template overhead (same fields: TSV vs inline template vs YAML config)

| Fields | TSV | Inline `-t` | YAML config | TPL vs TSV |
|--------|-----|-------------|-----------|------------|
| 3 | 0.043s | 0.065s | 0.066s | +51% |
| 5 | 0.057s | 0.092s | 0.091s | +61% |
| 10 | 0.106s | 0.169s | 0.169s | +59% |
| 15 | 0.158s | 0.249s | 0.249s | +58% |
| 20 | 0.192s | 0.309s | 0.311s | +61% |

## Methodology

- **Timing:** median of 5 runs, 1 warm-up discarded.
- **CLI:** wall-clock via `Time::HiRes`, stdout to /dev/null.
- **Library:** internal elapsed time reported by each script.
- **Template engine:** criterion framework (statistical, outlier-aware).

## Field tiers

| Tier | seedfaker fields | Notes |
|------|------------------|-------|
| 3 | name, email, phone | Common PII |
| 5 | + city, birthdate | Demographic |
| 10 | + country, username, postal-code, ssn, credit-card | With checksum validation |
| 15 | + address, company-name, job-title, iban, password | Heavy formatting |
| 20 | + ip, uuid, timestamp, passport, national-id | Full PII set |

## Field substitutions

Not all tools support the same fields. Where a tool lacks an equivalent, the closest available generator is used. This affects comparisons at 10+ fields.

| seedfaker field | fakedata substitute | Impact |
|-----------------|---------------------|--------|
| ssn | `int` | No format validation |
| credit-card | `int` | No Luhn checksum |
| iban | `domain` | Different complexity |
| passport | `int` | No format rules |
| national-id | `int` | No locale dispatch |

**polyfactory** generates unstructured random strings for all `str` fields. **@ngneat/falso** does not support deterministic seeding.

## Why native bindings are faster

seedfaker Python and Node.js packages call the same compiled Rust core via native extensions (PyO3/NAPI-RS). Pure-Python and pure-JS libraries run interpreted code per field per record — the gap is inherent to the runtime, not a quality difference.

## Reproduce

```bash
benchmarks/install.sh
make bench-full
```

## Per-field performance

See [results/fields.md](results/fields.md) (`make bench-fields`).

