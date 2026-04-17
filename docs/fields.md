# Fields

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

200+ fields across 17 groups. Credit cards pass Luhn, IBANs have valid check digits, SSNs follow locale format rules. See [field reference](field-reference.md) for the full list.

## Contents

- [Quick examples](#quick-examples)
- [Field syntax](#field-syntax) — modifiers, ranges, ordering, transforms
- [Serial](#serial) — sequential IDs and row counters
- [Context-dependent fields](#context-dependent-fields) — coherent identities with `--ctx strict`
- [Range](#range) — constrain numeric and date output
- [Ordering](#ordering) — monotonic asc/desc sequences
- [Modifiers](#modifiers) — field-specific formatting
- [Extended uniqueness](#extended-uniqueness-xuniq) — `:xuniq` for large-scale datasets
- [Transforms](#transforms) — upper/lower/capitalize
- [Enums](#enums) — custom value lists with weights
- [Field groups](#field-groups) — group shortcuts
- [Full reference](#full-reference)

## Quick examples

```bash
# Person records
seedfaker name email phone birthdate ssn --ctx strict --format csv -n 5

# Sequential numbering
seedfaker n=serial name email --format csv -n 1000

# Financial transactions
seedfaker id=serial amount=amount:usd status=enum:completed=8,pending=1,failed=1 --format jsonl -n 1000

# Server logs with ascending timestamps
seedfaker ip timestamp:asc:log http-method url http-status user-agent -n 10000

# Auth credentials
seedfaker username password:16 api-key session-id --format csv -n 100

# Multi-locale with native scripts
seedfaker name email phone -l ja,ko,zh --abc native --ctx strict -n 10

# Computed dataset: price × quantity with running total
seedfaker price=amount:1..500 qty=integer:1..20 "total=price*qty" running=total:sum --format csv -n 100
```

## Field syntax

Fields accept `:` segments in any order: `field[:modifier][:range][:transform][:ordering][:omit=N]`.

```bash
seedfaker phone:e164                  # +14155551234
seedfaker amount:100..5000:usd        # $100–$5,000
seedfaker timestamp:asc:log           # chronological Apache log
seedfaker name:upper                  # HELEN WHITE
seedfaker mac:plain:upper             # A1B2C3D4E5F6
```

`:omit=N` skips generation for N% of rows. Output depends on format: JSONL `null`, SQL `NULL`, CSV empty cell.

```bash
seedfaker email:omit=30                # 30% of rows have no email
seedfaker phone:e164:omit=15           # 15% omitted
```

In configs:

```yaml
columns:
  phone: phone:e164
  salary: amount:30000..150000:usd
  created: timestamp:asc:log:2024..2025
  middle_name: name:omit=40 # 40% omitted
```

## Serial

The only non-random field. `serial` outputs the current row number (0, 1, 2, ...). It ignores `--seed`, always produces the same sequence.

```bash
seedfaker id=serial name email --format csv -n 3 --seed demo --until 2025
# id,name,email
# 0,Paulina Laca,im.ivana@eunet.rs
# 1,Irene Michaelides,sigitas.staniulis@protonmail.com
```

Use `serial` for primary keys, row identifiers, and anything that needs a guaranteed sequential counter. In templates: `{{serial}}` — always available, no declaration needed. In configs: `id: serial`.

`serial` produces 0, 1, 2, ... with no gaps, unaffected by seed. `integer:asc` produces increasing values with gaps (step = range/1M) — use it for monotonic timestamps or amounts, not for IDs.

## Context-dependent fields

Fields like `name`, `email`, `username`, `phone`, and `ssn` are independent by default — each generated from its own seed. With `--ctx strict`, they share a single identity per row: email is derived from the name, username matches, gov IDs follow the locale.

```bash
# Independent — email has no relation to name
seedfaker name email username -n 5

# Coherent — all fields belong to the same person
seedfaker name email username ssn --ctx strict --locale en -n 5
```

See [context](context.md) for the full list of linked fields, `--ctx loose`, and how locale affects identity dispatch.

## Range

Constrain output with `FROM..TO`, `..TO`, or `FROM..`:

```bash
seedfaker integer:1..100              # uniform 1–100
seedfaker amount:100..5000:usd        # $100–$5,000
seedfaker age:21..65                  # working-age adults
seedfaker date:2020..2025             # years 2020–2025
seedfaker timestamp:asc:log -n 1000 --since 2025-03-28T14:00 --until 2025-03-28T16:00
```

### Fields that accept ranges

| Field       | Range unit             | Without range                                       |
| ----------- | ---------------------- | --------------------------------------------------- |
| `integer`   | min..max               | Realistic tiered (more small, fewer large)          |
| `float`     | min..max               | Uniform 0..9999                                     |
| `amount`    | min..max               | Realistic tiered (mostly $1–200, tapering to $10K+) |
| `age`       | min..max years         | Weighted demographic pyramid                        |
| `digits`    | min..max (zero-padded) | Random digit string                                 |
| `date`      | since..until           | `--since`..`--until`                                |
| `birthdate` | since..until           | Weighted age distribution                           |
| `timestamp` | since..until           | `--since`..`--until`                                |

Other fields (`name`, `email`, `uuid`, etc.) do not accept ranges — they generate values from their own dictionaries and formats. With explicit range, distribution is uniform. Without range, `integer` and `amount` produce realistic distributions — more small values, fewer large ones.

### Omitted bounds

`..TO` uses left default (0 for numbers, `--since` for dates). `FROM..` uses right default (999999 for numbers, `--until` for dates).

### Temporal ranges

`--since`/`--until` accept: year (`2025`), date (`2025-03-28`), datetime (`2025-03-28T14:00`), epoch seconds (`1711630800`). Inline ranges accept years and epoch seconds. See [CLI — temporal format](cli.md#temporal-format).

Priority: inline range > CLI flags > config options > defaults.

### Birthdate distribution

Without range, `birthdate` and `age` use a weighted demographic pyramid (more 18–35, fewer 66+). With explicit range (`birthdate:1990..2005`), distribution is uniform.

### Zipf distribution

`:zipf` switches a ranged field from uniform to Zipf (power-law). Low values appear far more often than high — models real FK distributions, page views, purchase counts.

```bash
seedfaker integer:1..50000:zipf -n 100000       # most values cluster near 1
seedfaker integer:1..1000:zipf=0.8 -n 10000     # mild skew
seedfaker integer:1..100:zipf=2 -n 10000        # extreme skew (~60% are 1)
seedfaker amount:1..5000:zipf:usd -n 10000      # Zipf amounts
```

| Exponent | Effect                                    |
| -------- | ----------------------------------------- |
| `0.5`    | Weak skew — close to uniform              |
| `0.8`    | Mild skew                                 |
| `1.0`    | Standard Zipf (default when bare `:zipf`) |
| `1.5`    | Heavy skew                                |
| `2.0`    | Extreme — rank 1 gets ~61% of all values  |

Requires a range. Works on `integer`, `float`, `amount`. Combines with all other segments: `integer:1..50000:zipf:omit=10:upper`.

## Ordering

`:asc` and `:desc` produce monotonic sequences. Combine with any modifier and range:

```bash
seedfaker timestamp:asc:log -n 10000                  # chronological server log
seedfaker amount:asc:100..5000:usd -n 1000            # ascending prices
seedfaker amount:desc:usd -n 100                       # descending prices
```

Step = `range / 1,000,000`. No dependency on `-n` — streaming and batch produce identical sequences. Values clamp at range boundary (no wrap-around).

For sequential IDs (0, 1, 2, ...) use [`serial`](#serial), not `integer:asc`.

Supported: `timestamp`, `date`, `integer`, `float`, `amount`.

## Modifiers

Field-specific modifiers:

```bash
seedfaker phone:e164                  # +14155551234
seedfaker phone:national              # (415) 555-1234
seedfaker credit-card:space           # 4532 1234 5678 9012
seedfaker amount:usd                  # $1,234.56
seedfaker amount:eur                  # €1.234,56
seedfaker amount:plain                # 1234.56 (no symbol)
seedfaker timestamp:unix              # 1710878749
seedfaker timestamp:ms                # 1735668003067
seedfaker timestamp:log               # 15/Feb/2025:21:32:37 +0000
seedfaker date:us                     # 03/28/2025
seedfaker date:eu                     # 28.03.2025
seedfaker password:pin                # 01416
seedfaker password:strong             # VncD+vfMp@?&873gd2sr
seedfaker url:ssh                     # ssh://root@service.prod.net:1132
seedfaker country-code:alpha3         # USA
seedfaker uuid:plain                  # 8d54f5fb531640549488deb7b0d0c3c9
seedfaker color:hex                   # #09fb2a
```

### Length modifier

`digits`, `letters`, `alnum`, `hex`, `base64`, `password` accept a numeric length:

```bash
seedfaker digits:4                    # 0469
seedfaker hex:byte                    # 4e
seedfaker password:16                 # Kx7#mQ9p=LFFv9b!
seedfaker alnum:32                    # xK7m2BqRtY...
```

### Password modifiers

| Modifier     | Output                                      | Use case                                                                         |
| ------------ | ------------------------------------------- | -------------------------------------------------------------------------------- |
| _(default)_  | `cat234`, `123456`, `correct-horse-battery` | Realistic distribution matching leaked password datasets                         |
| `:strong`    | `VncD+vfMp@?&873gd2sr`                      | Strong passwords: 16–24 chars, ≥2 uppercase, ≥2 lowercase, ≥2 digits, ≥2 symbols |
| `:mixed`     | `aIoY1_HlcoSM!oEh`                          | Random from mixed charset, 8–24 chars                                            |
| `:pin`       | `01416`                                     | 4–6 digit PIN                                                                    |
| `:memorable` | `cosmic-river-winter-battery`               | Passphrase, 3–5 words                                                            |
| `:16`        | `Kx7#mQ9p=LFFv9b!`                          | Mixed charset at exact length                                                    |

The default `password` generates values that match real-world password distributions: 40% common leaked passwords, 25% personal word+digits, 20% random mixed, 10% passphrase, 2% PIN. This is designed for training password strength detectors, data quality testing, and realistic test fixtures.

`password:strong` produces passwords that look like output from a password manager. Every password is guaranteed to contain all four character classes and has no three consecutive identical characters.

> **Not for production use.** All seedfaker values — including `password:strong` — are deterministic and derived from a seed. They are synthetic test data, not cryptographically random. Never use generated passwords for real authentication.

## Extended uniqueness (`:xuniq`)

By default, seedfaker generates realistic values — names, emails, and usernames drawn from locale-aware dictionaries. This produces natural-looking data with uniqueness that scales well into hundreds of thousands of records. See the [uniqueness report](../benchmarks/results/uniqueness.md) for measured collision rates.

For datasets where strict uniqueness is required at millions+ of records — database seeding, load testing, deduplication benchmarks — the `:xuniq` modifier extends the value space by appending a deterministic 5-character tag.

```bash
seedfaker email:xuniq -n 20000000 --seed prod         # 0 collisions at 20M
seedfaker username:xuniq -n 5000000 --seed load        # 0 collisions at 5M
```

| Field           | Default                | With `:xuniq`                |
| --------------- | ---------------------- | ---------------------------- |
| `email`         | `john.smith@gmail.com` | `john.smith.k7m4x@gmail.com` |
| `username`      | `jsmith42`             | `jsmith42_k7m4x`             |
| `login-name`    | `john.smith`           | `john.smith.k7m4x`           |
| `nickname`      | `darkwolf42`           | `darkwolf42_k7m4x`           |
| `social-handle` | `@jsmith`              | `@jsmith_k7m4x`              |

The tag is deterministic (same seed + same record = same tag), uses `[a-z0-9]` characters, and does not change the field format — emails remain valid RFC 5321, usernames remain `[a-z0-9_]`.

**Trade-off:** values are slightly longer and less natural-looking. Use `:xuniq` only when uniqueness constraints require it — the default algorithm is already well-suited for typical test datasets.

Supported fields: `email`, `username`, `login-name`, `nickname`, `social-handle`.

## Transforms

Case conversion on any field:

| Transform     | Example       |
| ------------- | ------------- |
| `:upper`      | `HELEN WHITE` |
| `:lower`      | `helen white` |
| `:capitalize` | `Helen white` |

Combine with modifiers: `mac:plain:upper` → `A1B2C3D4E5F6`.

## Enums

Custom value lists with optional weights:

```bash
seedfaker enum:active,suspended,deleted                    # equal probability
seedfaker enum:active=7,inactive=2,suspended=1             # 70/20/10
seedfaker enum:completed=8,pending=1,failed=1 -n 1000      # 80/10/10
```

In configs:

```yaml
columns:
  status: enum:completed=8,pending=1,failed=1
  role: enum:admin=1,user=9
  tier: enum:free=60,pro=30,enterprise=10
```

Locale weights use the same syntax: `--locale en=7,es=2,de=1`. See [CLI — locales](cli.md#locales).

## Field groups

Group names expand to all fields in that group:

```bash
seedfaker person --format csv -n 5         # name, first-name, last-name, ...
seedfaker auth --format jsonl -n 5         # username, password, api-key, ...
```

17 groups: core, text, daily, time, person, contact, location, finance, auth, gov-id, internet, blockchain, organization, healthcare, dev, ops, device.

## Full reference

Auto-generated reference with examples for every field and modifier: **[Field reference →](field-reference.md)**

## Related guides

- [Library usage](../guides/library-usage.md) — fields from Python / Node.js / Go / PHP / Ruby

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
