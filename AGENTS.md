# seedfaker

Deterministic synthetic data generator. Rust core and CLI, with bindings for Node.js (NAPI), Python (PyO3), browser (WASM), and a C-ABI library consumed by PHP, Ruby, and Go.

## Layout

- `rust/core/` — generators, field registry (`fields.yaml`), locales, RNG
- `rust/cli/` — `seedfaker` binary, MCP server, presets, and integration tests in `rust/cli/tests/`
- `rust/{pyo3,napi,wasm,ffi}/` — language bindings. `unsafe` is allowed only in `rust/ffi/`.
- `packages/` — publishable wrappers (npm, pip, php, ruby, go, wasm)
- `tools/` — codegen, verification, and release scripts called from the Makefile

## Commands

Local setup, once: `make setup-local`. Then in each shell: `source .venv/bin/activate && nvm use`.

- `LOCAL=1 make test` — build, then Rust tests, npm/pip/MCP checks, examples in 6 languages, cross-language determinism
- `LOCAL=1 make pre-commit` — the full gate. It runs `fmt`, which rewrites files.
- `make build` — release build of CLI, NAPI, PyO3, FFI
- `make lint` — clippy, eslint, ruff, prettier, taplo, shellcheck
- `make verify` — fails if generated files are stale
- `make audit` — cargo-deny + pnpm audit
- One Rust test suite: `cargo test --manifest-path rust/Cargo.toml -p seedfaker --test determinism`

`dev`, `test`, `pre-commit`, and `pre-release` run inside Docker unless `LOCAL=1` is set. Other targets always run on the host.

## Rules

- Same seed + same version must produce byte-identical output in every binding. `make test-cross` checks CLI = npm = pip.
- If a change alters generated output, `seedfaker --fingerprint` changes. That is a breaking change for users: run `make update-snapshots` and `make stamp-fingerprint`, and note it in `CHANGELOG.md`.
- Rust lints are deny-level: clippy `all` + `pedantic`, no `unwrap`/`expect`/`panic`. Don't silence them with `#[allow]`.
- `CHANGELOG.md` follows Keep a Changelog. Add an entry for every user-visible change.
- Write code, docs, and commit messages in American English.

## Generated files — never edit by hand

| File                                                                                                                                              | Regenerate with          |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| `rust/core/src/field_gen.rs` (from `rust/core/fields.yaml`)                                                                                       | `make field-gen`         |
| `packages/npm/index.d.ts`, `packages/pip/seedfaker/__init__.pyi`, `packages/go/opts_gen.go`, `field()` params in `packages/php/src/SeedFaker.php` | `make types`             |
| `@generated-start`/`@generated-end` regions in `packages/npm/index.js` and `packages/pip/seedfaker/__init__.py`                                   | `make bindings`          |
| `docs/field-reference.md`                                                                                                                         | `make fields`            |
| `FINGERPRINT` and fingerprint mentions in docs                                                                                                    | `make stamp-fingerprint` |

`make regen` regenerates all of them.

## Related repositories

`seedfaker-go`, `seedfaker-php`, and `homebrew-tap` are written by `.github/workflows/release.yml` on each release (from `packages/go`, `packages/php`, and the release tarballs). Change the source here, never in those repositories.
