# Replace

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Anonymize existing data by replacing selected columns with synthetic values. Input auto-detected as CSV or JSONL.

## Contents

- [CSV](#csv)
- [JSONL](#jsonl)
- [Temporal range](#temporal-range) — constrain generated dates
- [Determinism](#determinism) — same input + same seed = same output
- [Example workflow](#example-workflow)

## CSV

```bash
seedfaker replace email phone ssn --seed anon < input.csv > output.csv
```

```
# input.csv                          # output.csv
name,email,phone                     name,email,phone
Alice Chen,alice@corp.com,555-1234   Alice Chen,nolan.moreno.xxy@icloud.com,+1 (744) 555-2784
Bob Wilson,bob@work.org,555-5678     Bob Wilson,karterreid@ge.com,511-620-2275
```

- Reads header from first line
- Identifies columns by name
- Replaces values with deterministic synthetic data
- Preserves quotes and structure
- Unspecified columns pass through unchanged

## JSONL

```bash
$ seedfaker replace email --seed anon --until 2025 < input.jsonl
{"age":32,"email":"rohitquinn43@aol.com","name":"Alice Chen"}
{"age":45,"email":"emiliorobe7944@aol.com","name":"Bob Wilson"}
```

- Each line treated as a JSON object
- Specified keys replaced with synthetic values
- Null and empty values skipped
- Other keys pass through unchanged

## Temporal range

`--since` and `--until` constrain generated dates and timestamps in replaced columns:

```bash
seedfaker replace email birthdate --seed anon --since 1990 --until 2005 < input.csv > output.csv
```

Accepts year (`2025`), date (`2025-03-28`), datetime (`2025-03-28T14:00`), or epoch seconds. See [temporal format](cli.md#temporal-format).

## Determinism

With `--seed`, the same input value always maps to the same output value:

```
john@example.com → arjun.wolfe@fastmail.com   (always, with --seed anon)
jane@example.com → petra.schluter@brenntag.de  (always, with --seed anon)
```

Without `--seed`, replacements are random.

## Example workflow

```bash
# Export from database
psql -c "COPY users TO STDOUT CSV HEADER" mydb > raw.csv

# Anonymize PII columns
seedfaker replace email phone ssn --seed prod-anon < raw.csv > safe.csv

# Use in development
psql -c "\COPY users FROM 'safe.csv' CSV HEADER" devdb
```

## Related guides

- [Anonymise production data](../guides/anonymize-data.md) — anonymise CSV/JSONL dumps while preserving FK integrity

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
