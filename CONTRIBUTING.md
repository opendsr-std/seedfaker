# Contributing

## Requirements

Docker (default) or local toolchain (`LOCAL=1`): Rust 1.88+, Node 22+, Python 3.9+, PHP 8.1+, Ruby 2.7+, Go 1.21+.

## Workflow

```bash
make dev              # build all (Docker by default)
make test             # full suite: rust + npm + pip + MCP + examples + cross-determinism
make pre-commit       # the gate: dev + fmt + lint + test + audit + codegen + verify
LOCAL=1 make dev      # without Docker
```

## Code standards

- `unsafe` is denied outside `rust/ffi/` and `rust/napi/`
- `unwrap`, `expect`, `panic` are denied
- clippy::all + clippy::pedantic at deny
- All public fields and modifiers covered by determinism tests
- Every field in the registry appears in `--list` and `docs/field-reference.md`

## Pull requests

- `make pre-commit` before submitting
- Include tests for new fields or modifiers
- One feature or fix per PR
