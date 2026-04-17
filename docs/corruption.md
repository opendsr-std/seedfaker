# Corruption

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

`--corrupt <level>` corrupts generated values: whitespace noise, encoding errors, OCR artifacts, truncation, masking, field swaps. 15 types across 3 severity tiers.

## Contents

- [Usage](#usage)
- [Levels](#levels) — low, mid, high, extreme
- [Types](#types) — 15 corruptions by severity
- [In config files](#in-config-files)

## Usage

```bash
seedfaker name email --corrupt mid -n 5 --seed demo
```

## Levels

Each field is independently corrupted with probability equal to the level rate. Corrupted fields receive 1 to N passes (stacking), capped by the level.

| Level     | Rate | Max passes | Types available |
| --------- | ---- | ---------- | --------------- |
| `low`     | 5%   | 2          | 0-4             |
| `mid`     | 15%  | 3          | 0-9             |
| `high`    | 45%  | 3          | 0-14            |
| `extreme` | 95%  | 5          | 0-14            |

## Types

### 0-4: subtle (all levels)

| #   | Type                  | Example                      |
| --- | --------------------- | ---------------------------- |
| 0   | Extra spaces          | `John    Smith`              |
| 1   | Invisible characters  | ZWSP, NBSP, soft hyphen      |
| 2   | Unicode decomposition | `é` → `e` + combining accent |
| 3   | Merged words          | `JohnSmith`                  |
| 4   | Duplication           | `John Smith John Smith`      |

### 5-9: visible distortion (medium+)

| #   | Type             | Example               |
| --- | ---------------- | --------------------- |
| 5   | OCR substitution | `J0hn 5m!th`          |
| 6   | Mojibake         | `MÃ¼ller`             |
| 7   | HTML entities    | `O&#39;Brien`         |
| 8   | Garbled suffix   | `john@x.comR4a`       |
| 9   | Field swap       | phone in email column |

### 10-14: data loss (high+)

| #   | Type           | Example               |
| --- | -------------- | --------------------- |
| 10  | Empty value    | _(blank)_             |
| 11  | Truncation     | `John Sm`             |
| 12  | Star redaction | `ou*****tlook.co***m` |
| 13  | Partial mask   | `***-**-1234`         |
| 14  | X-masking      | `XXXXXXXXXXXX1234`    |

For NER/PII annotations with byte-offset spans and original values, see [`--annotated`](annotated.md).

## In config files

```yaml
options:
  corrupt: high
```

Corruption is [deterministic](determinism.md) — same seed + same level = same corrupted output. Base values are generated first, then corruption is applied. Base values are identical with or without `--corrupt`.

## Related guides

- [Training and evaluation datasets](../guides/training-data.md) — noisy training data with byte-offset spans

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
