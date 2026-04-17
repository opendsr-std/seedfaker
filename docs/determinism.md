# Determinism

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Same seed produces identical field values. Output format, record count, rate, and headers do not affect what is generated — only how it is printed.

## Contents

- [Seed](#seed) — reproducible output
- [RNG isolation](#rng-isolation) — per-field independence
- [What stays the same](#what-stays-the-same)
- [What changes output](#what-changes-output)

## Seed

`--seed` makes output reproducible. Without it, output is random.

```bash
seedfaker name age --seed demo --until 2025            # reproducible
seedfaker name birthdate --seed demo --since 1990 --until 2005   # pinned range
```

`--until` defaults to now — pin it for reproducibility. Warning printed when `--seed` set without `--until` (suppress with `-q`).

For configs, add [`fingerprint`](configs.md#fingerprint-guard) to lock output across seedfaker upgrades.

## RNG isolation

Each field gets its own RNG derived from `(seed, record number, field name)`. Fields are independently derivable — adding or removing a field does not change other fields' output.

## What stays the same

| Change                                 | Same output?                                  |
| -------------------------------------- | --------------------------------------------- |
| Add/remove fields                      | Yes — other fields unchanged                  |
| Add `--corrupt`                        | Base values same, corruption applied on top   |
| Change output format (csv, jsonl, sql) | Yes                                           |
| Add `--no-header`                      | Yes                                           |
| Change `-n` (record count)             | Existing records unchanged                    |
| `--shard I/N`                          | Row `k` byte-identical across any shard       |
| `--threads N`                          | Main thread writes sub-ranges in serial order |

## What changes output

| Change                    | Why                                                |
| ------------------------- | -------------------------------------------------- |
| Different `--seed`        | Different master seed                              |
| Different `--locale`      | Different locale pool                              |
| Add/remove `--ctx strict` | Identity correlation changes name/email generation |
| Add/remove `--abc native` | Script selection changes name data source          |

Corruption is also deterministic — same seed + same `--corrupt` level = same corrupted output.

## Related guides

- [Reproducible datasets](../guides/reproducible-datasets.md) — pinned fixtures, CI parity, fingerprint guard

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
