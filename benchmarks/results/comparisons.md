# Benchmark Results

## Environment

- **Date:** 2026-04-14 22:58 UTC
- **OS:** Darwin 23.5.0 arm64
- **CPU:** Apple M1 Pro
- **RAM:** 16 GB
- **Rust:** rustc 1.94.1 (e408947bf 2026-03-25)
- **Python:** Python 3.13.2
- **Node:** v22.22.2
- **seedfaker:** seedfaker 0.3.0-alpha.1 (sf0-158dc9f79ce46b43)
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
| seedfaker | 0.045s (2.2M/s) | 0.057s (1.8M/s) | 0.110s (909K/s) | 0.162s (617K/s) | 0.201s (498K/s) |
| fakedata | 0.058s (1.7M/s) · *1.3x slower* | 0.070s (1.4M/s) · *1.2x slower* | 0.086s (1.2M/s) · *1.3x faster* | 0.111s (901K/s) · *1.5x faster* | 0.187s (535K/s) · *1.1x faster* |

## 2. Python library (10000 records, in-memory)

seedfaker: PyO3 native extension. polyfactory: random strings (not structured PII).

| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |
|------|----------|----------|-----------|-----------|-----------|
| seedfaker | 0.010s (1.0M/s) | 0.014s (714K/s) | 0.026s (385K/s) | 0.039s (256K/s) | 0.050s (200K/s) |
| faker | 1.462s (7K/s) · *146.2x slower* | 1.810s (6K/s) · *129.3x slower* | 2.632s (4K/s) · *101.2x slower* | 4.639s (2K/s) · *118.9x slower* | 5.221s (2K/s) · *104.4x slower* |
| mimesis | 0.066s (152K/s) · *6.6x slower* | 0.096s (104K/s) · *6.9x slower* | 0.219s (46K/s) · *8.4x slower* | 0.336s (30K/s) · *8.6x slower* | 0.448s (22K/s) · *9.0x slower* |
| polyfactory | 0.794s (13K/s) · *79.4x slower* | 1.286s (8K/s) · *91.9x slower* | 2.506s (4K/s) · *96.4x slower* | 3.756s (3K/s) · *96.3x slower* | 4.960s (2K/s) · *99.2x slower* |

## 3. Node.js library (10000 records, in-memory)

seedfaker: NAPI-RS native extension.

| Tool | 3 fields | 5 fields | 10 fields | 15 fields | 20 fields |
|------|----------|----------|-----------|-----------|-----------|
| seedfaker | 0.017s (588K/s) | 0.027s (370K/s) | 0.068s (147K/s) | 0.129s (78K/s) | 0.202s (50K/s) |
| fakerjs | 0.081s (123K/s) · *4.8x slower* | 0.119s (84K/s) · *4.4x slower* | 0.193s (52K/s) · *2.8x slower* | 0.281s (36K/s) · *2.2x slower* | 0.376s (27K/s) · *1.9x slower* |
| chance | 0.046s (217K/s) · *2.7x slower* | 0.083s (120K/s) · *3.1x slower* | 0.125s (80K/s) · *1.8x slower* | 0.207s (48K/s) · *1.6x slower* | 0.324s (31K/s) · *1.6x slower* |
| falso | 0.063s (159K/s) · *3.7x slower* | 0.080s (125K/s) · *3.0x slower* | 0.128s (78K/s) · *1.9x slower* | 0.176s (57K/s) · *1.4x slower* | 0.256s (39K/s) · *1.3x slower* |
| jsf | 0.406s (25K/s) · *23.9x slower* | 0.456s (22K/s) · *16.9x slower* | 0.553s (18K/s) · *8.1x slower* | 0.719s (14K/s) · *5.6x slower* | 1.160s (9K/s) · *5.7x slower* |

## 4. Startup overhead (1 record)

| Tool | Time |
|------|------|
| seedfaker CLI | 0.003s |
| faker.py (+ interpreter) | 0.102s |
| mimesis (+ interpreter) | 0.079s |

## 5. Feature overhead (seedfaker CLI, 100000 records)

Baseline: 3 PII fields (name, email, phone), TSV to /dev/null.

| Feature | Time | Overhead |
|---------|------|----------|
| baseline (TSV) | 0.045s | — |
| --format csv | 0.062s | +38% |
| --ctx strict | 0.083s | +84% |
| --corrupt high | 0.100s | +122% |

### Template overhead (same fields: TSV vs inline template vs YAML config)

| Fields | TSV | Inline `-t` | YAML config | TPL vs TSV |
|--------|-----|-------------|-----------|------------|
| 3 | 0.045s | 0.068s | 0.071s | +51% |
| 5 | 0.059s | 0.103s | 0.093s | +75% |
| 10 | 0.112s | 0.175s | 0.177s | +56% |
| 15 | 0.166s | 0.260s | 0.265s | +57% |
| 20 | 0.210s | 0.322s | 0.323s | +53% |

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

