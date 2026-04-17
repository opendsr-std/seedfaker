# Reproducible datasets

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Generate deterministic datasets — same seed, same output, every run, every machine. For test fixtures, CI pipelines, or shared staging data.

> [Guides](README.md) · [Quick start](../docs/quick-start.md) · [Determinism](../docs/determinism.md) · [Configs](../docs/configs.md) · [CLI reference](../docs/cli.md)

## Contents

- [The problem](#the-problem)
- [Basic usage](#basic-usage)
- [Config files](#config-files)
- [Multi-table fixtures](#multi-table-fixtures)
- [Cross-language parity](#cross-language-parity)
- [Fingerprint guard](#fingerprint-guard)
- [Validation in CI](#validation-in-ci)
- [GitHub Actions](#github-actions)
- [Tips](#tips)

## The problem

faker.js, Faker, and similar libraries generate random data by default. Tests that depend on random values break unpredictably: snapshot mismatches, flaky assertions, CI reruns that waste time. Pinning a numeric seed helps until the library updates its algorithm and every fixture changes.

seedfaker with `--seed` produces byte-identical output across runs, machines, and languages. A fingerprint guard catches algorithm changes between versions.

## Basic usage

```bash
seedfaker name email --seed ci-2026 --until 2026 -n 100 --format csv > test/fixtures/users.csv
```

Run it twice. Diff the output. Zero differences.

```
name,email
Tariq bin Saif Al Osaimi,echeverriamartina@entelchile.net
Martina De la Cruz Flores,flor.coelho@tap.pt
Bu Fangyi,luzlozano@hey.com
...
```

## Config files

For fixtures with multiple columns, modifiers, or expressions, use a YAML config:

```yaml
columns:
  id: serial
  name: name
  email: email
  role: enum:admin=1,user=9
options:
  seed: ci-2026
  until: 2026
  count: 100
  format: csv
```

```bash
seedfaker run fixtures.yaml > test/fixtures/users.csv
```

## Multi-table fixtures

Define [multiple tables](../docs/multi-table.md) in one config and generate all at once:

```yaml
options:
  seed: ci-2026
  until: "2026"
  format: csv

users:
  columns:
    id: serial
    name: name
    email: email
  options:
    count: 100

orders:
  columns:
    id: serial
    user_id: users.id
    user_name: user_id->name
    amount: amount:plain:1..5000
  options:
    count: 500
```

```bash
seedfaker run fixtures.yaml --all --output-dir test/fixtures/
```

This writes `test/fixtures/users.csv` and `test/fixtures/orders.csv`.

## Cross-language parity

The same seed produces identical values in Python and Node.js tests:

```python
from seedfaker import SeedFaker

rows = SeedFaker(seed="ci-2026", until="2026").records(["name", "email"], 10)
```

```javascript
const { SeedFaker } = require("@opendsr/seedfaker");

const rows = new SeedFaker({ seed: "ci-2026", until: "2026" })
  .records(["name", "email"], { n: 10 });
```

Both produce the same 10 rows.

## Fingerprint guard

seedfaker includes a stable algorithm fingerprint. Pin it in your config to detect algorithm changes between versions:

```yaml
columns:
  name: name
  email: email
options:
  seed: ci-2026
  until: 2026
  count: 50
  format: csv
  fingerprint: sf0-a1b2c3d4
```

Mismatch = immediate failure, not silent drift.

Current fingerprint:

```bash
seedfaker --fingerprint
```

```
sf0-158dc9f79ce46b43
```

## Validation in CI

Check config syntax and field names without generating data:

```bash
seedfaker run fixtures.yaml --validate
```

No output on success, exit code 0.

## GitHub Actions

```yaml
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v5

      - name: Install seedfaker
        run: cargo install seedfaker

      - name: Validate fixture config
        run: seedfaker run fixtures.yaml --all --output-dir test/fixtures/ --validate

      - name: Generate fixtures
        run: seedfaker run fixtures.yaml --all --output-dir test/fixtures/

      - name: Run tests
        run: make test
```

## Tips

Always pin `--until` — without it, temporal fields use current time and fixtures differ across days:

```bash
seedfaker name timestamp --seed ci --until 2026 -n 10   # stable forever
```

Add a Makefile target for regeneration:

```makefile
fixtures:
	seedfaker run fixtures.yaml --all --output-dir test/fixtures/
```

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
