# Uniqueness Report

## Default vs `:xuniq`

Median duplicate % across 5 seeds. The `:xuniq` modifier adds a 5-char deterministic tag for guaranteed uniqueness at any scale.

| Field | Mode | 10.0K | 100.0K | 500.0K |
|-------|------|------|------|------|
| `email` | default | 0% | 0.00% dup | 0.02% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `username` | default | 0.02% dup | 0.12% dup | 0.54% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `login-name` | default | 0.01% dup | 0.13% dup | 0.61% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `nickname` | default | 0.18% dup | 1.76% dup | 6.71% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `social-handle` | default | 0.01% dup | 0.12% dup | 0.59% dup |
| | `:xuniq` | 0% | 0% | 0% |

\* zero collisions observed

See [fields — extended uniqueness](../docs/fields.md#extended-uniqueness-xuniq) for details.

---

## Multi-use per entity

When a single record draws the same field type N times (e.g. `doctor=name patient=name nurse=name`), duplicates within one row break realism. This table shows the median number of duplicate values per record (5 seeds × 1000 rows). 0 = all values distinct in the typical row.

| Field | ×5 | ×10 | ×25 | ×50 | ×100 |
|-------|------|------|------|------|------|
| `email` | 0 | 0 | 0 | 0 | 0 |
| `username` | 0 | 0 | 0 | 0 | 0 |
| `first-name` | 0 | 0 | 0 | 0 | 2 |
| `last-name` | 0 | 0 | 0 | 0 | 1 |
| `phone` | 0 | 0 | 0 | 0 | 0 |
| `city` | 0 | 0 | 0 | 1 | 4 |
| `ip` | 0 | 0 | 0 | 0 | 0 |
| `address` | 0 | 0 | 0 | 0 | 0 |
| `jwt` | 0 | 0 | 0 | 0 | 0 |
| `credit-card` | 0 | 0 | 0 | 0 | 0 |
| `passport` | 0 | 0 | 0 | 0 | 0 |
| `birthdate` | 0 | 0 | 0 | 0 | 0 |
| `uuid` | 0 | 0 | 0 | 0 | 0 |

Fields with large value spaces (`email`, `phone`, `ip`, `credit-card`, `jwt`, `passport`) produce zero in-row collisions at any practical multiplicity. Dictionary-bounded fields (`first-name`, `last-name`, `city`) follow birthday-paradox statistics — collisions grow as draws approach dictionary size.

---

## All fields

Measured: 5 seeds × 100.0K records per seed, locale: all.
Seed variance across all fields: <0.1% — results are seed-independent.

### Fields

| Field | Unique | Dup% | Type |
|-------|--------|------|------|
| `name` | 97.5K | 2.5% | medium |
| `first-name` | 7.4K | 92.6% | dictionary |
| `last-name` | 8.9K | 91.1% | dictionary |
| `email` | 100.0K | <0.01% | high-cardinality |
| `username` | 99.9K | 0.12% | high-cardinality |
| `nickname` | 98.3K | 1.7% | medium |
| `login-name` | 99.9K | 0.13% | high-cardinality |
| `phone` | 100.0K | <0.01% | algorithmic |
| `phone:e164` | 100.0K | 0% * | algorithmic |
| `address` | 99.8K | 0.18% | high-cardinality |
| `city` | 1.6K | 98.4% | dictionary |
| `postal-code` | 47.7K | 52.3% | medium |
| `ssn` | 100.0K | 0% * | algorithmic |
| `passport` | 100.0K | <0.01% | high-cardinality |
| `drivers-license` | 100.0K | 0% * | algorithmic |
| `credit-card` | 100.0K | 0% * | algorithmic |
| `iban` | 100.0K | 0% * | algorithmic |
| `ip` | 100.0K | <0.01% | high-cardinality |
| `ipv6` | 100.0K | 0% * | algorithmic |
| `uuid` | 100.0K | 0% * | algorithmic |
| `jwt` | 100.0K | 0% * | algorithmic |
| `api-key` | 100.0K | 0% * | algorithmic |
| `btc-address` | 100.0K | 0% * | algorithmic |
| `eth-address` | 100.0K | 0% * | algorithmic |
| `company-name` | 2.2K | 97.8% | dictionary |
| `ein` | 100.0K | <0.01% | high-cardinality |
| `employee-id` | 100.0K | 0% * | algorithmic |

\* no collisions observed at 5×100.0K

## Combinations

| Fields | Unique | Dup% |
|--------|--------|------|
| `name,email` | 100.0K | 0% * |
| `name,birthdate` | 100.0K | 0% * |
| `name,email,phone` | 100.0K | 0% * |
| `name,email,phone,birthdate` | 100.0K | 0% * |
| `name,email,ssn` | 100.0K | 0% * |
| `ip,username` | 100.0K | 0% * |
| `credit-card,amount` | 100.0K | 0% * |
| `ssn,name` | 100.0K | 0% * |

\* no collisions observed at 5×100.0K

## Scale planner

Median unique % across 5 seeds.

| Fields | 1.0K | 10.0K | 100.0K |
|--------|--------|--------|--------|
| `name` | 100% | 99.8% | 97.5% |
| `email` | 100% | 100% | 100% |
| `username` | 100% | 100% | 99.9% |
| `phone` | 100% | 100% | 100% |
| `credit-card` | 100% | 100% | 100% |
| `name,email,phone` | 100% | 100% | 100% |
