# Benchmarks

> [README](../README.md) · [Docs](../docs/) · [Guides](../guides/) · [Packages](../packages/)

Every number in these reports is produced by calling the same CLI/SDK that users install. No internal binaries, no synthetic harnesses — what you benchmark is what you ship.

Runnable end-to-end Postgres bulk-load benchmark: [`payments_5gb.sh`](payments_5gb.sh).

```bash
./payments_5gb.sh                                # ~100 MB default
./payments_5gb.sh --scale 50 --jobs 10           # ~5 GB, max parallel tables
./payments_5gb.sh --scale 50 --jobs 10 --shards 3 # + 3-way shard on big tables
./payments_5gb.sh --cleanup
```

`--shards N` splits `transactions`, `authorizations`, `ledger_entries` into N parallel `\COPY` streams each. Workflow and tuning rationale: [guides/seed-large-database](../guides/seed-large-database.md). CLI sharding reference: [docs/cli § Sharding](../docs/cli.md#sharding).

## Commands

| Command | What | Output |
|---------|------|--------|
| `make bench` | CLI tiers + per-field | `results/fast.md` + `results/fields.md` |
| `make bench-fast` | CLI throughput (3/5/10/20 fields) | `results/fast.md` |
| `make bench-fields` | Per-field throughput (all 200+ fields) | `results/fields.md` |
| `make bench-tpl` | Template engine (criterion) | stdout |
| `make bench-full` | All + competitor comparison | `results/comparisons.md` |
| `make uniqueness` | Collision rates at scale | `results/uniqueness.md` |
| `make determinism` | Cross-interface SHA-256 proof | `results/determinism.md` |

## What each report covers

**Throughput** (`fast.md`) — end-to-end CLI performance at 3/5/10/20 field tiers, templates, feature overhead. Median of 5 runs, 1 warm-up discarded.

**Per-field** (`fields.md`) — every field individually, measured via CLI (`seedfaker FIELD -n 200000 > /dev/null`). Median of 3 runs.

**Comparisons** (`comparisons.md`) — seedfaker vs 7 competitors across CLI, Python, and Node.js ecosystems. Same field tiers, multiplier column (Nx). Includes honest caveats about field substitutions.

**Uniqueness** (`uniqueness.md`) — collision rates at 100K/1M/5M records, multi-use per entity (×5..×100 aliased columns), field combinations, scale planner. All via direct CLI calls with `sort | uniq`.

**Determinism** (`determinism.md`) — SHA-256 proof that CLI, Python, Node.js, Go, PHP, Ruby, and MCP produce byte-identical output for the same seed.

## Methodology

- All measurements call the public CLI or SDK — never internal functions
- CLI benchmarks: wall-clock via `Time::HiRes`, stdout to `/dev/null`
- Library benchmarks: internal elapsed time reported by each script
- Template engine: criterion framework (statistical, outlier-aware)
- Uniqueness: 20 seeds per measurement, median reported

## CI regression gate

Thresholds (ubuntu-latest shared runner, 150K records):

| Tier | Limit |
|------|-------|
| 3 fields | < 0.25s |
| 10 fields | < 0.60s |
| 20 fields | < 1.20s |

## Competitors

Installed by `./install.sh`:

| Ecosystem | Tools |
|-----------|-------|
| CLI | [fakedata](https://github.com/lucapette/fakedata) (Go) |
| Python | [faker](https://pypi.org/project/Faker/), [mimesis](https://pypi.org/project/mimesis/), [polyfactory](https://pypi.org/project/polyfactory/) |
| Node | [@faker-js/faker](https://www.npmjs.com/package/@faker-js/faker), [chance](https://www.npmjs.com/package/chance), [@ngneat/falso](https://www.npmjs.com/package/@ngneat/falso) |

## Reproduce

```bash
# Quick (no competitors needed):
make bench

# Full suite with competitors:
benchmarks/install.sh
make bench-full

# Uniqueness (takes ~10 min at 1M):
make uniqueness

# Determinism proof:
make determinism
```

---

> [README](../README.md) · [Docs](../docs/) · [Guides](../guides/) · [Packages](../packages/)
