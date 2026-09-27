# seedfaker

## Commands

- `LOCAL=1 make pre-commit` is the gate before a commit. It formats and regenerates files in place.
- `LOCAL=1 make test`
- `cargo test --manifest-path rust/Cargo.toml -p seedfaker --test determinism` runs one suite.

Without `LOCAL=1`, `make pre-commit` and `make test` run in Docker.

## Rules

- Same seed and version produce byte-identical output across the CLI and all bindings.
- An intended change to seeded output needs `make update-snapshots` and `make stamp-fingerprint`.
- Add a `CHANGELOG.md` entry for every user-visible change.
- Fix clippy findings in code. A new `#[allow]` needs a comment naming the reason.
- Write code, docs, and commit messages in American English.

## Generated files

Never edit these by hand. `rust/core/src/field_gen.rs` comes from `rust/core/fields.yaml` through `make field-gen`. `make regen` regenerates the rest:

| File                                                                                                                                              | Command                  |
| ------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| `packages/npm/index.d.ts`, `packages/pip/seedfaker/__init__.pyi`, `packages/go/opts_gen.go`, `field()` params in `packages/php/src/SeedFaker.php` | `make types`             |
| `@generated-start`/`@generated-end` regions in `packages/npm/index.js` and `packages/pip/seedfaker/__init__.py`                                   | `make bindings`          |
| `docs/field-reference.md`                                                                                                                         | `make fields`            |
| `FINGERPRINT` and fingerprint values in READMEs and docs                                                                                          | `make stamp-fingerprint` |

## Related repositories

The release workflow writes `seedfaker-go`, `seedfaker-php`, and `homebrew-tap` from this repository. Change the source here.
