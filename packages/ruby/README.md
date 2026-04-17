# seedfaker

Ruby binding for [seedfaker](https://github.com/opendsr-std/seedfaker) — deterministic synthetic data with 200+ fields, 68 locales, same seed = same output.

[CLI](https://github.com/opendsr-std/seedfaker) · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · [Python](https://pypi.org/project/seedfaker/) · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · **Ruby** · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

## Requirements

- Ruby >= 3.1
- `libseedfaker_ffi` shared library (built from Rust source)

> **Pre-1.0:** API may change between minor versions. Pin your version.

## Usage

```ruby
require "seedfaker"

f = Seedfaker::SeedFaker.new(seed: "ci", locale: "en")

f.field("name")                                          # "Zoe Kumar"
f.field("phone", e164: true)                              # "+14155551234"

f.record(["name", "email"], ctx: "strict")               # single record
f.records(["name", "email"], n: 5, ctx: "strict")        # batch
f.validate(["name", "email:e164"])                       # check without generating

f.records(["name", "email"], n: 100, corrupt: "high")    # corrupted data

Seedfaker::SeedFaker.fingerprint                         # "sf0-..."
Seedfaker::SeedFaker.fields                              # all field names
```

## Documentation

- [Quick start](https://github.com/opendsr-std/seedfaker/blob/main/docs/quick-start.md)
- [Field reference (200+ fields)](https://github.com/opendsr-std/seedfaker/blob/main/docs/field-reference.md)
- [Library API](https://github.com/opendsr-std/seedfaker/blob/main/docs/library.md)
- [Guides](https://github.com/opendsr-std/seedfaker/blob/main/guides/) — library usage, seed databases, mock APIs, anonymize data, NER training
- [Full documentation](https://github.com/opendsr-std/seedfaker)

---

## Disclaimer

This software generates synthetic data that may resemble real-world identifiers, credentials, or personal information. All output is artificial. See [LICENSE](https://github.com/opendsr-std/seedfaker/blob/main/LICENSE) for the full legal disclaimer.
