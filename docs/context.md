# Context

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

`--ctx` controls whether fields within a record share locale and identity. Name, email, phone, and gov IDs become coherent — one person, one country.

## Contents

- [Modes](#modes) — strict, loose, off
- [Identity fields](#identity-fields) — which fields share identity
- [Gov-ID dispatch](#gov-id-dispatch) — locale-aware documents
- [In config files](#in-config-files)
- [Examples](#examples)

## Modes

### `--ctx strict`

All fields in a record use one locale and one identity.

```bash
seedfaker name email phone --locale en,de --ctx strict -n 3 --seed demo
```

| Field           | Without ctx                | With `--ctx strict`      |
| --------------- | -------------------------- | ------------------------ |
| name            | random locale              | record locale            |
| email           | random locale, random name | derived from record name |
| phone           | random country code        | record country code      |
| address         | random country             | record country           |
| ssn             | always US SSN              | locale-appropriate ID    |
| passport        | random country             | record country format    |
| drivers-license | always US                  | record country format    |
| national-id     | locale-dispatched          | record locale            |
| username        | random                     | derived from record name |

### `--ctx loose`

70% of records lock to one locale. 30% draw from the full locale pool. Identity correlation still applies.

### No flag

Each field independently selects from all configured locales. Name and email are unrelated.

## Identity fields

These fields share a single name when context is active:

`name`, `first-name`, `last-name`, `email`, `username`, `login-name`, `social-handle`

Email and username are derived from the shared name.

## Gov-ID dispatch

`ssn` and `national-id` dispatch by locale:

| Locale                | ssn / national-id        |
| --------------------- | ------------------------ |
| en, en-gb, en-ca, ... | US SSN (123-45-6789)     |
| de                    | Steuer-ID (12345678901)  |
| fr                    | NIR (1 85 12 75 123 456) |
| it                    | Codice Fiscale           |
| es                    | DNI                      |
| ja                    | My Number (12 digits)    |
| zh                    | Shenfenzheng (18 chars)  |
| pt-br                 | CPF                      |
| ru                    | INN                      |
| ...                   | see `national-id` source |

For details on how seed, locale, and context affect output, see [determinism](determinism.md).

## In config files

```yaml
options:
  ctx: strict
  locale: [en, de, fr]
```

## Examples

See [examples/context/](../examples/context/) for output across all 68 locales with `--ctx strict`.

Regenerate: `make field-examples`.

## Related guides

- [Library usage](../guides/library-usage.md) — ctx:strict in Python/Node.js
- [Training and evaluation datasets](../guides/training-data.md) — locale-correlated identities for training sets

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
