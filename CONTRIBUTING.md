# Contributing

## Requirements

Docker (default) or local toolchain (`LOCAL=1`): Rust 1.78+, Node 20+, Python 3.8+, PHP 7.4+, Ruby 2.7+, Go 1.21+.

## Workflow

```bash
make dev              # build all (Docker by default)
make test             # full suite: rust + npm + pip + MCP + examples + cross-determinism
make pre-commit       # the gate: dev + fmt + lint + test + codegen + verify
LOCAL=1 make dev      # without Docker
```

## Code standards

- `unsafe` is forbidden (except `rust/ffi/` for C-ABI)
- `unwrap`, `expect`, `panic` are denied
- clippy::all + clippy::pedantic at deny
- All public fields and modifiers covered by determinism tests
- Every field in the registry appears in `--list` and `docs/field-reference.md`

## Testing

`make test` runs:

- **test-rust** — 267 cargo tests (determinism, corruption, CLI, fields, formats, MCP, config, cross-determinism, dates, ranges, weights)
- **test-npm** — NAPI native loaded, fields, fingerprint, records with ctx=strict
- **test-pip** — PyO3 native loaded, fields, fingerprint, records with ctx=strict
- **test-mcp** — initialize, tools/list, generate, fingerprint, run_preset
- **test-examples** — 12 shell + python + node + php + ruby + go examples
- **test-cross** — CLI = npm = pip for same seed

## Pull requests

- `make pre-commit` before submitting
- Include tests for new fields or modifiers
- One feature or fix per PR
