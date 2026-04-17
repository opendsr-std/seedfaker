# @opendsr/seedfaker-cli

CLI for [seedfaker](https://github.com/opendsr-std/seedfaker) via npm — deterministic synthetic data with 200+ fields, 68 locales, multi-table FK, and streaming.

[**CLI**](https://github.com/opendsr-std/seedfaker) · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · [Python](https://pypi.org/project/seedfaker/) · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

## Install

```bash
npm install -g @opendsr/seedfaker-cli
```

Or via npx:

```bash
npx @opendsr/seedfaker-cli name email phone -n 5 --seed demo --until 2025
```

## Usage

```bash
seedfaker name email phone --seed ci --until 2025 -n 10
seedfaker name email --format csv -n 10000 --seed ci --until 2025 > fixtures.csv
seedfaker run nginx -n 0 --rate 5000 --seed prod
seedfaker replace name email < users.csv > anonymized.csv
```

## Documentation

- [CLI reference](https://github.com/opendsr-std/seedfaker/blob/main/docs/cli.md)
- [Field reference (200+ fields)](https://github.com/opendsr-std/seedfaker/blob/main/docs/field-reference.md)
- [Guides](https://github.com/opendsr-std/seedfaker/blob/main/guides/) — library usage, seed databases, mock APIs, anonymize data, NER training
- [Full documentation](https://github.com/opendsr-std/seedfaker)

---

## Disclaimer

This software generates synthetic data that may resemble real-world identifiers, credentials, or personal information. All output is artificial. See [LICENSE](https://github.com/opendsr-std/seedfaker/blob/main/LICENSE) for the full legal disclaimer.
