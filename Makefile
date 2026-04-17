SF        := rust/target/release/seedfaker
MANIFEST  := rust/Cargo.toml
RUN       := docker compose run --rm dev


# Resolve cargo: use PATH, then ~/.cargo/bin
CARGO := $(shell command -v cargo 2>/dev/null || echo "$$HOME/.cargo/bin/cargo")

define check_docker
	@docker info > /dev/null 2>&1 || { echo "Error: Docker is not running. Start Docker Desktop or use LOCAL=1 make $(1)"; exit 1; }
endef

# macOS needs dynamic_lookup for PyO3 extension modules
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
  PYO3_RUSTFLAGS := RUSTFLAGS="-C link-arg=-undefined -C link-arg=dynamic_lookup"
endif

# ═══════════════════════════════════════════════════════════════════
# Workflow — all execution inside Docker by default
# ═══════════════════════════════════════════════════════════════════
#
#   make dev            build all + install to host
#   make test           full test suite (rust + npm + pip + mcp + cross)
#   make pre-commit     dev + fmt + lint + test + codegen + verify
#   make pre-release    pre-commit + regen + examples
#
#   LOCAL=1 make dev    run without Docker (requires local toolchain)
#   make setup-local    check/install local toolchain

dev:
ifdef LOCAL
	$(MAKE) _dev
	@bash tools/system-install.sh
else
	$(call check_docker,dev)
	$(RUN) make _dev
endif

test:
ifdef LOCAL
	$(MAKE) _test
else
	$(call check_docker,test)
	$(RUN) make _test
endif

pre-commit:
ifdef LOCAL
	$(MAKE) _pre-commit
else
	$(call check_docker,pre-commit)
	$(RUN) make _pre-commit
endif

pre-release:
ifdef LOCAL
	$(MAKE) _pre-release
else
	$(call check_docker,pre-release)
	$(RUN) make _pre-release
endif

# ═══════════════════════════════════════════════════════════════════
# Internal targets (executed inside Docker or locally with LOCAL=1)
# ═══════════════════════════════════════════════════════════════════

_dev: build

_test: build test-rust test-npm test-pip test-mcp test-examples test-cross

_pre-commit: _dev fmt lint _test audit fields bindings types verify

_pre-release: _pre-commit regen field-examples size

# ═══════════════════════════════════════════════════════════════════
# Install
# ═══════════════════════════════════════════════════════════════════

system-install:
	@bash tools/system-install.sh

# ═══════════════════════════════════════════════════════════════════
# Docker
# ═══════════════════════════════════════════════════════════════════

docker-build:
	docker compose build

docker-clean:
	-docker compose down -v 2>/dev/null
	-docker rmi seedfaker-dev 2>/dev/null

docker-shell:
	$(RUN) bash

docker-image:
	docker build -t seedfaker/cli .

# ═══════════════════════════════════════════════════════════════════
# Build
# ═══════════════════════════════════════════════════════════════════

build-cli:
	$(CARGO) build --release --manifest-path $(MANIFEST) -p seedfaker

build-napi: build-cli
	$(CARGO) build --release --manifest-path $(MANIFEST) -p seedfaker-napi
	@rm -f packages/npm/seedfaker_napi.node
	@cp rust/target/release/libseedfaker_napi.dylib packages/npm/seedfaker_napi.node 2>/dev/null || \
	 cp rust/target/release/libseedfaker_napi.so packages/npm/seedfaker_napi.node 2>/dev/null || \
	 { echo "ERROR: libseedfaker_napi not found in target/release"; exit 1; }
	@file packages/npm/seedfaker_napi.node | grep -q "$(shell uname -m)" || \
	 { echo "WARNING: .node file architecture mismatch"; file packages/npm/seedfaker_napi.node; }

