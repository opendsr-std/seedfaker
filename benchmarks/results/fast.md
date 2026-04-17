# Quick Benchmark

- **Date:** 2026-04-14 22:57 UTC
- **OS:** Darwin 23.5.0 arm64
- **CPU:** Apple M1 Pro
- **Binary:** seedfaker 0.3.0-alpha.1 (sf0-158dc9f79ce46b43)
- **Records:** 150000
- **Method:** median of 5 runs (1 warm-up discarded)

## Field tiers

| Tier | Fields | Time | Throughput |
|------|--------|------|------------|
| 3 | name, email, phone | 0.064s | 2344K/s |
| 5 | + city, birthdate | 0.083s | 1807K/s |
| 10 | + country, username, postal-code, ssn, credit-card | 0.164s | 915K/s |
| 20 | + address, iban, password, ip, uuid, timestamp, ... | 0.300s | 500K/s |

## Single fields (extremes)

| Field | Time | Throughput | Note |
|-------|------|------------|------|
| boolean | 0.009s | 16667K/s | fastest |
| email | 0.037s | 4054K/s | PII, locale-aware |
| credit-card | 0.022s | 6818K/s | Luhn checksum |
| iban | 0.030s | 5000K/s | per-country format |
| jwt | 0.104s | 1442K/s | base64 encoding |
| ssh-private-key | 0.089s | 1685K/s | heaviest |

## Templates

| Type | Time | Throughput |
|------|------|------------|
| inline (3 fields) | 0.099s | 1515K/s |
| nginx preset (8 fields, conditionals) | 0.189s | 794K/s |
| chaos preset (9 fields, corruption) | 0.423s | 355K/s |

## Feature overhead

Baseline: 3 fields (name, email, phone), 150000 records.

| Feature | Time | vs baseline |
|---------|------|-------------|
| baseline (TSV) | 0.077s | — |
| --format csv | 0.101s | +31% |
| --ctx strict | 0.123s | +60% |
| --corrupt high | 0.149s | +94% |
