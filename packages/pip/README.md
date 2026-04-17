# seedfaker

Python binding for [seedfaker](https://github.com/opendsr-std/seedfaker) — deterministic synthetic data with 200+ fields, 68 locales, same seed = same output.

[CLI](https://github.com/opendsr-std/seedfaker) · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · **Python** · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

```bash
pip install seedfaker
```

> **Pre-1.0 notice:** The API may change between minor versions until 1.0.0 is released. Pin your version and check [CHANGELOG.md](https://github.com/opendsr-std/seedfaker/blob/main/CHANGELOG.md) before upgrading.

## Python API

```python
from seedfaker import SeedFaker

f = SeedFaker(seed="ci", locale="en")

# Single values
f.field("name")
f.field("phone", e164=True)
f.field("credit-card", space=True)
f.field("email", omit=30)                    # "" for 30% of calls

# Weighted locales: 70% English, 20% German, 10% French
mixed = SeedFaker(seed="ci", locale="en=7,de=2,fr=1")

# Single correlated record
f.record(["name", "email", "phone"], ctx="strict")
# → {"name": "Zoe Kumar", "email": "zoe.kumar@...", "phone": "+1..."}

# Batch
f.records(["name", "email", "phone"], n=5, ctx="strict")

# Validate without generating
f.validate(["name", "email:e164"])  # raises ValueError if invalid

# Corruption
f.records(["name", "email", "ssn"], n=100, corrupt="high")

# Determinism — same seed = same output
a = SeedFaker(seed="test")
b = SeedFaker(seed="test")
assert a.field("name") == b.field("name")

# Fingerprint — detect algorithm changes after upgrade
SeedFaker.fingerprint()  # 'sf0-...'

# All field names
SeedFaker.fields()
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