build-pyo3: build-cli
	$(PYO3_RUSTFLAGS) $(CARGO) build --release --manifest-path $(MANIFEST) -p seedfaker-python
	@rm -f packages/pip/seedfaker/_seedfaker.abi3.so
	@cp rust/target/release/lib_seedfaker.dylib packages/pip/seedfaker/_seedfaker.abi3.so 2>/dev/null || \
	 cp rust/target/release/lib_seedfaker.so packages/pip/seedfaker/_seedfaker.abi3.so 2>/dev/null || \
	 { echo "ERROR: lib_seedfaker not found in target/release"; exit 1; }
	@file packages/pip/seedfaker/_seedfaker.abi3.so | grep -q "$(shell uname -m)" || \
	 { echo "WARNING: .so file architecture mismatch"; file packages/pip/seedfaker/_seedfaker.abi3.so; }

build-ffi: build-cli
	$(CARGO) build --release --manifest-path $(MANIFEST) -p seedfaker-ffi

WASM_PACK := $(shell command -v wasm-pack 2>/dev/null || echo "$$HOME/.cargo/bin/wasm-pack")

build-wasm:
	@test -f $(WASM_PACK) || { echo "wasm-pack not found. Install: cargo install wasm-pack"; exit 1; }
	PATH="$$HOME/.cargo/bin:$$PATH" $(WASM_PACK) build rust/wasm --target web --out-dir ../../packages/wasm/web --out-name seedfaker_wasm
	PATH="$$HOME/.cargo/bin:$$PATH" $(WASM_PACK) build rust/wasm --target bundler --out-dir ../../packages/wasm/bundler --out-name seedfaker_wasm

build: build-cli build-napi build-pyo3 build-ffi
	@$(SF) --version

# ═══════════════════════════════════════════════════════════════════
# Quality
# ═══════════════════════════════════════════════════════════════════

test-rust:
	$(PYO3_RUSTFLAGS) $(CARGO) test --manifest-path $(MANIFEST) --workspace

test-npm:
	@bash tools/test-npm.sh

test-pip:
	@bash tools/test-pip.sh

test-mcp:
	@bash tools/test-mcp.sh $(SF)

test-examples:
	@bash tools/test-examples.sh

test-cross:
	@bash tools/test-cross.sh

test-release:
	@bash tools/test-release.sh $(P)

PY_FILES := packages/pip/seedfaker/ tools/*.py benchmarks/python/*.py examples/python/*.py
JS_FILES := packages/npm/index.js packages/npm/index.d.ts packages/npm-cli/bin/seedfaker.js
TOML_FILES := rust/Cargo.toml rust/*/Cargo.toml packages/pip/pyproject.toml
SH_FILES := benchmarks/*.sh .github/scripts/*.sh tools/*.sh examples/*.sh

fmt:
	$(CARGO) fmt --manifest-path $(MANIFEST) --all
	prettier --write $(JS_FILES) '**/*.json' '**/*.yml' '**/*.yaml' '*.md' 'docs/**/*.md' --log-level warn
	ruff format $(PY_FILES)
	ruff check --fix --exit-zero $(PY_FILES)
	taplo fmt $(TOML_FILES)
	@gofmt -w examples/go/main.go packages/go/seedfaker.go 2>/dev/null || true

lint:
	$(PYO3_RUSTFLAGS) $(CARGO) clippy --manifest-path $(MANIFEST) --workspace -- -D warnings
	eslint packages/npm/index.js
	ruff check $(PY_FILES)
	prettier --check $(JS_FILES) '**/*.json' '**/*.yml' '**/*.yaml' '*.md' 'docs/**/*.md' --log-level warn
	taplo lint $(TOML_FILES)
	shellcheck -S error $(SH_FILES)

verify: verify-types
	@bash tools/verify-docs.sh

# ═══════════════════════════════════════════════════════════════════
# Codegen
# ═══════════════════════════════════════════════════════════════════

field-gen:
	python3 tools/gen-fields.py

