#!/usr/bin/env bash
# Post-release verification: install from real registries in Docker, run checks.
#
# Each test runs in a fresh Docker container. brew is the only exception —
# Homebrew requires macOS and cannot run in Docker.
# Docker must be available; the script exits immediately if it is not.
#
# Packages / bindings:
#   brew      homebrew tap   (macOS native)
#   cargo     cargo install  (Rust CLI)
#   npm-lib   @opendsr/seedfaker
#   npm-cli   @opendsr/seedfaker-cli
#   wasm      @opendsr/seedfaker-wasm
#   pip       seedfaker      (Python)
#   php       opendsr/seedfaker
#   ruby      seedfaker      (Ruby gem)
#   go        github.com/opendsr-std/seedfaker-go
#
# Usage:
#   bash tools/test-release.sh                              # all
#   bash tools/test-release.sh npm-cli pip                  # selective
#   VERSION=0.1.0-alpha.29 bash tools/test-release.sh       # pin version
#
# Makefile:
#   make test-release                                        # all
#   make test-release P="npm-cli pip"                        # selective
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="${VERSION:-}"
PASSED=0
FAILED=0
FAILURES=()

ALL_PKGS=(brew cargo npm-lib npm-cli wasm pip php ruby go)

if [ $# -gt 0 ]; then
  PKGS=("$@")
else
  PKGS=("${ALL_PKGS[@]}")
fi

enabled() {
  for p in "${PKGS[@]}"; do [ "$p" = "$1" ] && return 0; done
  return 1
}

pass()   { echo "  ok: $1"; PASSED=$((PASSED + 1)); }
header() {
  echo ""
  echo "=== $1 ==="
  [ -n "$2" ] && echo "    $2"
}

run_or_fail() {
  local label="$1"; shift
  if "$@"; then
    pass "$label"
    return 0
  else
    echo "  --- FAIL: $label ---"
    FAILED=$((FAILED + 1))
    FAILURES+=("$label")
    return 1
  fi
}

# Write CONTENT to a temp file, run it in IMAGE, clean up.
# Usage: docker_run LABEL IMAGE CONTENT
docker_run() {
  local label="$1"
  local image="$2"
  local content="$3"
  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  printf '%s\n' "$content" > "$tmp"
  run_or_fail "$label" docker run --rm -v "$tmp":/test.sh:ro "$image" sh /test.sh
  rm -f "$tmp"
}

resolve_version() {
  if [ -n "$VERSION" ]; then
    echo "$VERSION"
  else
    grep '^version' "$ROOT/rust/cli/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/'
  fi
}

# ---------------------------------------------------------------------------
# brew
# ---------------------------------------------------------------------------
test_brew() {
  header "brew" "homebrew tap: opendsr-std/tap/seedfaker  (macOS native)"
  if ! command -v brew >/dev/null 2>&1; then
    echo "  SKIP: brew not installed (https://brew.sh)"
    return
  fi
  local ver
  ver=$(resolve_version)
  brew tap opendsr-std/tap
  brew install seedfaker
  run_or_fail "brew: --version"     seedfaker --version
  run_or_fail "brew: --fingerprint" seedfaker --fingerprint
  run_or_fail "brew: generate"      seedfaker name email phone --seed release-test -n 3 --until 2025
  brew uninstall seedfaker 2>/dev/null || true
  brew untap opendsr-std/tap 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# cargo
# ---------------------------------------------------------------------------
test_cargo() {
  local ver
  ver=$(resolve_version)
  header "cargo" "crate: seedfaker@${ver}  (rust:alpine)"
  docker_run "cargo: install + generate" "rust:alpine" "
set -e
cargo install seedfaker --version ${ver} --root /tmp/sf
/tmp/sf/bin/seedfaker --version
/tmp/sf/bin/seedfaker --fingerprint
/tmp/sf/bin/seedfaker name email phone --seed release-test -n 3 --until 2025
"
}

# ---------------------------------------------------------------------------
# npm-lib
# ---------------------------------------------------------------------------
test_npm_lib() {
  local pkg="@opendsr/seedfaker"
  [ -n "$VERSION" ] && pkg="${pkg}@${VERSION}" || pkg="${pkg}@next"
  header "npm-lib" "package: ${pkg}  (node:22)"

  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  cat > "$tmp" << 'JSEOF'
const { SeedFaker } = require("@opendsr/seedfaker");
const ver = require("@opendsr/seedfaker/package.json").version;
console.log("  version:     " + ver);
console.log("  fingerprint: " + SeedFaker.fingerprint());
const f = new SeedFaker({ seed: "release-test", locale: "en" });
const v = f.field("name");
if (!v) throw new Error("field() returned empty");
const r = f.records(["name", "email"], { n: 3 });
if (r.length !== 3) throw new Error("records() returned " + r.length);
console.log("  name:        " + v);
JSEOF
  run_or_fail "npm-lib: install + api" \
    docker run --rm -v "$tmp":/check.js:ro node:22 sh -c "
set -e
mkdir /work && cd /work
npm init -y >/dev/null
npm install ${pkg}
cp /check.js check.js && node check.js
"
  rm -f "$tmp"
}

# ---------------------------------------------------------------------------
# npm-cli
# ---------------------------------------------------------------------------
test_npm_cli() {
  local pkg="@opendsr/seedfaker-cli"
  [ -n "$VERSION" ] && pkg="${pkg}@${VERSION}" || pkg="${pkg}@next"
  header "npm-cli" "package: ${pkg}  (node:22)"

  docker_run "npm-cli: install + generate" "node:22" "
set -e
mkdir /work && cd /work
npm init -y >/dev/null
npm install ${pkg}
./node_modules/.bin/seedfaker --version
./node_modules/.bin/seedfaker --fingerprint
./node_modules/.bin/seedfaker name email phone --seed release-test -n 3 --until 2025
"

  docker_run "npm-cli: npx" "node:22" "
set -e
npx --yes ${pkg} --version
npx --yes ${pkg} --fingerprint
npx --yes ${pkg} name email phone --seed release-test -n 3 --until 2025
"

  docker_run "npm-cli: global install" "node:22" "
set -e
npm install -g ${pkg} >/dev/null
seedfaker --version
seedfaker --fingerprint
seedfaker name email phone --seed release-test -n 3 --until 2025
"
}

# ---------------------------------------------------------------------------
# wasm
# ---------------------------------------------------------------------------
test_wasm() {
  local pkg="@opendsr/seedfaker-wasm"
  [ -n "$VERSION" ] && pkg="${pkg}@${VERSION}" || pkg="${pkg}@next"
  header "wasm" "package: ${pkg}  (node:22-alpine)"

  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  cat > "$tmp" << 'MJSEOF'
import { createRequire } from "module";
const require = createRequire(import.meta.url);
const { version } = require("@opendsr/seedfaker-wasm/package.json");
const { SeedFaker } = await import("@opendsr/seedfaker-wasm");
await SeedFaker.init();
console.log("  version:     " + version);
console.log("  fingerprint: " + SeedFaker.fingerprint());
const f = new SeedFaker({ seed: "release-test", locale: "en" });
const v = f.field("name");
if (!v) throw new Error("field() returned empty");
const r = f.records(["name", "email"], { n: 3 });
if (r.length !== 3) throw new Error("records() returned " + r.length);
console.log("  name:        " + v);
MJSEOF
  run_or_fail "wasm: install + api" \
    docker run --rm -v "$tmp":/check.mjs:ro node:22-alpine sh -c "
set -e
mkdir /work && cd /work
echo '{\"type\":\"module\"}' > package.json
npm install ${pkg}
cp /check.mjs check.mjs && node --experimental-wasm-modules check.mjs
"
  rm -f "$tmp"
}

# ---------------------------------------------------------------------------
# pip
# ---------------------------------------------------------------------------
test_pip() {
  local pkg="seedfaker"
  [ -n "$VERSION" ] && pkg="seedfaker==${VERSION}"
  header "pip" "package: ${pkg}  (python:3.12)"

  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  cat > "$tmp" << 'PYEOF'
import seedfaker
from seedfaker import SeedFaker
print("  version:     " + seedfaker.__version__)
print("  fingerprint: " + SeedFaker.fingerprint())
f = SeedFaker(seed="release-test", locale="en")
v = f.field("name")
assert v, "field() returned empty"
r = f.records(["name", "email"], n=3)
assert len(r) == 3, "records() returned " + str(len(r))
print("  name:        " + v)
PYEOF
  run_or_fail "pip: install + api" \
    docker run --rm -v "$tmp":/check.py:ro python:3.12 sh -c "
set -e
pip install ${pkg} -q
python3 /check.py
"
  rm -f "$tmp"
}

# ---------------------------------------------------------------------------
# php
# ---------------------------------------------------------------------------
test_php() {
  local pkg="opendsr/seedfaker"
  [ -n "$VERSION" ] && pkg="opendsr/seedfaker:${VERSION}"
  header "php" "package: ${pkg}  (php:8.4-cli)"

  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  cat > "$tmp" << 'PHPEOF'
<?php
require_once '/work/vendor/autoload.php';
use Seedfaker\SeedFaker;
echo "  fingerprint: " . SeedFaker::fingerprint() . "\n";
$f = new SeedFaker(seed: "release-test", locale: "en");
$v = $f->field("name");
if (!$v) { fwrite(STDERR, "field() returned empty\n"); exit(1); }
$r = $f->records(["name", "email"], n: 3);
if (count($r) !== 3) { fwrite(STDERR, "records() returned " . count($r) . "\n"); exit(1); }
echo "  name:        " . $v . "\n";
PHPEOF
  run_or_fail "php: install + api" \
    docker run --rm -v "$tmp":/check.php:ro php:8.4-cli bash -c "
set -e
apt-get update -q >/dev/null 2>&1
apt-get install -y -q --no-install-recommends libffi-dev pkg-config curl unzip >/dev/null 2>&1
docker-php-ext-install ffi >/dev/null 2>&1
curl -sS https://getcomposer.org/installer | php -- --install-dir=/usr/local/bin --filename=composer >/dev/null 2>&1
mkdir /work && cd /work
composer init --no-interaction -q 2>/dev/null
composer config minimum-stability alpha
composer config preferred-install dist
composer require ${pkg} -q 2>/dev/null
php -d ffi.enable=1 /check.php
"
  rm -f "$tmp"
}

# ---------------------------------------------------------------------------
# ruby
# ---------------------------------------------------------------------------
test_ruby() {
  local gem_args gem_ver
  if [ -n "$VERSION" ]; then
    # RubyGems uses 0.1.0.pre.alpha.N format; our VERSION uses 0.1.0-alpha.N
    gem_ver=$(echo "$VERSION" | sed 's/-/.pre./')
    gem_args="seedfaker --version ${gem_ver} --no-document"
  else
    gem_args="seedfaker --pre --no-document"
  fi
  header "ruby" "gem: seedfaker${VERSION:+ ${VERSION}}  (ruby:3.3-alpine)"

  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  cat > "$tmp" << 'RUBYEOF'
require "seedfaker"
puts "  fingerprint: " + Seedfaker::SeedFaker.fingerprint
f = Seedfaker::SeedFaker.new(seed: "release-test", locale: "en")
v = f.field("name")
raise "field() returned empty" if v.nil? || v.empty?
r = f.records(%w[name email], n: 3)
raise "records() returned " + r.size.to_s unless r.size == 3
puts "  name:        " + v
RUBYEOF
  run_or_fail "ruby: install + api" \
    docker run --rm -v "$tmp":/check.rb:ro ruby:3.3-alpine sh -c "
set -e
gem install ${gem_args}
ruby /check.rb
"
  rm -f "$tmp"
}

# ---------------------------------------------------------------------------
# go
# ---------------------------------------------------------------------------
test_go() {
  local pkg="github.com/opendsr-std/seedfaker-go"
  [ -n "$VERSION" ] && pkg="${pkg}@v${VERSION}"
  header "go" "module: ${pkg}  (golang:1.22)"

  local tmp
  tmp=$(mktemp /tmp/sf-release-XXXXXX)
  cat > "$tmp" << 'GOEOF'
package main

import (
	"fmt"
	seedfaker "github.com/opendsr-std/seedfaker-go"
)

func main() {
	fp, err := seedfaker.Fingerprint()
	if err != nil { panic(err) }
	fmt.Println("  fingerprint:", fp)
	f, err := seedfaker.New(seedfaker.Options{Seed: "release-test", Locale: "en"})
	if err != nil { panic(err) }
	defer f.Close()
	vals, err := f.Field("name")
	if err != nil { panic(err) }
	if len(vals) == 0 || vals[0] == "" { panic("field() returned empty") }
	r, err := f.Records(seedfaker.RecordOpts{Fields: []string{"name", "email"}, N: 3})
	if err != nil { panic(err) }
	if len(r) != 3 { panic(fmt.Sprintf("records() returned %d", len(r))) }
	fmt.Println("  name:        " + vals[0])
}
GOEOF
  run_or_fail "go: install + api" \
    docker run --rm -v "$tmp":/src/main.go:ro golang:1.22 sh -c "
set -e
mkdir /work && cd /work
go mod init test-seedfaker >/dev/null
cp /src/main.go main.go
go get ${pkg}
go run main.go
"
  rm -f "$tmp"
}

# ---------------------------------------------------------------------------
# Run
# ---------------------------------------------------------------------------
if ! command -v docker >/dev/null 2>&1; then
  echo "FATAL: docker not found — required for test-release"
  exit 1
fi

echo "seedfaker release verification"
[ -n "$VERSION" ] && echo "version: $VERSION"
echo "packages: ${PKGS[*]}"

enabled brew     && test_brew    || true
enabled cargo    && test_cargo   || true
enabled npm-lib  && test_npm_lib || true
enabled npm-cli  && test_npm_cli || true
enabled wasm     && test_wasm    || true
enabled pip      && test_pip     || true
enabled php      && test_php     || true
enabled ruby     && test_ruby    || true
enabled go       && test_go      || true

echo ""
echo "--- results ---"
echo "passed: $PASSED  failed: $FAILED"
if [ $FAILED -gt 0 ]; then
  echo "failures:"
  for f in "${FAILURES[@]}"; do echo "  - $f"; done
  exit 1
fi
