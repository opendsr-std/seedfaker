# Library

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)

Bindings for Python, Node.js, Go, PHP, Ruby, and browser (WASM). All produce byte-identical output for the same seed — see [Determinism](determinism.md). For LLM / AI-agent integration via the Model Context Protocol, see [MCP](mcp.md).

## Contents

- [Python](#python)
- [JavaScript](#javascript)
- [Go](#go)
- [PHP](#php)
- [Ruby](#ruby)
- [Browser (WASM)](#browser-wasm)
- [API summary](#api-summary)
- [Temporal ranges](#temporal-ranges)
- [Determinism](#determinism)

## Python

```bash
pip install seedfaker
```

```python
from seedfaker import SeedFaker

f = SeedFaker(seed="ci", locale="en", tz="+0300", since=1990, until=2025)

f.field("name")                                          # "Arjun Wolfe"
f.field("phone", e164=True)                              # "+14155551234"
f.field("name", n=3)                                     # ["Arjun Wolfe", ...]
f.record(["name", "email"], ctx="strict")                # {"name": ..., "email": ...}
f.records(["name", "email"], n=5, ctx="strict")          # [{"name": ..., "email": ...}, ...]
f.validate(["name", "email:e164"])                       # ok or raises ValueError

SeedFaker.fields()       # all field names
SeedFaker.fingerprint()  # "sf0-158dc9f79ce46b43"
```

Native PyO3 extension.

## JavaScript

```bash
npm install @opendsr/seedfaker
```

```javascript
const { SeedFaker } = require("@opendsr/seedfaker");

const f = new SeedFaker({ seed: "ci", locale: "en", since: 1990, until: 2025 });

f.field("name"); // "Arjun Wolfe"
f.field("phone", { e164: true }); // "+14155551234"
f.field("name", { n: 3 }); // ["Arjun Wolfe", ...]
f.record(["name", "email"], { ctx: "strict" }); // {name: ..., email: ...}
f.records(["name", "email"], { n: 5, ctx: "strict" }); // [{name: ..., email: ...}, ...]
f.validate(["name", "email:e164"]); // ok or throws

SeedFaker.fields(); // all field names
SeedFaker.fingerprint(); // "sf0-158dc9f79ce46b43"
```

Native NAPI-RS extension.

## Go

```bash
go get github.com/opendsr-std/seedfaker-go
```

```go
f, _ := seedfaker.New(seedfaker.Options{Seed: "ci", Locale: "en"})
defer f.Close()

name, _ := f.Field("name")
rec, _ := f.Record(seedfaker.RecordOpts{Fields: []string{"name", "email"}, Ctx: "strict"})
records, _ := f.Records(seedfaker.RecordOpts{Fields: []string{"name", "email"}, N: 5, Ctx: "strict"})
_ = f.Validate(seedfaker.ValidateOpts{Fields: []string{"name", "email"}})

fields, _ := seedfaker.Fields()
fp, _ := seedfaker.Fingerprint()
```

CGO bindings via `libseedfaker_ffi`.

## PHP

```bash
composer require opendsr/seedfaker
```

```php
use Seedfaker\SeedFaker;

$f = new SeedFaker(seed: "ci", locale: "en");

$f->field("name");
$f->record(["name", "email"], ctx: "strict");
$f->records(["name", "email"], n: 5, ctx: "strict");
$f->validate(["name", "email:e164"]);

SeedFaker::fields();
SeedFaker::fingerprint();
```

Requires PHP FFI extension + `libseedfaker_ffi`.

## Ruby

```bash
gem install seedfaker
```

```ruby
require "seedfaker"

f = Seedfaker::SeedFaker.new(seed: "ci", locale: "en")

f.field("name")
f.record(["name", "email"], ctx: "strict")
f.records(["name", "email"], n: 5, ctx: "strict")
f.validate(["name", "email:e164"])

Seedfaker::SeedFaker.fields
Seedfaker::SeedFaker.fingerprint
```

Fiddle FFI bindings via `libseedfaker_ffi`.

## Browser (WASM)

```bash
npm install @opendsr/seedfaker-wasm
```

Bundler (webpack, vite, rspack):

```js
import { SeedFaker } from "@opendsr/seedfaker-wasm";
```

Plain browser (no bundler):

```js
import { SeedFaker } from "@opendsr/seedfaker-wasm/web";
```

Both require async init:

```js
await SeedFaker.init();

const f = new SeedFaker({ seed: "ci", locale: "en", until: 2025 });
f.field("name");
f.record(["name", "email"], { ctx: "strict" });
f.records(["name", "email"], { n: 5 });
f.validate(["name", "email:e164"]);

SeedFaker.fields();
SeedFaker.fingerprint();
```

`await SeedFaker.init()` required before creating instances.

## API summary

All bindings expose the same 7 entry points:

| Method        | Python                                           | Node.js                       | Go                               | PHP                                | Ruby                                  |
| ------------- | ------------------------------------------------ | ----------------------------- | -------------------------------- | ---------------------------------- | ------------------------------------- |
| Constructor   | `SeedFaker(seed?, locale?, tz?, since?, until?)` | `new SeedFaker({...})`        | `New(Options{...})`              | `new SeedFaker(...)`               | `SeedFaker.new(...)`                  |
| Single field  | `field(field, n?, **opts)`                       | `field(field, {n?, ...opts})` | `Field(field, opts?)`            | `field(field, n?, ...bools)`       | `field(field, n:, **opts)`            |
| Single record | `record(fields, ctx?, corrupt?)`                 | `record(fields, opts?)`       | `Record(fields, ctx, corrupt)`   | `record(fields, ctx?, corrupt?)`   | `record(fields, ctx:, corrupt:)`      |
| Batch         | `records(fields, n, ctx, corrupt)`               | `records(fields, opts)`       | `Records(opts)`                  | `records(fields, n, ctx, corrupt)` | `records(fields, n:, ctx:, corrupt:)` |
| Validate      | `validate(fields, ctx?, corrupt?)`               | `validate(fields, opts?)`     | `Validate(fields, ctx, corrupt)` | `validate(fields, ctx?, corrupt?)` | `validate(fields, ctx:, corrupt:)`    |
| Field list    | `SeedFaker.fields()`                             | `SeedFaker.fields()`          | `Fields()`                       | `SeedFaker::fields()`              | `SeedFaker.fields`                    |
| Fingerprint   | `SeedFaker.fingerprint()`                        | `SeedFaker.fingerprint()`     | `Fingerprint()`                  | `SeedFaker::fingerprint()`         | `SeedFaker.fingerprint`               |

## Locale weights

The `locale` parameter accepts comma-separated codes with optional weights:

```js
// Node.js
new SeedFaker({ locale: "en=7,de=2,fr=1" })  // 70% en, 20% de, 10% fr

// Python
SeedFaker(locale="en=7,de=2,fr=1")

// Go
New(Options{Locale: "en=7,de=2,fr=1"})
```

Omit weights for equal distribution: `"en,de,fr"`. Single locale: `"en"`.

## Temporal ranges

`since` and `until` accept year (`2025`) and epoch seconds (`1711630800`). CLI and configs also accept date (`2025-03-28`) and datetime (`2025-03-28T14:00`). See [temporal format](cli.md#temporal-format).

## Omitted values

`:omit=N` skips generation for N% of rows. In bindings, omitted values return empty strings. CLI outputs format-specific nulls (JSONL `null`, SQL `NULL`, CSV empty cell).

```js
f.field("email", { omit: 30 }); // "" for 30% of calls
f.records(["name", "email:omit=30"], { n: 100 });
```

## Related guides

- [Library usage](../guides/library-usage.md) — Python + Node.js library patterns

---

> [README](../README.md) · [Docs](README.md) · [Guides](../guides/) · [Packages](../packages/)
