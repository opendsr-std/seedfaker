#!/usr/bin/env node

const { execFileSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const PLATFORMS = {
  "darwin-arm64": "@opendsr/seedfaker-darwin-arm64",
  "darwin-x64": "@opendsr/seedfaker-darwin-x64",
  "linux-x64": "@opendsr/seedfaker-linux-x64",
  "linux-arm64": "@opendsr/seedfaker-linux-arm64",
  "win32-x64": "@opendsr/seedfaker-win32-x64",
};

const BIN_NAME = process.platform === "win32" ? "seedfaker.exe" : "seedfaker";

function findBinary() {
  const key = `${process.platform}-${process.arch}`;
  const pkg = PLATFORMS[key];

  if (!pkg) {
    console.error(`seedfaker: unsupported platform ${key}`);
    console.error(`supported: ${Object.keys(PLATFORMS).join(", ")}`);
    process.exit(1);
  }

  try {
    const pkgDir = path.dirname(require.resolve(`${pkg}/package.json`));
    const nativeBin = path.join(pkgDir, "bin", BIN_NAME);
    const muslBin = path.join(pkgDir, "bin", `${BIN_NAME}-musl`);
    // On Linux prefer static musl binary — works on any glibc version and Alpine
    if (process.platform === "linux" && fs.existsSync(muslBin)) return muslBin;
    if (fs.existsSync(nativeBin)) return nativeBin;
  } catch (_) {}

  const { execSync } = require("child_process");
  try {
    const which = process.platform === "win32" ? "where" : "which";
    return execSync(`${which} seedfaker`, { encoding: "utf8" }).trim();
  } catch (_) {}

  console.error(
    "seedfaker: binary not found.\n\n" +
      "Install the platform package or the Rust CLI:\n" +
      "  cargo install seedfaker\n",
  );
  process.exit(1);
}

const bin = findBinary();
try {
  fs.chmodSync(bin, 0o755);
} catch (_) {}
try {
  execFileSync(bin, process.argv.slice(2), { stdio: "inherit" });
} catch (e) {
  if (e.status != null) process.exit(e.status);
  throw e;
}
