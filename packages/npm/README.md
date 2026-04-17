# seedfaker

Node.js binding for [seedfaker](https://github.com/opendsr-std/seedfaker) — deterministic synthetic data with 200+ fields, 68 locales, same seed = same output.

[CLI](https://github.com/opendsr-std/seedfaker) · **Node.js** · [Python](https://pypi.org/project/seedfaker/) · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

```bash
npm install @opendsr/seedfaker
```

> **Pre-1.0 notice:** The API may change between minor versions until 1.0.0 is released. Pin your version and check [CHANGELOG.md](https://github.com/opendsr-std/seedfaker/blob/main/CHANGELOG.md) before upgrading.

## ESM

```js
import { SeedFaker } from "@opendsr/seedfaker";

const f = new SeedFaker({ seed: "ci", locale: "en" });

f.field("name");                              // "Zoe Kumar"
f.field("phone", { e164: true });              // "+14155551234"
f.field("email", { omit: 30 });               // "" for 30% of calls

// Weighted locales: 70% English, 20% German, 10% French
const mixed = new SeedFaker({ seed: "ci", locale: "en=7,de=2,fr=1" });

// Single correlated record
f.record(["name", "email", "phone"], { ctx: "strict" });
// → { name: "Zoe Kumar", email: "zoe.kumar@...", phone: "+1..." }

// Batch
f.records(["name", "email", "phone"], { n: 5, ctx: "strict" });

// Validate without generating
f.validate(["name", "email:e164"]);  // throws if invalid

// Corruption
f.records(["name", "email", "ssn"], { n: 100, corrupt: "high" });

// Determinism check
const a = new SeedFaker({ seed: "test" });
const b = new SeedFaker({ seed: "test" });
assert.strictEqual(a.field("name"), b.field("name"));

// Fingerprint — detect algorithm changes after upgrade
SeedFaker.fingerprint(); // "sf0-..."

// All field names
SeedFaker.fields();
```

## CommonJS

```js
const { SeedFaker } = require("@opendsr/seedfaker");

const f = new SeedFaker({ seed: "ci", locale: "en" });
f.field("name");
```

## CLI

**npm** (global install):

```bash
npm install -g @opendsr/seedfaker-cli
```

**npx** (no install):

```bash
npx @opendsr/seedfaker-cli name email phone -n 10 --seed demo
```

**Homebrew**:

```bash
brew install opendsr-std/tap/seedfaker
```

**Cargo**:

```bash
cargo install seedfaker
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