fields:
	@bash tools/gen-field-reference.sh

bindings:
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@$(SF) --list-json | python3 tools/gen-bindings.py

types: types-ts types-py types-go types-php
	@echo "All types generated."

types-ts:
	@python3 tools/gen-ts-types.py
	@cp build/types/npm/index.d.ts packages/npm/index.d.ts
	@pnpm exec prettier --write packages/npm/index.d.ts --log-level warn

types-py:
	@python3 tools/gen-py-types.py
	@ruff format build/types/pip/__init__.pyi --quiet
	@cp build/types/pip/__init__.pyi packages/pip/seedfaker/__init__.pyi

types-go:
	@python3 tools/gen-go-types.py
	@cp build/types/go/opts_gen.go packages/go/opts_gen.go

types-php:
	@python3 tools/gen-php-types.py

verify-types:
	@python3 tools/gen-ts-types.py
	@pnpm exec prettier --stdin-filepath x.d.ts < build/types/npm/index.d.ts > build/types/npm/.verify-a.tmp 2>/dev/null
	@pnpm exec prettier --stdin-filepath x.d.ts < packages/npm/index.d.ts > build/types/npm/.verify-b.tmp 2>/dev/null
	@diff -q build/types/npm/.verify-a.tmp build/types/npm/.verify-b.tmp || { echo "STALE: packages/npm/index.d.ts — run 'make types'"; exit 1; }
	@python3 tools/gen-py-types.py
	@ruff format build/types/pip/__init__.pyi --quiet
	@diff -q build/types/pip/__init__.pyi packages/pip/seedfaker/__init__.pyi || { echo "STALE: packages/pip/seedfaker/__init__.pyi — run 'make types'"; exit 1; }
	@python3 tools/gen-go-types.py
	@diff -q build/types/go/opts_gen.go packages/go/opts_gen.go || { echo "STALE: packages/go/opts_gen.go — run 'make types'"; exit 1; }
	@echo "Type files up to date."

regen: fields bindings types update-snapshots stamp-fingerprint

update-snapshots:
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@bash tools/update-snapshots.sh

stamp-fingerprint:
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@bash tools/stamp-fingerprint.sh

field-examples:
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@SEEDFAKER="$(CURDIR)/$(SF)" bash examples/context/generate.sh
	@for spec in app-json auth chaos email llm-prompt medical nginx payment pii-leak postgres stacktrace syslog user-table; do \
	  $(SF) run "$$spec" -n 5 --seed spec-example --until 2025 > "examples/presets/$$spec.txt"; \
	done
	@echo "examples/context/ and examples/presets/ updated."

# ═══════════════════════════════════════════════════════════════════
# Bench
# ═══════════════════════════════════════════════════════════════════

bench: bench-fast bench-fields

bench-fast:
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@bash benchmarks/fast.sh

bench-fields: build-cli
	@bash benchmarks/fields.sh

bench-full: build-cli
	@SEEDFAKER="$(CURDIR)/$(SF)" BENCH_RUNS="$(or $(BENCH_RUNS),5)" \
	 BENCH_SKIP_PYTHON="$(BENCH_SKIP_PYTHON)" BENCH_SKIP_NODE="$(BENCH_SKIP_NODE)" \
	 bash benchmarks/full.sh

uniqueness: build-cli
	@MAX=$(or $(MAX),1000000) bash benchmarks/uniqueness.sh

determinism: build-cli
	@bash benchmarks/determinism.sh

audit:
	@bash tools/audit.sh

# ═══════════════════════════════════════════════════════════════════
# Examples
# ═══════════════════════════════════════════════════════════════════

examples: field-examples
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@SEEDFAKER="$(CURDIR)/$(SF)" bash examples/run-all.sh --no-docker

examples-docker:
	@test -f $(SF) || { echo "Run 'make build' first."; exit 1; }
	@SEEDFAKER="$(CURDIR)/$(SF)" bash examples/run-all.sh

