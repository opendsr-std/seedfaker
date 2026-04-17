# @opendsr/seedfaker-wasm

Browser WASM build of [seedfaker](https://github.com/opendsr-std/seedfaker) — deterministic synthetic data with 200+ fields, 68 locales. Runs entirely in the browser, no server.

[CLI](https://github.com/opendsr-std/seedfaker) · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · [Python](https://pypi.org/project/seedfaker/) · **Browser/WASM** · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

```bash
npm install @opendsr/seedfaker-wasm
```

> **Pre-1.0:** API may change between minor versions. Pin your version.

## Bundler (webpack, vite, rspack)

```js
import { SeedFaker } from "@opendsr/seedfaker-wasm";

await SeedFaker.init();

const f = new SeedFaker({ seed: "demo", locale: "en", until: 2025 });
f.field("name");                                         // "Zoe Kumar"
f.record(["name", "email"], { ctx: "strict" });          // { name: ..., email: ... }
f.records(["name", "email"], { n: 5 });                  // [{ ... }, ...]
f.validate(["name", "email:e164"]);                      // throws if invalid

SeedFaker.fields();                                      // all field names
SeedFaker.fingerprint();                                 // "sf0-..."
```

## Plain browser (no bundler)

```html
<script type="module">
import { SeedFaker } from "@opendsr/seedfaker-wasm/web";

await SeedFaker.init();
const f = new SeedFaker({ seed: "demo" });
console.log(f.field("name"));
</script>
```

## API

`await SeedFaker.init()` required once before creating instances.

| Method | Description |
|--------|-------------|
| `SeedFaker.init()` | Load WASM (async, call once) |
| `new SeedFaker(opts?)` | Create instance |
| `field(name, opts?)` | Single field value |
| `record(fields, opts?)` | Single record |
| `records(fields, opts?)` | Batch records |
| `validate(fields, opts?)` | Validate without generating |
| `SeedFaker.fields()` | All field names |
| `SeedFaker.fingerprint()` | Algorithm fingerprint |

## Documentation

- [Quick start](https://github.com/opendsr-std/seedfaker/blob/main/docs/quick-start.md)
- [Field reference (200+ fields)](https://github.com/opendsr-std/seedfaker/blob/main/docs/field-reference.md)
- [Library API](https://github.com/opendsr-std/seedfaker/blob/main/docs/library.md)
- [Guides](https://github.com/opendsr-std/seedfaker/blob/main/guides/) — library usage, seed databases, mock APIs, anonymize data, NER training
- [Full documentation](https://github.com/opendsr-std/seedfaker)

---

## Disclaimer

This software generates synthetic data that may resemble real-world identifiers, credentials, or personal information. All output is artificial. See [LICENSE](https://github.com/opendsr-std/seedfaker/blob/main/LICENSE) for the full legal disclaimer.
