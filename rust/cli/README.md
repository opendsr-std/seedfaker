# seedfaker

Deterministic synthetic data generator — 200+ fields, 68 locales, multi-table FK, same seed = same output.

**CLI** · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · [Python](https://pypi.org/project/seedfaker/) · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

## Install

```bash
brew install opendsr-std/tap/seedfaker         # macOS / Linux
cargo install seedfaker                        # from source
npm install -g @opendsr/seedfaker-cli          # npm
```

> **Pre-1.0:** API may change. Pin your version.

## Quick start

```bash
seedfaker name email phone --seed demo --until 2025 -n 5
seedfaker name email ssn --ctx strict --locale en --seed ci --until 2025 -n 10
seedfaker run nginx -n 1000 --seed bench --until 2025
seedfaker run shop.yaml --all --output-dir ./data/ --format csv
seedfaker replace email ssn --seed anon < production.csv > safe.csv
seedfaker name email --annotated --seed train --until 2025 -n 1000
```

## Features

- **Multi-table FK** — `users.id:zipf`, `customer_id->name`, computed totals
- **Annotated output** — byte-offset spans for NER/PII training
- **Replace** — anonymize CSV/JSONL with deterministic masking
- **13 presets** — nginx, auth, postgres, payment, pii-leak, syslog, medical
- **Formats** — CSV, TSV, JSONL, SQL, templates with conditionals and loops
- **Streaming** — unlimited output with `-n 0`, rate limiting with `--rate`
- **MCP server** — Model Context Protocol for AI agents

## Documentation

- [Quick start](https://github.com/opendsr-std/seedfaker/blob/main/docs/quick-start.md)
- [CLI reference](https://github.com/opendsr-std/seedfaker/blob/main/docs/cli.md)
- [Field reference (200+ fields)](https://github.com/opendsr-std/seedfaker/blob/main/docs/field-reference.md)
- [Guides](https://github.com/opendsr-std/seedfaker/blob/main/guides/) — library usage, seed databases, mock APIs, anonymize data, NER training
- [Full documentation](https://github.com/opendsr-std/seedfaker)

## License

MIT
