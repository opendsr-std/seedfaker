# seedfaker-core

Core engine for [seedfaker](https://github.com/opendsr-std/seedfaker).

For the CLI, install [`seedfaker`](https://crates.io/crates/seedfaker). For language bindings, see the [library docs](https://github.com/opendsr-std/seedfaker/blob/main/docs/library.md).

[CLI](https://crates.io/crates/seedfaker) · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · [Python](https://pypi.org/project/seedfaker/) · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · [Go](https://github.com/opendsr-std/seedfaker-go) · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

> **Pre-1.0 notice:** The API may change between minor versions until 1.0.0 is released. Pin your version and check [CHANGELOG.md](https://github.com/opendsr-std/seedfaker/blob/main/CHANGELOG.md) before upgrading.

## Highlights

- 200+ fields, 17 groups, 68 locales with native scripts
- Context mode, corruption simulation (15 types, 4 levels)
- Multi-table FK support (Fk/FkDeref column types)
- Dependencies: `getrandom`, `itoa`

## Usage

```rust
use seedfaker_core::{hash_seed, field, pipeline, locale, rng::Rng};

let seed = hash_seed("demo");
let locales = locale::resolve(&[]).unwrap();
let f = field::lookup("email").unwrap();
let dh = pipeline::field_domain_hash(seed, f, "");

let mut ctx = seedfaker_core::ctx::GenContext {
    rng: Rng::derive_fast(dh, 0),
    locales: &locales,
    modifier: "",
    identity: None,
    tz_offset_minutes: 0,
    since: seedfaker_core::temporal::DEFAULT_SINCE,
    until: seedfaker_core::temporal::default_until(),
    range: None,
    ordering: field::Ordering::None,
    numeric: None,
};
let mut buf = String::new();
f.generate(&mut ctx, &mut buf);
// buf contains a deterministic email
```

## Documentation

- [Quick start](https://github.com/opendsr-std/seedfaker/blob/main/docs/quick-start.md)
- [Field reference (200+ fields)](https://github.com/opendsr-std/seedfaker/blob/main/docs/field-reference.md)
- [CLI reference](https://github.com/opendsr-std/seedfaker/blob/main/docs/cli.md)
- [Library API](https://github.com/opendsr-std/seedfaker/blob/main/docs/library.md) (Python, Node.js, Go, PHP, Ruby)
- [MCP server](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)
- [Guides](https://github.com/opendsr-std/seedfaker/blob/main/guides/) — library usage, seed databases, mock APIs, anonymize data, NER training

## License

MIT
