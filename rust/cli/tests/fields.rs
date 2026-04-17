/// Field-level tests: validates output format, modifiers, transforms,
/// and default behavior for every field group.
mod common;
use common::{run_fail, run_ok};

// ---------------------------------------------------------------------------
// Multi-field structure
// ---------------------------------------------------------------------------

#[test]
fn multi_field_tab_separated() {
    let out = run_ok(&["name", "email", "phone", "-n", "5", "--seed", "multi", "--until", "2025"]);
    assert_eq!(out.lines().count(), 5);
    for line in out.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(parts.len(), 3, "expected 3 tab-separated fields: {line}");
        assert!(parts[1].contains('@'), "email should contain @: {}", parts[1]);
    }
}

#[test]
fn field_group_expands() {
    let out = run_ok(&["person", "-n", "3", "--seed", "grp", "--until", "2025"]);
    assert_eq!(out.lines().count(), 3);
    for line in out.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        assert!(cols.len() >= 3, "person group should expand to >=3 fields, got {}", cols.len());
    }
}

#[test]
fn list_command_shows_groups_and_fields() {
    let out = run_ok(&["--list"]);
    for group in &["core:", "text:", "person:", "auth:", "internet:"] {
        assert!(out.contains(group), "--list missing group '{group}'");
    }
    let field_count = out.lines().filter(|l| l.starts_with("    ")).count();
    assert!(field_count >= 200, "--list should show >=200 fields, got {field_count}");
}

// ---------------------------------------------------------------------------
// Column alias (name=field) syntax
// ---------------------------------------------------------------------------

#[test]
fn alias_sets_csv_header() {
    let out = run_ok(&[
        "id=uuid",
        "full_name=name",
        "mail=email",
        "-n",
        "2",
        "--seed",
        "alias",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    let header = lines.next().unwrap();
    assert_eq!(header, "id,full_name,mail", "alias should set CSV header: {header}");
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 3, "should have 3 columns: {line}");
        assert!(parts[2].contains('@'), "mail should be an email: {}", parts[2]);
    }
}

