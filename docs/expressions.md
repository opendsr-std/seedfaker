# Expressions and aggregators

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Arithmetic between columns and running totals across records. Derive computed values without external tools.

## Contents

- [Expressions](#expressions) — arithmetic between columns
- [Operators](#operators)
- [Type system](#type-system) — which fields support arithmetic
- [Chaining](#chaining) — intermediate columns
- [Aggregators](#aggregators) — running sum/count
- [Grouped aggregators](#grouped-aggregators) — per-key totals
- [Combining expressions and aggregators](#combining-expressions-and-aggregators)

## Expressions

Arithmetic between numeric columns using `+`, `-`, `*`:

```bash
# Price x quantity = subtotal
seedfaker price=amount:1..500 qty=integer:1..20 "total=price*qty" --seed shop --format csv -n 5 --until 2025

# Base + bonus
seedfaker base=integer:30000..80000 bonus=integer:1000..10000 "total=base+bonus" --seed hr --format csv -n 5 --until 2025

# Margin calculation
seedfaker cost=amount:10..100 markup=integer:10..50 "revenue=cost+markup" "profit=revenue-cost" --seed biz --format csv -n 5 --until 2025
```

Whitespace around operators is optional: `price*qty`, `price * qty`, `price *qty` all work.

## Operators

| Op  | Example          | Result     |
| --- | ---------------- | ---------- |
| `+` | `base + bonus`   | Sum        |
| `-` | `revenue - cost` | Difference |
| `*` | `price * qty`    | Product    |

One operator per expression. For complex formulas, chain through [intermediate columns](#chaining).

## Type system

Only fields with numeric output support arithmetic:

`integer`, `float`, `amount`, `timestamp`, `date`, `age`, `digit`, `bit`, `trit`, `dice`, `serial`, `latitude`, `longitude`, `port`, `http-status`, `latency`

Type compatibility is validated at compile time:

| Left      | Right     | Result                       |
| --------- | --------- | ---------------------------- |
| Int       | Int       | Int                          |
| Int       | Float     | Float                        |
| Float     | Float     | Float                        |
| Money     | Money     | Money                        |
| Money     | Int/Float | Money                        |
| Date      | Int       | Date (delta in days)         |
| Timestamp | Int       | Timestamp (delta in seconds) |
| Date      | Date      | Int (difference in days)     |
| Timestamp | Timestamp | Int (difference in seconds)  |

Non-numeric fields (`name`, `email`, `uuid`) produce an error. Incompatible types (`date + money`) produce an error.

### Date and timestamp arithmetic

Date arithmetic operates in days, timestamp arithmetic in seconds:

```bash
# Start date + random duration in days
seedfaker start=date:2024..2025 days=integer:1..90 "end=start+days" --seed demo --format csv -n 5 --until 2025
```

### Column references

A column value can be referenced by name. With a modifier, the raw value is reformatted:

```yaml
columns:
  base: amount:30000..80000
  base_usd: base:usd # same value as base, formatted as $30,000.00
  total: price * qty # expression result
  display: total:usd # total reformatted as $1,234.56
  start: date:2024..2025
  end_date: start + days # date arithmetic
  formatted: end_date:us # end_date reformatted as MM/DD/YYYY
```

Without modifier, the reference copies the value as-is. With modifier, the raw numeric value is reformatted (works with amount→usd/eur/gbp, date→us/eu, timestamp→unix/ms).

### Hyphenated field names

`-` in field names (`user-agent`, `http-status`) is not confused with subtraction — the parser checks the field registry first.

## Chaining

For complex formulas, use intermediate columns. Columns can be declared in any order — the engine resolves the dependency graph automatically:

```yaml
columns:
  price: amount:10..500:plain
  qty: integer:1..20
  subtotal: price * qty
  discount: amount:0..50:plain
  total: subtotal - discount
```

`total` depends on `subtotal`, which depends on `price` and `qty`. Declaration order does not matter — the engine computes the topological order at compile time. Circular dependencies produce an error.

## Aggregators

Running totals and per-group counters across records:

```bash
# Running total
seedfaker amount=amount:plain total=amount:sum --seed demo --format csv -n 5 --until 2025
# amount,total
# 238554.54,238554.54
# 771.97,239326.51
# 5.31,239331.82
```

### Functions

| Syntax          | Description                      |
| --------------- | -------------------------------- |
| `col:sum`       | Running total of column values   |
| `col:sum=group` | Running total per group          |
| `col:count`     | Running count of distinct values |

### Auto-naming

| Definition       | Column name         |
| ---------------- | ------------------- |
| `amount:sum`     | `sum_amount`        |
| `amount:sum=uid` | `sum_amount_by_uid` |
| `uid:count`      | `count_uid`         |

Use `name=source:sum` for explicit names: `balance=amount:sum`.

## Grouped aggregators

Group by another column for per-key totals:

```bash
# Per-customer running balance
seedfaker cid=integer:0..10 val=amount:plain balance=val:sum=cid --seed shop --format csv -n 20 --until 2025
```

Each unique `cid` value maintains its own running sum.

## Combining expressions and aggregators

Aggregators can read from expression results:

```yaml
columns:
  price: amount:10..500:plain
  qty: integer:1..20
  subtotal: price * qty
  running: subtotal:sum
  order_num: price:count

options:
  seed: shop
  until: "2025"
  format: csv
```

## Related guides

- [Reproducible datasets](../guides/reproducible-datasets.md) — computed columns in fixtures

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
