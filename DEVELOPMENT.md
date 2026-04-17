# Development

## Repository structure

```
seedfaker/
├── rust/
│   ├── Cargo.toml            # workspace (core, cli, pyo3, napi, ffi)
│   ├── core/                 # seedfaker-core (library crate)
│   │   └── src/
│   │       ├── lib.rs         # Faker struct, public API
│   │       ├── field.rs       # field registry, modifiers, transforms
│   │       ├── corrupt.rs     # corruption engine
│   │       ├── gen/           # field generators
│   │       ├── locale/        # locale data (68 locales)
│   │       ├── rng.rs         # deterministic PRNG
│   │       ├── script.rs      # Script, Ctx, Corrupt enums
│   │       ├── ctx.rs         # Identity struct for --ctx strict
│   │       └── tz.rs          # timezone offset parsing
│   ├── cli/                  # seedfaker (binary crate)
│   │   ├── src/
│   │   │   ├── main.rs
│   │   │   ├── cli.rs         # clap CLI definitions
│   │   │   ├── engine.rs      # record generation engine
│   │   │   ├── format.rs      # output formatting + corruption application
│   │   │   ├── writers.rs     # TSV/CSV/JSON/SQL writers
│   │   │   ├── config.rs      # YAML config loader
│   │   │   ├── mcp.rs         # MCP server
│   │   │   └── aggr.rs        # sum/count aggregators
│   │   ├── presets/           # 13 YAML presets
│   │   └── tests/             # 14 test suites, 322+ tests
│   ├── pyo3/                 # Python native extension (PyO3)
│   ├── napi/                 # Node.js native extension (NAPI-RS)
│   └── ffi/                  # C-ABI shared library (PHP, Ruby, Go, etc.)
├── include/
│   └── seedfaker.h           # C header for FFI consumers
├── packages/
│   ├── npm/                  # @opendsr/seedfaker
│   ├── pip/                  # seedfaker
│   ├── php/                  # opendsr/seedfaker (Composer, FFI)
│   ├── ruby/                 # seedfaker (gem, Fiddle FFI)
│   └── go/                   # seedfaker-go (cgo)
├── examples/                 # shell, Python, Node.js, PHP, Ruby, Go
├── benchmarks/               # performance comparison suite
├── tools/                    # gen-fields.py, gen-bindings.py, verify-docs.sh
├── docs/                     # documentation
├── .github/workflows/        # CI pipelines
├── Makefile                  # Docker-first workflow
├── Dockerfile.dev            # dev image
├── docker-compose.yml        # dev service
├── README.md
└── CONTRIBUTING.md
```

## Workflow

All execution inside Docker by default. `LOCAL=1` bypasses Docker.

```bash
make dev              # build CLI + NAPI + PyO3 + FFI
make test             # rust + npm + pip + MCP + all examples + cross-determinism
make pre-commit       # dev + fmt + lint + test + codegen + verify
make pre-release      # pre-commit + regen + sizes
make system-install   # install CLI + pip + npm on the host
LOCAL=1 make dev      # run without Docker
```

`make pre-commit` catches:

- formatting and lint violations
- test failures (npm/pip native, MCP, PHP/Ruby/Go FFI, shell examples)
- stale generated files (`field-reference.md`, bindings, fingerprint)
- DEVELOPMENT.md referencing deleted files

### Docker

```bash
make docker-build     # build dev image
make docker-shell     # shell inside dev container
make docker-clean     # remove containers and volumes
```

### Individual targets

```bash
make build              # CLI + NAPI + PyO3 + FFI
make test-rust          # 267 cargo tests
make test-npm           # NAPI native module verification
make test-pip           # PyO3 native module verification
make test-mcp           # MCP JSON-RPC protocol (5 operations)
make test-examples      # all examples: shell + python + node + php + ruby + go
make test-cross         # CLI = npm = pip for same seed
make fmt                # cargo fmt + prettier + black + taplo
make lint               # clippy + eslint + ruff + taplo + shellcheck
make verify             # check generated files match binary
make regen              # regenerate all: field-reference + bindings + snapshots + fingerprint
```

## Linting

Lint rules in `rust/Cargo.toml` at workspace level:

- `clippy::all` + `clippy::pedantic` at **deny** level
- `unwrap_used`, `expect_used`, `panic` — **deny**
- `unused_imports`, `dead_code` — **deny**
- `unsafe_code` — **forbid** (except `rust/ffi/` which requires `unsafe` for C-ABI)

Formatting: `rust/rustfmt.toml` (Rust), prettier (JS), black (Python), taplo (TOML), shellcheck (shell).

## Benchmarks

```bash
make bench              # quick: CLI tier throughput + per-field (results/fast.md, results/fields.md)
make bench-fast         # CLI tier only (min of 5 runs, 150K records)
make bench-fields       # per-field perf (200K records per field via CLI)
make bench-full         # all of the above + competitor comparisons (requires ./benchmarks/install.sh)
make nick-uniq          # quick identity uniqueness spot-check (3 fields × 5 seeds)
make uniqueness         # full collision analysis across all fields (results/uniqueness.md)
```

CI regression gate runs `bench-fast` on every push to main. Thresholds in `.github/workflows/bench.yml`.

Results committed to `benchmarks/results/`. See `benchmarks/README.md` for methodology.

## Adding a field

1. Add generator in `core/src/gen/<module>.rs`
2. Add entry in `core/fields.yaml`
3. `make field-gen` → `make pre-commit`

## Adding a corruption type

1. Add function in `core/src/corrupt.rs`, dispatch in `apply_one_corruption`
2. Assign to severity tier (light 0–4, medium 5–9, heavy 10–14)
3. Update `docs/corruption.md`
4. `make pre-commit`
