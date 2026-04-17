/// Expression arithmetic: type system, date/timestamp math, cross-interface determinism.
mod common;
use common::{run_fail, run_ok, tempfile};

use std::sync::atomic::{AtomicU32, Ordering};
static COUNTER: AtomicU32 = AtomicU32::new(0);

fn config(yaml: &str) -> (std::path::PathBuf, String) {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = tempfile(&format!("expr{id}"));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("e.yaml");
    std::fs::write(&path, yaml).expect("write");
    let out = run_ok(&["run", path.to_str().expect("p")]);
    (dir, out)
}

fn config_fail(yaml: &str) {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = tempfile(&format!("exf{id}"));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("e.yaml");
    std::fs::write(&path, yaml).expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// int ↔ int
// ---------------------------------------------------------------------------

#[test]
fn int_add_int() {
    let (dir, out) = config("columns:\n  a: integer:100..200\n  b: integer:1..10\n  c: a + b\noptions:\n  seed: t1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("i64")).collect();
        assert_eq!(v[2], v[0] + v[1], "int+int: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn int_sub_int() {
    let (dir, out) = config("columns:\n  a: integer:100..200\n  b: integer:1..10\n  c: a - b\noptions:\n  seed: t2\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("i64")).collect();
        assert_eq!(v[2], v[0] - v[1], "int-int: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn int_mul_int() {
    let (dir, out) = config("columns:\n  a: integer:10..50\n  b: integer:2..5\n  c: a * b\noptions:\n  seed: t3\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("i64")).collect();
        assert_eq!(v[2], v[0] * v[1], "int*int: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// float ↔ float, int ↔ float
// ---------------------------------------------------------------------------

#[test]
fn float_add_float() {
    let (dir, out) = config("columns:\n  a: float:10..100\n  b: float:1..10\n  c: a + b\noptions:\n  seed: f1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<f64> =
            line.split(',').map(|s| s.trim_matches('"').parse().expect("f64")).collect();
        assert!((v[2] - (v[0] + v[1])).abs() < 0.5, "float+float: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn int_mul_float() {
    let (dir, out) = config("columns:\n  a: integer:10..50\n  b: float:1..5\n  c: a * b\noptions:\n  seed: if1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        assert!((v[2] - (v[0] * v[1])).abs() < 0.5, "int*float: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// money ↔ money, money ↔ int/float
// ---------------------------------------------------------------------------

#[test]
fn money_add_money() {
    let (dir, out) = config("columns:\n  a: amount:100..500:plain\n  b: amount:10..50:plain\n  c: a + b\noptions:\n  seed: m1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        assert!((v[2] - (v[0] + v[1])).abs() < 0.01, "money+money: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn money_sub_money() {
    let (dir, out) = config("columns:\n  a: amount:100..500:plain\n  b: amount:10..50:plain\n  c: a - b\noptions:\n  seed: m2\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        assert!((v[2] - (v[0] - v[1])).abs() < 0.01, "money-money: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn money_mul_int() {
    let (dir, out) = config("columns:\n  price: amount:10..100:plain\n  qty: integer:2..5\n  total: price * qty\noptions:\n  seed: mi1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        assert!((v[2] - (v[0] * v[1])).abs() < 0.01, "money*int: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn money_add_int() {
    let (dir, out) = config("columns:\n  price: amount:100..200:plain\n  fee: integer:5..20\n  total: price + fee\noptions:\n  seed: mai\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        assert!((v[2] - (v[0] + v[1])).abs() < 0.01, "money+int: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// date + int (days)
// ---------------------------------------------------------------------------

#[test]
fn date_add_days() {
    let (dir, out) = config("columns:\n  hire: date:2020..2024\n  days: integer:30..365\n  term: hire + days\noptions:\n  seed: d1\n  until: \"2025\"\n  format: csv\n  count: 10\n");
    for line in out.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts[0].len(), 10, "hire YYYY-MM-DD: {}", parts[0]);
        assert_eq!(parts[2].len(), 10, "term YYYY-MM-DD: {}", parts[2]);
        assert!(parts[2] > parts[0], "term after hire: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn date_sub_days() {
    let (dir, out) = config("columns:\n  end: date:2024..2025\n  days: integer:30..365\n  start: end - days\noptions:\n  seed: d2\n  until: \"2025\"\n  format: csv\n  count: 10\n");
    for line in out.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        assert!(parts[2] < parts[0], "start before end: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// timestamp + int (seconds)
// ---------------------------------------------------------------------------

#[test]
fn timestamp_add_seconds() {
    let (dir, out) = config("columns:\n  req: timestamp:2024..2025\n  delay: integer:60..3600\n  resp: req + delay\noptions:\n  seed: ts1\n  until: \"2025\"\n  format: csv\n  count: 10\n");
    for line in out.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        assert!(parts[0].ends_with('Z'), "req ISO: {}", parts[0]);
        assert!(parts[2].ends_with('Z'), "resp ISO: {}", parts[2]);
        assert!(parts[2] > parts[0], "resp after req: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Chaining: expr → expr → aggr
// ---------------------------------------------------------------------------

#[test]
fn chain_expr_to_aggr() {
    let (dir, out) = config("columns:\n  price: amount:10..100:plain\n  qty: integer:1..5\n  subtotal: price * qty\n  running: subtotal:sum\noptions:\n  seed: ch1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    let mut sum = 0.0_f64;
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        let expected_sub = v[0] * v[1];
        assert!((v[2] - expected_sub).abs() < 0.01, "subtotal: {line}");
        sum += v[2];
        assert!((v[3] - sum).abs() < 0.01, "running sum: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn chain_three_exprs() {
    let (dir, out) = config("columns:\n  a: integer:10..100\n  b: integer:1..10\n  c: a + b\n  d: c * integer:2..3\noptions:\n  seed: ch2\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("i64")).collect();
        assert_eq!(v[2], v[0] + v[1], "c=a+b: {line}");
        // d = c * (2 or 3), so d >= c*2 and d <= c*3
        assert!(v[3] >= v[2] * 2 && v[3] <= v[2] * 3, "d=c*2..3: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Free declaration order
// ---------------------------------------------------------------------------

#[test]
fn free_order_total_first() {
    let (dir, out) = config("columns:\n  total: a + b\n  a: integer:100..200\n  b: integer:1..10\noptions:\n  seed: fo1\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    let header = out.lines().next().expect("header");
    assert_eq!(header, "total,a,b");
    for line in out.lines().skip(1) {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("i64")).collect();
        assert_eq!(v[0], v[1] + v[2], "total=a+b: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Determinism: same seed → same output
// ---------------------------------------------------------------------------

#[test]
fn deterministic_across_runs() {
    let yaml = "columns:\n  a: integer:10..100\n  b: integer:1..10\n  c: a + b\n  d: a * b\noptions:\n  seed: det\n  until: \"2025\"\n  format: csv\n  count: 10\n";
    let (dir1, out1) = config(yaml);
    let (dir2, out2) = config(yaml);
    assert_eq!(out1, out2, "same seed must produce identical output");
    let _ = std::fs::remove_dir_all(&dir1);
    let _ = std::fs::remove_dir_all(&dir2);
}

// ---------------------------------------------------------------------------
// CLI expressions
// ---------------------------------------------------------------------------

#[test]
fn cli_expr_add() {
    let out = run_ok(&[
        "a=integer:10..100",
        "b=integer:1..10",
        "c=a+b",
        "-n",
        "5",
        "--seed",
        "cli1",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    for line in out.lines().skip(1) {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("i64")).collect();
        assert_eq!(v[2], v[0] + v[1], "cli a+b: {line}");
    }
}

#[test]
fn cli_expr_mul() {
    let out = run_ok(&[
        "p=amount:10..100:plain",
        "q=integer:2..5",
        "t=p*q",
        "-n",
        "5",
        "--seed",
        "cli2",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    for line in out.lines().skip(1) {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        assert!((v[2] - (v[0] * v[1])).abs() < 0.01, "cli p*q: {line}");
    }
}

// ---------------------------------------------------------------------------
// Column ref with modifier (col:modifier)
// ---------------------------------------------------------------------------

#[test]
fn ref_money_usd() {
    let (dir, out) = config("columns:\n  price: amount:10..500:plain\n  qty: integer:2..5\n  total: price * qty\n  total_usd: total:usd\noptions:\n  seed: rusd\n  until: \"2025\"\n  format: tsv\n  count: 3\n");
    for line in out.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        assert!(parts[3].starts_with('$'), "total_usd should start with $: {}", parts[3]);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ref_date_us() {
    let (dir, out) = config("columns:\n  hire: date:2020..2024\n  days: integer:30..365\n  term: hire + days\n  term_us: term:us\noptions:\n  seed: rdus\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        // US format: MM/DD/YYYY
        assert!(parts[3].contains('/'), "term_us should be MM/DD/YYYY: {}", parts[3]);
        assert_eq!(parts[3].len(), 10, "term_us length: {}", parts[3]);
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ref_timestamp_unix() {
    let (dir, out) = config("columns:\n  ts: timestamp:2024..2025\n  ts_unix: ts:unix\noptions:\n  seed: rtsu\n  until: \"2025\"\n  format: csv\n  count: 3\n");
    for line in out.lines().skip(1) {
        let parts: Vec<&str> = line.split(',').collect();
        let epoch: i64 = parts[1].trim_matches('"').parse().expect("epoch should be integer");
        assert!(epoch > 1_700_000_000 && epoch < 1_800_000_000, "epoch range: {epoch}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ref_plain_copy() {
    let (dir, out) = config("columns:\n  a: integer:100..200\n  b: a\noptions:\n  seed: rcpy\n  until: \"2025\"\n  format: csv\n  count: 5\n");
    for line in out.lines().skip(1) {
        let v: Vec<&str> = line.split(',').collect();
        assert_eq!(v[0], v[1], "ref without modifier should copy: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Type errors — must be rejected at compile time
// ---------------------------------------------------------------------------

#[test]
fn reject_money_mul_money() {
    config_fail("columns:\n  a: amount:10..100\n  b: amount:10..100\n  c: a * b\n");
}

#[test]
fn reject_date_mul_int() {
    config_fail("columns:\n  d: date:2020..2025\n  c: d * integer:2..5\n");
}

#[test]
fn reject_date_add_money() {
    config_fail("columns:\n  d: date:2020..2025\n  a: amount:10..100\n  c: d + a\n");
}

#[test]
fn reject_date_add_float() {
    config_fail("columns:\n  d: date:2020..2025\n  c: d + float:1..10\n");
}

#[test]
fn reject_timestamp_mul() {
    config_fail("columns:\n  t: timestamp:2024..2025\n  c: t * integer:2..5\n");
}

#[test]
fn reject_text_arithmetic() {
    config_fail("columns:\n  a: email\n  b: a + integer:1..10\n");
}

#[test]
fn reject_circular_dependency() {
    config_fail("columns:\n  a: b + integer:1..10\n  b: a + integer:1..10\n");
}

// ---------------------------------------------------------------------------
// Cross-interface determinism: CLI = config for same expressions
// ---------------------------------------------------------------------------

#[test]
fn cli_matches_config() {
    let cli_out = run_ok(&[
        "a=integer:100..200",
        "b=integer:1..10",
        "c=a+b",
        "-n",
        "5",
        "--seed",
        "cross",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);

    let (dir, cfg_out) = config("columns:\n  a: integer:100..200\n  b: integer:1..10\n  c: a + b\noptions:\n  seed: cross\n  until: \"2025\"\n  format: csv\n  count: 5\n");

    assert_eq!(cli_out, cfg_out, "CLI and config must produce identical output");
    let _ = std::fs::remove_dir_all(&dir);
}
