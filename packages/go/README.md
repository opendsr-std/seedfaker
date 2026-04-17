# seedfaker-go

Go binding for [seedfaker](https://github.com/opendsr-std/seedfaker) — deterministic synthetic data with 200+ fields, 68 locales, same seed = same output.

[CLI](https://github.com/opendsr-std/seedfaker) · [Node.js](https://www.npmjs.com/package/@opendsr/seedfaker) · [Python](https://pypi.org/project/seedfaker/) · [Browser/WASM](https://www.npmjs.com/package/@opendsr/seedfaker-wasm) · **Go** · [PHP](https://packagist.org/packages/opendsr/seedfaker) · [Ruby](https://rubygems.org/gems/seedfaker) · [MCP](https://github.com/opendsr-std/seedfaker/blob/main/docs/mcp.md)

## Requirements

- Go >= 1.22
- `libseedfaker_ffi` shared library
- CGO enabled

## Install

```bash
go get github.com/opendsr-std/seedfaker-go
```

Pre-built `libseedfaker_ffi` binaries are included in `bin/` for supported platforms.

> **Pre-1.0 notice:** The API may change between minor versions until 1.0.0 is released. Pin your version and check [CHANGELOG.md](https://github.com/opendsr-std/seedfaker/blob/main/CHANGELOG.md) before upgrading.

For development from source:

```bash
make build-ffi
cd examples/go
CGO_LDFLAGS="-L../../rust/target/release -lseedfaker_ffi" go run main.go
```

## Usage

```go
package main

import (
    "fmt"
    seedfaker "github.com/opendsr-std/seedfaker-go"
)

func main() {
    f, _ := seedfaker.New(seedfaker.Options{Seed: "ci", Locale: "en"})
    defer f.Close()

    // Single values
    name, _ := f.Field("name")
    phone, _ := f.Field("phone")
    fmt.Println(name, phone)

    // Single correlated record
    rec, _ := f.Record([]string{"name", "email", "phone"}, "strict", "")
    fmt.Println(rec) // map[name:Zoe Kumar email:zoe.kumar@... phone:+1...]

    // Batch
    records, _ := f.Records(seedfaker.RecordOpts{
        Fields: []string{"name", "email"},
        N:      5,
    })
    fmt.Println(records)

    // Validate without generating
    _ = f.Validate([]string{"name", "email:e164"}, "", "")

    // Fingerprint and field list
    fp, _ := seedfaker.Fingerprint()
    fields, _ := seedfaker.Fields()
    fmt.Println(fp, len(fields))
}
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
