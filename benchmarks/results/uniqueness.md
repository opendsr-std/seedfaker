# Uniqueness Report

## Default vs `:xuniq`

Median duplicate % across 5 seeds. The `:xuniq` modifier adds a 5-char deterministic tag for guaranteed uniqueness at any scale.

| Field | Mode | 100.0K | 1.00M | 5.00M |
|-------|------|------|------|------|
| `email` | default | 0.00% dup | 0.04% dup | 0.20% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `username` | default | 0.12% dup | 1.04% dup | 4.05% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `login-name` | default | 0.13% dup | 1.15% dup | 4.38% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `nickname` | default | 1.76% dup | 10.48% dup | 19.73% dup |
| | `:xuniq` | 0% | 0% | 0% |
| `social-handle` | default | 0.12% dup | 1.13% dup | 4.38% dup |
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

Measured: 5 seeds × 1.00M records per seed, locale: all.
Seed variance across all fields: <0.1% — results are seed-independent.

### Fields

| Field | Unique | Dup% | Type |
|-------|--------|------|------|
| `name` | 800.4K | 20.0% | medium |
| `first-name` | 7.4K | 99.3% | dictionary |
| `last-name` | 8.9K | 99.1% | dictionary |
| `email` | 999.6K | 0.04% | high-cardinality |
| `username` | 989.6K | 1.0% | medium |
| `nickname` | 895.1K | 10.5% | medium |
| `login-name` | 988.5K | 1.1% | medium |
| `phone` | 999.9K | <0.01% | high-cardinality |
| `phone:e164` | 1000.0K | <0.01% | high-cardinality |
| `address` | 984.1K | 1.6% | medium |
| `city` | 1.6K | 99.8% | dictionary |
| `postal-code` | 86.6K | 91.3% | medium |
| `ssn` | 1000.0K | <0.01% | high-cardinality |
| `passport` | 999.9K | 0.01% | high-cardinality |
| `drivers-license` | 1000.0K | <0.01% | high-cardinality |
| `credit-card` | 1.00M | 0% * | algorithmic |
| `iban` | 1.00M | 0% * | algorithmic |
| `ip` | 999.9K | 0.01% | high-cardinality |
| `ipv6` | 1.00M | 0% * | algorithmic |
| `uuid` | 1.00M | 0% * | algorithmic |
| `jwt` | 1.00M | 0% * | algorithmic |
| `api-key` | 1.00M | 0% * | algorithmic |
| `btc-address` | 1.00M | 0% * | algorithmic |
| `eth-address` | 1.00M | 0% * | algorithmic |
| `company-name` | 2.2K | 99.8% | dictionary |
| `ein` | 999.5K | 0.05% | high-cardinality |
| `employee-id` | 1.00M | 0% * | algorithmic |

\* no collisions observed at 5×1.00M

## Combinations

| Fields | Unique | Dup% |
|--------|--------|------|
| `name,email` | 1.00M | 0% * |
| `name,birthdate` | 1000.0K | <0.01% |
| `name,email,phone` | 1.00M | 0% * |
| `name,email,phone,birthdate` | 1.00M | 0% * |
| `name,email,ssn` | 1.00M | 0% * |
| `ip,username` | 1.00M | 0% * |
| `credit-card,amount` | 1.00M | 0% * |
| `ssn,name` | 1.00M | 0% * |

\* no collisions observed at 5×1.00M

## Scale planner

Median unique % across 5 seeds.

| Fields | 1.0K | 10.0K | 100.0K | 1.00M |
|--------|--------|--------|--------|--------|
| `name` | 100% | 99.8% | 97.5% | 80.0% |
| `email` | 100% | 100% | 100% | 100.0% |
| `username` | 100% | 100% | 99.9% | 99.0% |
| `phone` | 100% | 100% | 100% | 100.0% |
| `credit-card` | 100% | 100% | 100% | 100% |
| `name,email,phone` | 100% | 100% | 100% | 100% |
