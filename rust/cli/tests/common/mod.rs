// Test helper module — shared across integration tests.
// Each test binary compiles this independently, importing only a subset of functions.
// dead_code is unavoidable for shared test helpers in Rust integration tests.
#![allow(dead_code)]

use std::process::Command;

pub fn seedfaker() -> Command {
    Command::new(env!("CARGO_BIN_EXE_seedfaker"))
}

pub fn run_ok(args: &[&str]) -> String {
    let out = seedfaker().args(args).output().expect("failed to execute");
    assert!(
        out.status.success(),
        "args {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("invalid utf8")
}

pub fn run_fail(args: &[&str]) {
    let out = seedfaker().args(args).output().expect("failed to execute");
    assert!(!out.status.success(), "args {:?} should have failed", args);
}

pub fn parse_jsonl(s: &str) -> Vec<serde_json::Value> {
    s.lines().map(|l| serde_json::from_str(l).expect("invalid JSON line")).collect()
}

pub fn tempfile(prefix: &str) -> std::path::PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!("seedfaker-test-{}-{}", prefix, std::process::id()));
    p
}
