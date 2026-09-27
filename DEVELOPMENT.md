# Development

## Repository structure

```
seedfaker/
├── rust/
│   ├── core/                 # seedfaker-core: generators, field registry (fields.yaml), locales, RNG
│   ├── cli/                  # seedfaker binary, MCP server, presets, integration tests
│   ├── pyo3/                 # Python extension (PyO3)
│   ├── napi/                 # Node.js extension (NAPI-RS)
│   ├── wasm/                 # browser build (wasm-bindgen)
│   └── ffi/                  # C-ABI library for PHP, Ruby, Go
├── include/                  # C header for FFI consumers
├── packages/                 # npm, npm-cli, wasm, pip, php, ruby, go
├── examples/                 # shell, Python, Node.js, PHP, Ruby, Go
├── benchmarks/               # performance comparison suite
├── tools/                    # codegen, verification, release scripts
└── docs/
```

## Workflow

All execution inside Docker by default. `LOCAL=1` bypasses Docker.

```bash
make dev              # build CLI + NAPI + PyO3 + FFI
make test             # rust + npm + pip + MCP + all examples + cross-determinism
make pre-commit       # dev + codegen + fmt + lint + test + audit + verify
make pre-release      # pre-commit + regen + field examples + sizes
make system-install   # install CLI + pip + npm on the host
LOCAL=1 make dev      # run without Docker
```

### Local setup (without Docker)

```bash
nvm use                     # Node version from .nvmrc
make setup-local            # check toolchains, create .venv from requirements-dev.txt
source .venv/bin/activate   # Python tools (ruff, pyyaml) from the venv
LOCAL=1 make test
```

`make pre-commit` catches:

- formatting and lint violations
- test failures (npm/pip native, MCP, PHP/Ruby/Go FFI, shell examples)
- stale generated files (`field-reference.md`, bindings, fingerprint)

### Docker

```bash
make docker-build     # build dev image
make docker-shell     # shell inside dev container
make docker-clean     # remove containers and volumes
```

### Individual targets

```bash
make build              # CLI + NAPI + PyO3 + FFI
make test-rust          # cargo tests
make test-npm           # NAPI native module verification
make test-pip           # PyO3 native module verification
make test-mcp           # MCP JSON-RPC protocol
make test-examples      # all examples: shell + python + node + php + ruby + go
make test-cross         # CLI = npm = pip for same seed
make fmt                # cargo fmt + prettier + ruff + taplo + gofmt
make lint               # clippy + eslint + ruff + prettier + taplo + shellcheck
make verify             # check generated files match binary
make regen              # field-reference + bindings + types + snapshots + fingerprint
```

## Linting

Lint rules in `rust/Cargo.toml` at workspace level:

- `clippy::all` + `clippy::pedantic` at **deny** level
- `unwrap_used`, `expect_used`, `panic` — **deny**
- `unused_imports`, `dead_code` — **deny**
- `unsafe_code` — **deny**, allowed in `rust/ffi/` and `rust/napi/`

Formatting: `rust/rustfmt.toml` (Rust), prettier (JS), ruff (Python), taplo (TOML), shellcheck (shell).

## Benchmarks

```bash
make bench              # quick: CLI tier throughput + per-field (results/fast.md, results/fields.md)
make bench-fast         # CLI tier throughput
make bench-fields       # per-field throughput
make bench-full         # all of the above + competitor comparisons (requires ./benchmarks/install.sh)
make uniqueness         # full collision analysis across all fields (results/uniqueness.md)
```

`.github/workflows/bench.yml` runs `bench-fast` as a regression gate on manual dispatch. Thresholds are in that file.

Results committed to `benchmarks/results/`. See `benchmarks/README.md` for methodology.

## Adding a field

1. Add generator in `rust/core/src/gen/<module>.rs`
2. Add entry in `rust/core/fields.yaml`
3. `make field-gen` → `make pre-commit`

## Adding a corruption type

1. Add function in `rust/core/src/corrupt.rs` and register it in `CORRUPTIONS`
2. Assign to severity tier (light 0–4, medium 5–9, heavy 10–14)
3. Update `docs/corruption.md`
4. `make pre-commit`