# ═══════════════════════════════════════════════════════════════════
# Release
# ═══════════════════════════════════════════════════════════════════

bump:
	@test -n "$(VERSION)" || { echo "Usage: make bump VERSION=0.1.0-alpha.2"; exit 1; }
	@bash tools/bump-version.sh $(VERSION)

release:
	@test -n "$(VERSION)" || { echo "Usage: make release VERSION=0.1.0"; exit 1; }
	git tag -a "v$(VERSION)" -m "Release v$(VERSION)"
	git push origin "v$(VERSION)"

size:
	@echo "CLI binary:"
	@test -f $(SF) && ls -lh $(SF) | awk '{print "  " $$5 "  " $$NF}' || echo "  not built"
	@echo "NAPI module:"
	@test -f packages/npm/seedfaker_napi.node && ls -lh packages/npm/seedfaker_napi.node | awk '{print "  " $$5}' || echo "  not built"
	@echo "PyO3 module:"
	@test -f packages/pip/seedfaker/_seedfaker.abi3.so && ls -lh packages/pip/seedfaker/_seedfaker.abi3.so | awk '{print "  " $$5}' || echo "  not built"

clean:
	-$(CARGO) clean --manifest-path $(MANIFEST) 2>/dev/null
	-rm -f packages/npm/seedfaker_napi.node packages/pip/seedfaker/_seedfaker.abi3.so packages/pip/seedfaker/_seedfaker.so

# ═══════════════════════════════════════════════════════════════════

help:
	@echo "Workflow (all run in Docker by default):"
	@echo "  dev               Build all + install to host"
	@echo "  test              Rust + npm + pip + MCP + cross-determinism"
	@echo "  pre-commit        dev + fmt + lint + test + codegen + verify"
	@echo "  pre-release       pre-commit + regen + examples"
	@echo ""
	@echo "  LOCAL=1 make dev  Run without Docker (local toolchain)"
	@echo ""
	@echo "Install:"
	@echo "  system-install    Install pre-built artifacts to host"
	@echo ""
	@echo "Post-release:"
	@echo "  test-release P=\"npm-cli pip\"     Verify published packages (brew cargo npm-lib npm-cli wasm pip php ruby go)"
	@echo ""
	@echo "Docker:"
	@echo "  docker-build      Build dev image"
	@echo "  docker-shell      Open shell in dev container"
	@echo "  docker-image      Build production image"
	@echo "  docker-clean      Remove dev containers and volumes"
	@echo ""
	@echo "Build:"
	@echo "  build             CLI + NAPI + PyO3 + FFI"
	@echo "  build-wasm        WASM (requires wasm-pack)"
	@echo ""
	@echo "Quality:"
	@echo "  fmt               Format all"
	@echo "  lint              Lint all"
	@echo "  verify            Check generated files"
	@echo "  audit             cargo deny + pnpm audit"
	@echo ""
	@echo "Codegen:"
	@echo "  regen             fields + bindings + types + snapshots + fingerprint"
	@echo ""
	@echo "Release:"
	@echo "  bump VERSION=X    Bump version across all packages"
	@echo "  release VERSION=X Tag + push (triggers release workflow)"
	@echo ""
	@echo "Bench:"
	@echo "  bench             Quick: CLI tier + per-field"
	@echo "  bench-full        All + competitor comparisons"

.PHONY: dev test pre-commit pre-release \
        _dev _test _pre-commit _pre-release \
        system-install build-wasm docker-build docker-clean docker-shell docker-image \
        build-cli build-napi build-pyo3 build-ffi build \
        test-rust test-npm test-pip test-mcp test-examples test-cross test-release fmt lint verify \
        field-gen fields bindings regen update-snapshots stamp-fingerprint field-examples \
        bench bench-fast bench-fields bench-full uniqueness determinism audit \
        examples examples-docker bump release size clean help
