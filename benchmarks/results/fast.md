# Quick Benchmark

- **Date:** 2026-05-03 16:34 UTC
- **OS:** Darwin 23.5.0 arm64
- **CPU:** Apple M1 Pro
- **Binary:** seedfaker 0.3.0-alpha.2 (sf0-158dc9f79ce46b43)
- **Records:** 150000
- **Method:** median of 5 runs (1 warm-up discarded)

## Field tiers

| Tier | Fields | Time | Throughput |
|------|--------|------|------------|
| 3 | name, email, phone | 0.064s | 2344K/s |
| 5 | + city, birthdate | 0.080s | 1875K/s |
| 10 | + country, username, postal-code, ssn, credit-card | 0.155s | 968K/s |
| 20 | + address, iban, password, ip, uuid, timestamp, ... | 0.278s | 540K/s |

## Single fields (extremes)

| Field | Time | Throughput | Note |
|-------|------|------------|------|
| boolean | 0.008s | 18750K/s | fastest |
| email | 0.035s | 4286K/s | PII, locale-aware |
| credit-card | 0.021s | 7143K/s | Luhn checksum |
| iban | 0.029s | 5172K/s | per-country format |
| jwt | 0.091s | 1648K/s | base64 encoding |
| ssh-private-key | 0.087s | 1724K/s | heaviest |

## Templates

| Type | Time | Throughput |
|------|------|------------|
| inline (3 fields) | 0.093s | 1613K/s |
| nginx preset (8 fields, conditionals) | 0.180s | 833K/s |
| chaos preset (9 fields, corruption) | 0.397s | 378K/s |

## Feature overhead

Baseline: 3 fields (name, email, phone), 150000 records.

| Feature | Time | vs baseline |
|---------|------|-------------|
| baseline (TSV) | 0.061s | — |
| --format csv | 0.087s | +43% |
| --ctx strict | 0.114s | +87% |
| --corrupt high | 0.136s | +123% |