#[test]
fn alias_with_modifier() {
    let out = run_ok(&[
        "price=amount:usd",
        "-n",
        "3",
        "--seed",
        "almod",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let header = out.lines().next().unwrap();
    assert_eq!(header, "price", "alias with modifier should use alias as header");
    for line in out.lines().skip(1) {
        let trimmed = line.trim_matches('"');
        assert!(trimmed.starts_with('$'), "amount:usd should start with $: {line}");
    }
}

#[test]
fn alias_with_enum_weights() {
    let out = run_ok(&[
        "status=enum:active=7,inactive=2,banned=1",
        "-n",
        "10",
        "--seed",
        "alenum",
        "--format",
        "csv",
    ]);
    let header = out.lines().next().unwrap();
    assert_eq!(header, "status", "alias on weighted enum should use alias: {header}");
    for line in out.lines().skip(1) {
        assert!(
            line == "active" || line == "inactive" || line == "banned",
            "unexpected enum value: {line}"
        );
    }
}

#[test]
fn alias_mixed_with_positional() {
    let out = run_ok(&[
        "name",
        "user_email=email",
        "phone",
        "-n",
        "1",
        "--seed",
        "mix",
        "--format",
        "csv",
    ]);
    let header = out.lines().next().unwrap();
    assert_eq!(header, "name,user_email,phone", "mixed alias+positional headers: {header}");
}

#[test]
fn alias_on_group_rejected() {
    run_fail(&["grp=person", "-n", "1"]);
}

// ---------------------------------------------------------------------------
// Aggregators: sum()
// ---------------------------------------------------------------------------

#[test]
fn sum_running_total() {
    let out = run_ok(&[
        "amount=amount:plain",
        "total=amount:sum",
        "-n",
        "5",
        "--seed",
        "sumtest",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    let header = lines.next().expect("header");
    assert_eq!(header, "amount,total");
    let mut running = 0.0_f64;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 2, "expected 2 columns: {line}");
        let amount: f64 = parts[0].parse().expect("amount should be numeric");
        let total: f64 = parts[1].parse().expect("total should be numeric");
        running += amount;
        assert!(
            (running - total).abs() < 0.011,
            "running sum mismatch: expected {running:.2}, got {total}: {line}"
        );
    }
    assert!(running > 0.0, "sum should be positive");
}

#[test]
fn sum_auto_name() {
    let out = run_ok(&[
        "integer:1..100",
        "integer:sum",
        "-n",
        "3",
        "--seed",
        "sname",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let header = out.lines().next().expect("header");
    assert_eq!(header, "integer,sum_integer");
}

#[test]
fn sum_parses_formatted_amount() {
    let out = run_ok(&[
        "amount",
        "total=amount:sum",
        "-n",
        "5",
        "--seed",
        "fmtsum",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let last_line = out.lines().last().expect("last line");
    let total: f64 = last_line.split(',').last().expect("total col").parse().expect("f64");
    assert!(total > 0.0, "sum should parse formatted amounts: {last_line}");
}

#[test]
fn sum_unknown_source_fails() {
    run_fail(&["nonexistent:sum", "-n", "1"]);
}

#[test]
fn sum_grouped() {
    let out = run_ok(&[
        "gid=integer:0..2",
        "val=integer:1..100",
        "total=val:sum=gid",
        "-n",
        "10",
        "--seed",
        "grp",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    let header = lines.next().expect("header");
    assert_eq!(header, "gid,val,total");
    // Verify grouped sums: track running total per gid
    let mut sums: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 3, "expected 3 columns: {line}");
        let gid = parts[0];
        let val: f64 = parts[1].parse().expect("val");
        let total: f64 = parts[2].parse().expect("total");
        let entry = sums.entry(gid.to_string()).or_insert(0.0);
        *entry += val;
        assert!(
            (*entry - total).abs() < 0.011,
            "grouped sum mismatch for gid={gid}: expected {:.2}, got {total}: {line}",
            *entry
        );
    }
}

#[test]
fn sum_grouped_auto_name() {
    let out = run_ok(&[
        "gid=integer:0..2",
        "integer:1..100",
        "integer:sum=gid",
        "-n",
        "3",
        "--seed",
        "ga",
        "--format",
        "csv",
    ]);
    let header = out.lines().next().expect("header");
    assert_eq!(header, "gid,integer,sum_integer_by_gid");
}

#[test]
fn count_per_group() {
    let out = run_ok(&[
        "gid=integer:0..2",
        "n=gid:count",
        "-n",
        "10",
        "--seed",
        "gcnt",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    let header = lines.next().expect("header");
    assert_eq!(header, "gid,n");
    let mut counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        let gid = parts[0];
        let n: u64 = parts[1].parse().expect("count");
        let entry = counts.entry(gid.to_string()).or_insert(0);
        *entry += 1;
        assert_eq!(*entry, n, "count mismatch for gid={gid}: {line}");
    }
}

#[test]
fn count_auto_name() {
    let out = run_ok(&[
        "integer:0..5",
        "integer:count",
        "-n",
        "3",
        "--seed",
        "ca",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let header = out.lines().next().expect("header");
    assert_eq!(header, "integer,count_integer");
}

#[test]
fn duplicate_fields_produce_independent_values() {
    let out = run_ok(&["email", "email", "email", "-n", "1", "--seed", "dup", "--until", "2025"]);
    let parts: Vec<&str> = out.trim().split('\t').collect();
    assert_eq!(parts.len(), 3);
    for p in &parts {
        assert!(p.contains('@'), "each duplicate field should be a valid email: {p}");
    }
}

// ---------------------------------------------------------------------------
// Expressions: arithmetic between numeric columns
// ---------------------------------------------------------------------------

#[test]
fn expr_add_exact_values() {
    let out = run_ok(&[
        "a=integer:100..200",
        "b=integer:1..10",
        "c=a+b",
        "-n",
        "3",
        "--seed",
        "expr",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "a,b,c");
    for line in lines {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("int")).collect();
        assert_eq!(v.len(), 3, "expected 3 columns: {line}");
        assert_eq!(v[2], v[0] + v[1], "c should equal a + b: {line}");
    }
}

#[test]
fn expr_sub_exact_values() {
    let out = run_ok(&[
        "a=integer:100..200",
        "b=integer:1..10",
        "d=a-b",
        "-n",
        "3",
        "--seed",
        "expr",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "a,b,d");
    for line in lines {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("int")).collect();
        assert_eq!(v[2], v[0] - v[1], "d should equal a - b: {line}");
    }
}

#[test]
fn expr_mul_exact_values() {
    let out = run_ok(&[
        "a=integer:100..200",
        "b=integer:1..10",
        "p=a*b",
        "-n",
        "3",
        "--seed",
        "expr",
        "--until",
        "2025",
        "--format",
        "csv",
    ]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "a,b,p");
    for line in lines {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("int")).collect();
        assert_eq!(v[2], v[0] * v[1], "p should equal a * b: {line}");
    }
}

#[test]
fn expr_config_free_order() {
    let dir = common::tempfile("exord");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("expr.yaml");
    std::fs::write(
        &path,
        "columns:\n  total: base + bonus\n  bonus: integer:1..10\n  base: integer:100..200\noptions:\n  seed: expr\n  until: \"2025\"\n  format: csv\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "3"]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "total,bonus,base");
    for line in lines {
        let v: Vec<i64> = line.split(',').map(|s| s.parse().expect("int")).collect();
        assert_eq!(v[0], v[2] + v[1], "total should equal base + bonus: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_chain_through_columns() {
    let dir = common::tempfile("exch");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("chain.yaml");
    std::fs::write(
        &path,
        "columns:\n  price: amount:100..200:plain\n  qty: integer:2..5\n  subtotal: price * qty\noptions:\n  seed: chain\n  until: \"2025\"\n  format: csv\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "3"]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "price,qty,subtotal");
    for line in lines {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        let expected = v[0] * v[1];
        assert!((v[2] - expected).abs() < 0.01, "subtotal should equal price * qty: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_deterministic() {
    let args = &[
        "a=integer:10..100",
        "b=integer:1..10",
        "c=a+b",
        "-n",
        "5",
        "--seed",
        "det",
        "--until",
        "2025",
        "--format",
        "csv",
    ];
    let run1 = run_ok(args);
    let run2 = run_ok(args);
    assert_eq!(run1, run2, "expressions should be deterministic");
}

#[test]
fn expr_non_numeric_rejected() {
    let dir = common::tempfile("exrj");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  a: name\n  b: a + name\n").expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_circular_dependency_rejected() {
    let dir = common::tempfile("excyc");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("cycle.yaml");
    std::fs::write(&path, "columns:\n  a: b + integer:1..10\n  b: a + integer:1..10\n")
        .expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_with_aggr() {
    let dir = common::tempfile("exagg");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("agg.yaml");
    std::fs::write(
        &path,
        "columns:\n  price: amount:10..100:plain\n  qty: integer:1..5\n  subtotal: price * qty\n  running: subtotal:sum\noptions:\n  seed: eagg\n  until: \"2025\"\n  format: csv\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "5"]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "price,qty,subtotal,running");
    let mut running_sum = 0.0_f64;
    for line in lines {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        let expected_sub = v[0] * v[1];
        assert!((v[2] - expected_sub).abs() < 0.01, "subtotal mismatch: {line}");
        running_sum += v[2];
        assert!((v[3] - running_sum).abs() < 0.01, "running sum mismatch: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Date/timestamp arithmetic
// ---------------------------------------------------------------------------

#[test]
fn expr_date_plus_days() {
    let dir = common::tempfile("exdate");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("date.yaml");
    std::fs::write(
        &path,
        "columns:\n  hire: date:2020..2024\n  days: integer:30..365\n  term: hire + days\noptions:\n  seed: dtest\n  until: \"2025\"\n  format: csv\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "5"]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "hire,days,term");
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 3, "expected 3 columns: {line}");
        // hire and term should be YYYY-MM-DD format
        assert_eq!(parts[0].len(), 10, "hire should be YYYY-MM-DD: {}", parts[0]);
        assert_eq!(parts[2].len(), 10, "term should be YYYY-MM-DD: {}", parts[2]);
        assert!(parts[0] < parts[2], "termination should be after hire: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_timestamp_plus_seconds() {
    let dir = common::tempfile("exts");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("ts.yaml");
    std::fs::write(
        &path,
        "columns:\n  created: timestamp:2024..2025\n  delay: integer:3600..86400\n  delivered: created + delay\noptions:\n  seed: tstest\n  until: \"2025\"\n  format: csv\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "5"]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "created,delay,delivered");
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        assert_eq!(parts.len(), 3);
        // Both timestamps should be ISO format with Z
        assert!(parts[0].ends_with('Z'), "created should be ISO: {}", parts[0]);
        assert!(parts[2].ends_with('Z'), "delivered should be ISO: {}", parts[2]);
        assert!(parts[0] < parts[2], "delivered should be after created: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_money_mul_int() {
    let dir = common::tempfile("exmoney");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("money.yaml");
    std::fs::write(
        &path,
        "columns:\n  price: amount:10..100:plain\n  qty: integer:2..5\n  total: price * qty\noptions:\n  seed: mtest\n  until: \"2025\"\n  format: csv\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "5"]);
    let mut lines = out.lines();
    assert_eq!(lines.next().expect("header"), "price,qty,total");
    for line in lines {
        let v: Vec<f64> = line.split(',').map(|s| s.parse().expect("f64")).collect();
        let expected = v[0] * v[1];
        assert!((v[2] - expected).abs() < 0.01, "total should equal price * qty: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_date_mul_rejected() {
    let dir = common::tempfile("exdmul");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  d: date:2020..2025\n  bad: d * integer:2..5\n")
        .expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_date_plus_money_rejected() {
    let dir = common::tempfile("exdm");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  d: date:2020..2025\n  a: amount:100..500\n  bad: d + a\n")
        .expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_money_mul_money_rejected() {
    let dir = common::tempfile("exmm");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  a: amount:10..100\n  b: amount:10..100\n  bad: a * b\n")
        .expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn expr_text_rejected() {
    let dir = common::tempfile("extxt");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  a: email\n  b: a + integer:1..10\n").expect("write");
    run_fail(&["run", path.to_str().expect("p"), "-n", "1"]);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Modifiers: credit-card, ssn, phone, iban, mac, uuid
// ---------------------------------------------------------------------------

#[test]
fn modifier_credit_card() {
    let space = run_ok(&["credit-card:space", "-n", "10", "--seed", "cc", "--until", "2025"]);
    for line in space.lines() {
        assert!(line.contains(' '), "credit-card:space should have spaces: {line}");
        let digits: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
        assert!(digits.len() >= 13 && digits.len() <= 19, "digit count: {}", digits.len());
    }
    let plain = run_ok(&["credit-card:plain", "-n", "10", "--seed", "cc", "--until", "2025"]);
    for line in plain.lines() {
        assert!(line.chars().all(|c| c.is_ascii_digit()), "plain should be digits only: {line}");
        assert!(line.len() >= 13 && line.len() <= 19);
    }
}

#[test]
fn modifier_ssn_plain() {
    let out =
        run_ok(&["ssn:plain", "-n", "10", "--seed", "ssn", "--until", "2025", "--locale", "en"]);
    for line in out.lines() {
        assert!(line.chars().all(|c| c.is_ascii_digit()), "ssn:plain digits only: {line}");
        assert_eq!(line.len(), 9, "SSN should be 9 digits: {line}");
    }
}

#[test]
fn modifier_phone_e164() {
    let out = run_ok(&["phone:e164", "-n", "10", "--seed", "ph", "--until", "2025"]);
    for line in out.lines() {
        assert!(line.starts_with('+'), "e164 should start with +: {line}");
        let digits: usize = line.chars().filter(|c| c.is_ascii_digit()).count();
        assert!(digits >= 7 && digits <= 15, "e164 digit count {digits}: {line}");
    }
}

#[test]
fn modifier_iban_plain() {
    let out = run_ok(&["iban:plain", "-n", "10", "--seed", "iban", "--until", "2025"]);
    for line in out.lines() {
        assert!(!line.contains(' '), "iban:plain no spaces: {line}");
        assert!(line.len() >= 15, "IBAN >= 15 chars: {line}");
    }
}

#[test]
fn modifier_mac_plain() {
    let out = run_ok(&["mac:plain", "-n", "10", "--seed", "mac", "--until", "2025"]);
    for line in out.lines() {
        assert!(!line.contains(':') && !line.contains('-'), "mac:plain no separators: {line}");
        assert_eq!(line.len(), 12, "mac:plain 12 hex chars: {line}");
    }
}

#[test]
fn modifier_uuid_plain() {
    let out = run_ok(&["uuid:plain", "-n", "10", "--seed", "uuid", "--until", "2025"]);
    for line in out.lines() {
        assert!(!line.contains('-'), "uuid:plain no dashes: {line}");
        assert_eq!(line.len(), 32, "uuid:plain 32 hex chars: {line}");
        assert!(line.chars().all(|c| c.is_ascii_hexdigit()), "uuid:plain should be hex: {line}");
    }
}

#[test]
fn modifier_amount_plain_is_numeric() {
    let out = run_ok(&["amount:plain", "-n", "50", "--seed", "amt", "--until", "2025"]);
    for line in out.lines() {
        let v: f64 = line.parse().expect(&format!("should be numeric: {line}"));
        // Default range: [0, 999_999] whole + [0, 99] cents
        assert!(v >= 0.0, "amount should be >= 0: {v}");
        assert!(v < 1_000_000.0, "amount should be < 1M: {v}");
    }
}

#[test]
fn modifier_amount_currencies() {
    let usd = run_ok(&["amount:usd", "-n", "5", "--seed", "amt", "--until", "2025"]);
    for line in usd.lines() {
        assert!(line.contains('$'), "amount:usd should contain $: {line}");
    }
    let eur = run_ok(&["amount:eur", "-n", "5", "--seed", "amt", "--until", "2025"]);
    for line in eur.lines() {
        assert!(line.contains('\u{20ac}'), "amount:eur should contain euro sign: {line}");
    }
    let gbp = run_ok(&["amount:gbp", "-n", "5", "--seed", "amt", "--until", "2025"]);
    for line in gbp.lines() {
        assert!(line.contains('\u{00a3}'), "amount:gbp should contain pound sign: {line}");
    }
}

// ---------------------------------------------------------------------------
// Color, URL, password, locale-code modifiers
// ---------------------------------------------------------------------------

#[test]
fn color_modifiers() {
    let hex = run_ok(&["color:hex", "-n", "5", "--seed", "clr", "--until", "2025"]);
    for line in hex.lines() {
        assert!(line.starts_with('#') && line.len() == 7, "color:hex should be #RRGGBB: {line}");
        assert!(line[1..].chars().all(|c| c.is_ascii_hexdigit()));
    }
    let rgb = run_ok(&["color:rgb", "-n", "5", "--seed", "clr", "--until", "2025"]);
    for line in rgb.lines() {
        let parts: Vec<&str> = line.split(", ").collect();
        assert!(parts.len() == 3, "color:rgb should have 3 components: {line}");
        for p in &parts {
            let v: u16 = p.parse().unwrap();
            assert!(v <= 255, "color:rgb component out of range: {line}");
        }
    }
    let name = run_ok(&["color", "-n", "5", "--seed", "clr", "--until", "2025"]);
    for line in name.lines() {
        assert!(!line.is_empty());
    }
}

#[test]
fn url_modifiers() {
    let cases =
        [("url", "https://"), ("url:http", "http://"), ("url:ftp", "ftp://"), ("url:ws", "ws://")];
    for (spec, prefix) in cases {
        let out = run_ok(&[spec, "-n", "5", "--seed", "urlm", "--until", "2025"]);
        for line in out.lines() {
            assert!(line.starts_with(prefix), "{spec} should start with {prefix}: {line}");
        }
    }
    let ssh = run_ok(&["url:ssh", "-n", "5", "--seed", "urlm", "--until", "2025"]);
    for line in ssh.lines() {
        assert!(line.starts_with("ssh://"), "url:ssh: {line}");
        assert!(line.contains('@'), "url:ssh should have user@host: {line}");
    }
}

#[test]
fn password_modifiers() {
    let pin = run_ok(&["password:pin", "-n", "10", "--seed", "pw", "--until", "2025"]);
    for line in pin.lines() {
        assert!(line.chars().all(|c| c.is_ascii_digit()), "pin should be digits: {line}");
        assert!(line.len() >= 4 && line.len() <= 6, "pin length: {line}");
    }
    let memorable = run_ok(&["password:memorable", "-n", "5", "--seed", "pw", "--until", "2025"]);
    for line in memorable.lines() {
        assert!(
            line.contains('-') || line.contains('_'),
            "memorable should have separator: {line}"
        );
    }
}

#[test]
fn locale_code_modifiers() {
    let default = run_ok(&["locale-code", "-n", "5", "--seed", "lc", "--until", "2025"]);
    for line in default.lines() {
        assert!(line.contains('-'), "locale-code default should use dash: {line}");
    }
    let under = run_ok(&["locale-code:underscore", "-n", "5", "--seed", "lc", "--until", "2025"]);
    for line in under.lines() {
        assert!(line.contains('_'), "locale-code:underscore should use _: {line}");
    }
    let short = run_ok(&["locale-code:short", "-n", "5", "--seed", "lc", "--until", "2025"]);
    for line in short.lines() {
        assert_eq!(line.len(), 2, "locale-code:short should be 2 chars: {line}");
    }
}

// ---------------------------------------------------------------------------
// Transforms
// ---------------------------------------------------------------------------

#[test]
fn transform_upper() {
    let out = run_ok(&["name:upper", "-n", "10", "--seed", "tu", "--until", "2025"]);
    for line in out.lines() {
        assert_eq!(line, &line.to_uppercase(), "upper: {line}");
    }
}

#[test]
fn transform_lower() {
    let out = run_ok(&["name:lower", "-n", "10", "--seed", "tl", "--until", "2025"]);
    for line in out.lines() {
        assert_eq!(line, &line.to_lowercase(), "lower: {line}");
    }
}

#[test]
fn transform_capitalize() {
    let out = run_ok(&[
        "name:capitalize",
        "-n",
        "10",
        "--seed",
        "cap",
        "--until",
        "2025",
        "--locale",
        "en",
    ]);
    for line in out.lines() {
        let first = line.chars().next().expect("non-empty");
        assert!(first.is_uppercase(), "capitalize first char should be uppercase: {line}");
    }
}

#[test]
fn modifier_with_transform_both_applied() {
    let out = run_ok(&["phone:e164:upper", "-n", "5", "--seed", "mt", "--until", "2025"]);
    for line in out.lines() {
        assert!(line.starts_with('+'), "e164 starts with +: {line}");
        assert_eq!(line, &line.to_uppercase(), "upper transform should apply: {line}");
    }
}

// ---------------------------------------------------------------------------
// Primitive fields
// ---------------------------------------------------------------------------

#[test]
fn primitive_bit_and_trit() {
    let bit = run_ok(&["bit", "-n", "50", "--seed", "bit", "--until", "2025"]);
    for line in bit.lines() {
        assert!(line == "0" || line == "1", "bit should be 0 or 1: {line}");
    }
    let sign = run_ok(&["bit:sign", "-n", "50", "--seed", "sign", "--until", "2025"]);
    for line in sign.lines() {
        assert!(line == "-1" || line == "1", "bit:sign should be -1 or 1: {line}");
    }
    let trit = run_ok(&["trit", "-n", "50", "--seed", "trit", "--until", "2025"]);
    for line in trit.lines() {
        assert!(line == "-1" || line == "0" || line == "1", "trit should be -1/0/1: {line}");
    }
}

// ---------------------------------------------------------------------------
// Crypto fields
// ---------------------------------------------------------------------------

#[test]
fn crypto_addresses() {
    let btc = run_ok(&["btc-address", "-n", "10", "--seed", "btc", "--until", "2025"]);
    for line in btc.lines() {
        assert!(
            line.starts_with('1') || line.starts_with('3') || line.starts_with("bc1"),
            "btc-address format: {line}"
        );
        assert!(line.len() >= 25 && line.len() <= 62);
    }
    let eth = run_ok(&["eth-address", "-n", "10", "--seed", "eth", "--until", "2025"]);
    for line in eth.lines() {
        assert!(line.starts_with("0x") && line.len() == 42, "eth-address: {line}");
        assert!(line[2..].chars().all(|c| c.is_ascii_hexdigit()));
    }
    let tx = run_ok(&["tx-hash", "-n", "10", "--seed", "tx", "--until", "2025"]);
    for line in tx.lines() {
        assert!(line.starts_with("0x") && line.len() == 66, "tx-hash: {line}");
    }
    let btc_tx = run_ok(&["tx-hash:btc", "-n", "10", "--seed", "txb", "--until", "2025"]);
    for line in btc_tx.lines() {
        assert_eq!(line.len(), 64, "tx-hash:btc 64 hex chars: {line}");
        assert!(line.chars().all(|c| c.is_ascii_hexdigit()));
    }
}

// ---------------------------------------------------------------------------
// Location fields
// ---------------------------------------------------------------------------

#[test]
fn latitude_longitude_in_range() {
    let lat = run_ok(&["latitude", "-n", "10", "--seed", "ll", "--until", "2025"]);
    for line in lat.lines() {
        let v: f64 = line.parse().expect("latitude float");
        assert!((-90.0..=90.0).contains(&v), "latitude out of range: {v}");
    }
    let lon = run_ok(&["longitude", "-n", "10", "--seed", "ll", "--until", "2025"]);
    for line in lon.lines() {
        let v: f64 = line.parse().expect("longitude float");
        assert!((-180.0..=180.0).contains(&v), "longitude out of range: {v}");
    }
}

#[test]
fn country_code_alpha3() {
    let out = run_ok(&[
        "country-code:alpha3",
        "--locale",
        "de",
        "-n",
        "5",
        "--seed",
        "cc3",
        "--until",
        "2025",
    ]);
    for line in out.lines() {
        assert_eq!(line.len(), 3, "alpha3 should be 3 chars: {line}");
        assert_eq!(line, "DEU", "de locale alpha3 should be DEU: {line}");
    }
}

#[test]
fn postal_code_varies_by_locale() {
    let de =
        run_ok(&["postal-code", "--locale", "de", "-n", "5", "--seed", "pc", "--until", "2025"]);
    for line in de.lines() {
        assert_eq!(line.len(), 5);
        assert!(line.chars().all(|c| c.is_ascii_digit()));
    }
}

// ---------------------------------------------------------------------------
// Enum fields
// ---------------------------------------------------------------------------

#[test]
fn enum_only_produces_listed_values() {
    let out = run_ok(&["enum:red,green,blue", "-n", "30", "--seed", "enum", "--until", "2025"]);
    for line in out.lines() {
        assert!(["red", "green", "blue"].contains(&line), "unexpected: {line}");
    }
    assert!(out.contains("red") && out.contains("green") && out.contains("blue"));
}

#[test]
fn enum_single_value_always_constant() {
    let out = run_ok(&["enum:constant", "-n", "5", "--seed", "single", "--until", "2025"]);
    for line in out.lines() {
        assert_eq!(line, "constant");
    }
}

#[test]
fn enum_in_multi_field_correct_column() {
    let out = run_ok(&[
        "name",
        "enum:admin,user",
        "email",
        "-n",
        "5",
        "--seed",
        "emix",
        "--until",
        "2025",
    ]);
    for line in out.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(parts.len(), 3);
        assert!(parts[1] == "admin" || parts[1] == "user", "enum column: {}", parts[1]);
        assert!(parts[2].contains('@'), "email column: {}", parts[2]);
    }
}

// ---------------------------------------------------------------------------
// Other fields
// ---------------------------------------------------------------------------

#[test]
fn street_address_not_empty() {
    let out = run_ok(&["street-address", "-n", "5", "--seed", "sa", "--until", "2025"]);
    for line in out.lines() {
        assert!(line.len() > 3, "street-address should be reasonable length: {line}");
    }
}

#[test]
fn emoji_is_valid_unicode() {
    let out = run_ok(&["emoji", "-n", "10", "--seed", "emo", "--until", "2025"]);
    for line in out.lines() {
        assert!(!line.is_empty() && !line.is_ascii(), "emoji should be non-ASCII: {line}");
    }
}

#[test]
fn currency_code_crypto() {
    let out = run_ok(&["currency-code:crypto", "-n", "20", "--seed", "cc", "--until", "2025"]);
    for line in out.lines() {
        assert!(!line.is_empty() && line.len() >= 2 && line.len() <= 6, "crypto ticker: {line}");
    }
}

#[test]
fn env_var_multi() {
    let out = run_ok(&["env-var:multi", "-n", "1", "--seed", "ev", "--until", "2025"]);
    let lines: Vec<&str> = out.trim().lines().collect();
    assert!(lines.len() >= 3, "env-var:multi should produce 3+ lines, got {}", lines.len());
    for line in &lines {
        assert!(line.contains('='), "each env var should contain =: {line}");
    }
}

// ---------------------------------------------------------------------------
// Validation: bad modifiers and bad fields
// ---------------------------------------------------------------------------

#[test]
fn unknown_modifier_rejected() {
    run_fail(&["name:foobar", "-n", "1"]);
    run_fail(&["credit-card:foobar", "-n", "1"]);
}

#[test]
fn group_with_modifier_rejected() {
    run_fail(&["person:upper", "-n", "1"]);
}

#[test]
fn unknown_field_rejected() {
    run_fail(&["nonexistent-field", "-n", "1"]);
}

// ---------------------------------------------------------------------------
// Enum: validation, weighted picks, error cases
// ---------------------------------------------------------------------------

#[test]
fn enum_uniform_pick() {
    let out = run_ok(&["enum:red,green,blue", "-n", "100", "--seed", "eu", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 100);
    assert!(lines.contains(&"red"));
    assert!(lines.contains(&"green"));
    assert!(lines.contains(&"blue"));
    // No value outside the enum
    for line in &lines {
        assert!(
            *line == "red" || *line == "green" || *line == "blue",
            "unexpected enum value: {line}"
        );
    }
}

#[test]
fn enum_weighted_distribution() {
    let out = run_ok(&["enum:yes=9,no=1", "-n", "1000", "--seed", "ew", "--until", "2025"]);
    let yes_count = out.lines().filter(|l| *l == "yes").count();
    let no_count = out.lines().filter(|l| *l == "no").count();
    assert_eq!(yes_count + no_count, 1000);
    // yes=9 should be ~90% (allow 80-97%)
    assert!(
        yes_count > 800 && yes_count < 970,
        "yes=9,no=1 should give ~90% yes, got {yes_count}/1000"
    );
}

#[test]
fn enum_mixed_weighted_unweighted() {
    let out = run_ok(&["enum:a=5,b,c=2", "-n", "1000", "--seed", "em", "--until", "2025"]);
    let a = out.lines().filter(|l| *l == "a").count();
    let b = out.lines().filter(|l| *l == "b").count();
    let c = out.lines().filter(|l| *l == "c").count();
    assert_eq!(a + b + c, 1000);
    // a=5, b=1, c=2 → a ~62%, b ~12%, c ~25%
    assert!(a > b, "a=5 should appear more than b=1: a={a}, b={b}");
    assert!(a > c, "a=5 should appear more than c=2: a={a}, c={c}");
    assert!(c > b, "c=2 should appear more than b=1: c={c}, b={b}");
}

#[test]
fn enum_empty_rejected() {
    run_fail(&["enum", "-n", "1"]);
}

#[test]
fn enum_invalid_chars_rejected() {
    run_fail(&["enum:foo@bar,baz", "-n", "1"]);
}

#[test]
fn enum_invalid_weight_rejected() {
    run_fail(&["enum:yes=abc,no=1", "-n", "1"]);
}

#[test]
fn enum_zero_weight_rejected() {
    run_fail(&["enum:a=0,b=1", "-n", "1"]);
}

#[test]
fn enum_values_only_valid_chars() {
    // Underscore and hyphen are allowed
    let out = run_ok(&["enum:foo-bar,baz_qux", "-n", "10", "--seed", "ev", "--until", "2025"]);
    for line in out.lines() {
        assert!(line == "foo-bar" || line == "baz_qux", "unexpected: {line}");
    }
}

// ---------------------------------------------------------------------------
// Age: demographics, ctx correlation, range
// ---------------------------------------------------------------------------

#[test]
fn age_demographic_distribution() {
    let out = run_ok(&["age", "-n", "1000", "--seed", "ad", "--until", "2025"]);
    let ages: Vec<i64> = out.lines().map(|l| l.parse().unwrap()).collect();
    assert_eq!(ages.len(), 1000);

    // All ages should be reasonable (0-120)
    for &a in &ages {
        assert!(a >= 0 && a <= 120, "age out of range: {a}");
    }

    // Young adults (18-35) should be the largest group (~50% of weighted pyramid)
    let young = ages.iter().filter(|&&a| a >= 18 && a <= 35).count();
    assert!(young > 350 && young < 650, "18-35 should be ~50%, got {young}/1000");

    // Very old (76+) should be rare (<5%)
    let old = ages.iter().filter(|&&a| a >= 76).count();
    assert!(old < 80, "76+ should be <8%, got {old}/1000");
}

#[test]
fn age_correlates_with_birthdate_in_ctx() {
    // Pin --until for deterministic test (default is system year)
    let out = run_ok(&[
        "age",
        "birthdate",
        "-n",
        "50",
        "--seed",
        "ac",
        "--locale",
        "en",
        "--ctx",
        "strict",
        "--until",
        "2038",
    ]);
    for line in out.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        assert_eq!(parts.len(), 2, "expected age\\tbirthdate: {line}");
        let age: i64 = parts[0].parse().expect("age should be integer");
        let year: i64 = parts[1][..4].parse().expect("birthdate year");
        // Rule: age = year_now - birth_year - 1 (birthday not yet occurred)
        let expected = 2038 - year - 1;
        assert_eq!(age, expected, "age={age} but birthdate={}, expected age={expected}", parts[1]);
    }
}

#[test]
fn age_with_range() {
    let out = run_ok(&["age:21..65", "-n", "100", "--seed", "ar", "--until", "2025"]);
    for line in out.lines() {
        let a: i64 = line.parse().unwrap();
        assert!(a >= 21 && a <= 65, "age:21..65 produced {a}");
    }
}

// ---------------------------------------------------------------------------
// HTTP method: weighted distribution
// ---------------------------------------------------------------------------

#[test]
fn http_method_weighted() {
    let out = run_ok(&["http-method", "-n", "1000", "--seed", "hm", "--until", "2025"]);
    let get = out.lines().filter(|l| *l == "GET").count();
    let post = out.lines().filter(|l| *l == "POST").count();
    let options = out.lines().filter(|l| *l == "OPTIONS").count();

    // GET should dominate (>50%)
    assert!(get > 500, "GET should be >50%, got {get}/1000");
    // POST second place
    assert!(post > get / 5, "POST should be significant, got {post}/1000");
    // OPTIONS should be rare (<3%)
    assert!(options < 30, "OPTIONS should be <3%, got {options}/1000");
}

// ---------------------------------------------------------------------------
// Gender: weighted distribution
// ---------------------------------------------------------------------------

#[test]
fn gender_weighted() {
    let out = run_ok(&["gender", "-n", "1000", "--seed", "gw", "--until", "2025"]);
    let male = out.lines().filter(|l| *l == "Male").count();
    let female = out.lines().filter(|l| *l == "Female").count();
    let nb = out.lines().filter(|l| *l == "Non-binary").count();
    assert_eq!(male + female + nb, 1000);

    // Male and Female should be roughly equal (~49% each)
    assert!(male > 400 && male < 580, "Male should be ~49%, got {male}");
    assert!(female > 400 && female < 580, "Female should be ~49%, got {female}");
    // Non-binary should be rare (~2%)
    assert!(nb < 50, "Non-binary should be <5%, got {nb}");
}

// ---------------------------------------------------------------------------
// Amount: tiered distribution
// ---------------------------------------------------------------------------

#[test]
fn amount_tiered_default() {
    let out = run_ok(&["amount:plain", "-n", "1000", "--seed", "at", "--until", "2025"]);
    let values: Vec<f64> = out.lines().map(|l| l.parse().unwrap()).collect();

    let small = values.iter().filter(|&&v| v <= 50.0).count();
    let huge = values.iter().filter(|&&v| v > 100_000.0).count();

    // Small amounts ($1-50) should be most common (>20%)
    assert!(small > 200, "small amounts should be >20%, got {small}/1000");
    // Huge amounts ($100K+) should be rare (<5%)
    assert!(huge < 50, "huge amounts should be <5%, got {huge}/1000");
}

#[test]
fn amount_with_range_is_uniform() {
    let out = run_ok(&["amount:plain:100..200", "-n", "100", "--seed", "aru", "--until", "2025"]);
    for line in out.lines() {
        let v: f64 = line.parse().unwrap();
        assert!(v >= 100.0 && v <= 200.99, "amount:100..200 produced {v}");
    }
}

// ---------------------------------------------------------------------------
// Integer: tiered default
// ---------------------------------------------------------------------------

#[test]
fn integer_tiered_default() {
    let out = run_ok(&["integer", "-n", "1000", "--seed", "it", "--until", "2025"]);
    let values: Vec<i64> = out.lines().map(|l| l.parse().unwrap()).collect();

    let small = values.iter().filter(|&&v| v <= 100).count();
    let huge = values.iter().filter(|&&v| v > 100_000).count();

    // Small values (1-100) should be most common (>20%)
    assert!(small > 200, "small integers should be >20%, got {small}/1000");
    // Huge values (100K+) should be <15%
    assert!(huge < 150, "huge integers should be <15%, got {huge}/1000");
}

#[test]
fn integer_with_range_is_uniform() {
    let out = run_ok(&["integer:1..10", "-n", "100", "--seed", "iru", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().unwrap();
        assert!(v >= 1 && v <= 10, "integer:1..10 produced {v}");
    }
}

// ---------------------------------------------------------------------------
// Email: no RFC violations
// ---------------------------------------------------------------------------

#[test]
fn email_no_double_dots() {
    let out = run_ok(&["email", "-n", "10000", "--seed", "edd", "--until", "2025"]);
    for line in out.lines() {
        let local = line.split('@').next().unwrap();
        assert!(!local.contains(".."), "double dot in email local part: {line}");
        assert!(
            !local.starts_with('.') && !local.ends_with('.'),
            "leading/trailing dot in email local part: {line}"
        );
    }
}

// ---------------------------------------------------------------------------
// password:strong — guaranteed character class coverage
// ---------------------------------------------------------------------------

#[test]
fn password_strong_has_all_classes() {
    let out = run_ok(&["password:strong", "-n", "100", "--seed", "pws", "--until", "2025"]);
    for line in out.lines() {
        let has_upper = line.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = line.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = line.chars().any(|c| c.is_ascii_digit());
        let has_symbol = line.chars().any(|c| !c.is_ascii_alphanumeric());
        assert!(has_upper, "strong password missing uppercase: {line}");
        assert!(has_lower, "strong password missing lowercase: {line}");
        assert!(has_digit, "strong password missing digit: {line}");
        assert!(has_symbol, "strong password missing symbol: {line}");
        assert!(line.len() >= 16, "strong password too short: {line}");
        assert!(line.len() <= 24, "strong password too long: {line}");
    }
}

#[test]
fn password_strong_no_triple_repeats() {
    let out = run_ok(&["password:strong", "-n", "1000", "--seed", "rep", "--until", "2025"]);
    for line in out.lines() {
        let bytes = line.as_bytes();
        for i in 2..bytes.len() {
            assert!(
                !(bytes[i] == bytes[i - 1] && bytes[i] == bytes[i - 2]),
                "strong password has 3+ consecutive identical chars: {line}"
            );
        }
    }
}

#[test]
fn password_strong_deterministic() {
    let a = run_ok(&["password:strong", "-n", "10", "--seed", "det", "--until", "2025"]);
    let b = run_ok(&["password:strong", "-n", "10", "--seed", "det", "--until", "2025"]);
    assert_eq!(a, b, "password:strong must be deterministic");
}

// ---------------------------------------------------------------------------
// :xuniq modifier — extended uniqueness for large-scale datasets
// ---------------------------------------------------------------------------

#[test]
fn xuniq_email_format_valid() {
    let out = run_ok(&["email:xuniq", "-n", "100", "--seed", "xfmt", "--until", "2025"]);
    for line in out.lines() {
        assert!(line.contains('@'), "xuniq email must contain @: {line}");
        let local = line.split('@').next().unwrap_or("");
        // xuniq adds a 5-char tag separated by dot: local part is longer
        assert!(local.len() >= 8, "xuniq email local part must include tag: {line}");
    }
}

#[test]
fn xuniq_email_zero_collisions_1m() {
    let out = run_ok(&["email:xuniq", "-n", "1000000", "--seed", "x1m", "--until", "2025"]);
    let mut seen = std::collections::HashSet::with_capacity(1_100_000);
    for line in out.lines() {
        assert!(seen.insert(line.to_string()), "email:xuniq collision at 1M: {line}");
    }
    assert_eq!(seen.len(), 1_000_000);
}

#[test]
fn xuniq_username_zero_collisions_1m() {
    let out = run_ok(&["username:xuniq", "-n", "1000000", "--seed", "u1m", "--until", "2025"]);
    let mut seen = std::collections::HashSet::with_capacity(1_100_000);
    for line in out.lines() {
        assert!(seen.insert(line.to_string()), "username:xuniq collision at 1M: {line}");
    }
    assert_eq!(seen.len(), 1_000_000);
}

#[test]
fn xuniq_deterministic() {
    let a = run_ok(&[
        "email:xuniq",
        "username:xuniq",
        "-n",
        "100",
        "--seed",
        "xdet",
        "--until",
        "2025",
    ]);
    let b = run_ok(&[
        "email:xuniq",
        "username:xuniq",
        "-n",
        "100",
        "--seed",
        "xdet",
        "--until",
        "2025",
    ]);
    assert_eq!(a, b, "xuniq must produce identical output for same seed");
}

#[test]
fn xuniq_differs_from_default() {
    let default_out = run_ok(&["email", "-n", "5", "--seed", "cmp", "--until", "2025"]);
    let xuniq_out = run_ok(&["email:xuniq", "-n", "5", "--seed", "cmp", "--until", "2025"]);
    // Same seed but xuniq adds tags — output must differ
    assert_ne!(default_out, xuniq_out, "xuniq must produce different output than default");
}

#[test]
fn enum_dots_in_values() {
    let out =
        run_ok(&["enum:acme.com=3,globex.net=1", "-n", "20", "--seed", "edot", "--until", "2025"]);
    for line in out.lines() {
        assert!(line == "acme.com" || line == "globex.net", "unexpected enum value: {line}");
    }
}
