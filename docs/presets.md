# Presets

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

13 embedded configs for common data patterns.

## Contents

- [Available presets](#available-presets)
- [Usage](#usage)
- [Source](#source)

## Available presets

| Preset       | Description                        | Format         |
| ------------ | ---------------------------------- | -------------- |
| `nginx`      | Nginx combined access log          | Template       |
| `auth`       | SSH auth.log events                | Template       |
| `app-json`   | Structured application log (JSON)  | JSONL template |
| `postgres`   | PostgreSQL audit log               | Template       |
| `payment`    | Payment transactions (Stripe-like) | JSONL template |
| `pii-leak`   | Unstructured text with PII         | Template       |
| `user-table` | User records                       | CSV            |
| `email`      | Email conversations (MIME)         | Multiline      |
| `stacktrace` | Java/Python exceptions with PII    | Multiline      |
| `chaos`      | Mixed formats + corruption         | Template       |
| `llm-prompt` | AI/LLM prompt-response pairs       | JSONL template |
| `syslog`     | Structured key=value system logs   | Template       |
| `medical`    | Healthcare records (HL7-style)     | JSONL template |

## Usage

```bash
seedfaker run nginx -n 1000 --seed demo
seedfaker run pii-leak -n 100 --corrupt mid --annotated
seedfaker run --list                                      # list all presets
```

In configs, presets serve as starting points — copy and customize:

```bash
seedfaker run user-table --seed ci -n 10000 --format csv > fixtures.csv
```

## Source

Preset YAML files: [`rust/cli/src/presets/`](https://github.com/opendsr-std/seedfaker/tree/main/rust/cli/src/presets)

## Related guides

- [Mock API server](../guides/mock-api-server.md) — plug a preset into Express/FastAPI
- [API load testing](../guides/api-load-testing.md) — stream nginx/auth/payment presets to endpoints

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
