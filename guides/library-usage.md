# Using seedfaker as a library

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)

Generate synthetic data from Python, Node.js, Go, PHP, Ruby, or WASM. Same API, same output across all languages.

> [Guides](README.md) · [Library reference](../docs/library.md) · [Fields](../docs/fields.md) · [Context](../docs/context.md)

## Contents

- [Install](#install)
- [Single value](#single-value)
- [Modifiers](#modifiers)
- [Records](#records)
- [Correlated identity](#correlated-identity)
- [Locales](#locales)
- [Corruption](#corruption)
- [Determinism](#determinism)
- [Field list](#field-list)

## Install

```bash
pip install seedfaker          # Python
npm install @opendsr/seedfaker # Node.js
```

See [library reference](../docs/library.md) for Go, PHP, Ruby, WASM.

## Single value

```python
from seedfaker import SeedFaker

sf = SeedFaker(seed="demo")
sf.field("name")          # "Paulina Laca"
sf.field("email")         # "im.ivana@eunet.rs"
sf.field("phone")         # "+278458384682"
sf.field("uuid")          # "d144d04c-8323-4dc5-b642-aef0e70be3b8"
sf.field("credit-card")   # "4174078583236433"
sf.field("ssn")           # "9580255797203"
```

```js
const { SeedFaker } = require("@opendsr/seedfaker");

const sf = new SeedFaker({ seed: "demo" });
sf.field("name");          // "Paulina Laca"
sf.field("email");         // "im.ivana@eunet.rs"
```

Every call with the same seed returns the same value.

## Modifiers

```python
sf.field("phone", e164=True)           # "+14155551234"
sf.field("amount", usd=True)           # "$793.66"
sf.field("credit-card", space=True)    # "4174 0785 8323 6433"
sf.field("mac", plain=True)            # "A1B2C3D4E5F6"
sf.field("name", upper=True)           # "PAULINA LACA"
```

## Records

One record = one row of data:

```python
sf.record(["name", "email", "phone"])
# {"name": "Paulina Laca", "email": "im.ivana@eunet.rs", "phone": "+278458384682"}
```

Batch:

```python
rows = sf.records(["name", "email", "phone"], n=1000)
# [{"name": "Paulina Laca", ...}, {"name": "Irene Michaelides", ...}, ...]
```

## Correlated identity

By default, fields are independent — name and email are unrelated. With `ctx="strict"`, all fields in a record belong to one person:

```python
sf.record(["name", "email", "phone", "ssn"], ctx="strict")
# name and email match, phone uses the same locale, SSN is country-appropriate
```

```python
# Without ctx: Japanese name + German phone + Brazilian SSN
# With ctx="strict": all fields belong to one German person
sf = SeedFaker(seed="demo", locale="de")
sf.record(["name", "email", "phone", "ssn"], ctx="strict")
```

## Locales

```python
SeedFaker(seed="demo", locale="en")         # English names, US phones
SeedFaker(seed="demo", locale="ja")         # Japanese names
SeedFaker(seed="demo", locale="en=7,de=3")  # 70% English, 30% German
```

68 locales. See [field reference](../docs/field-reference.md) for locale-specific fields.

## Corruption

```python
sf.records(["name", "email"], n=100, corrupt="high")
# OCR errors, mojibake, truncation, field swaps
```

Four levels: `low`, `mid`, `high`, `extreme`. See [corruption](../docs/corruption.md).

## Determinism

```python
a = SeedFaker(seed="test")
b = SeedFaker(seed="test")
assert a.field("name") == b.field("name")  # always true

# Same seed in Node.js produces the same value
```

## Field list

```python
SeedFaker.fields()       # all field names
SeedFaker.fingerprint()  # algorithm version — changes when output would change
```

---

> [README](../README.md) · [Docs](../docs/) · [Guides](README.md) · [Packages](../packages/)
