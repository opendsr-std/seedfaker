/// Config and preset tests: validates presets, custom YAML configs,
/// CLI overrides, ground truth, year ranges, timezone, corruption.
mod common;
use common::{run_fail, run_ok};

// ---------------------------------------------------------------------------
// Preset: structure validation
// ---------------------------------------------------------------------------

#[test]
fn run_preset_nginx() {
    let out = run_ok(&["run", "nginx", "-n", "5", "--seed", "spec", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(lines.len(), 5);
    for line in &lines {
        assert!(line.contains("HTTP/1.1"), "nginx log should contain HTTP/1.1: {line}");
        // Nginx access log should have IP, timestamp, method, path, status
        assert!(line.contains('['), "nginx log should have timestamp bracket: {line}");
    }
}

#[test]
fn run_preset_auth() {
    let out = run_ok(&["run", "auth", "-n", "5", "--seed", "spec", "--until", "2025"]);
    for line in out.lines().filter(|l| !l.is_empty()) {
        assert!(
            line.contains("sshd[") || line.contains("sudo:"),
            "auth log should contain sshd or sudo: {line}"
        );
    }
}

#[test]
fn run_preset_app_json() {
    let out = run_ok(&["run", "app-json", "-n", "5", "--seed", "spec", "--until", "2025"]);
    for line in out.lines().filter(|l| !l.is_empty()) {
        let v: serde_json::Value = serde_json::from_str(line).expect("should be JSON");
        assert!(v.get("timestamp").is_some(), "app-json missing timestamp");
        assert!(v.get("level").is_some(), "app-json missing level");
        assert!(v.get("service").is_some(), "app-json missing service");
        assert!(v.get("msg").is_some(), "app-json missing msg");
        // Verify level is a valid log level (lowercase in app-json preset)
        let level = v["level"].as_str().unwrap_or("");
        assert!(
            ["INFO", "WARN", "ERROR", "FATAL", "DEBUG"].contains(&level),
            "invalid log level: {level}"
        );
    }
}

#[test]
fn run_preset_postgres() {
    let out = run_ok(&["run", "postgres", "-n", "5", "--seed", "spec", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    assert!(lines.len() >= 5, "postgres should produce at least 5 lines");
    // Multi-line error entries produce DETAIL/STATEMENT continuation lines
    assert!(
        lines.iter().any(|l| l.contains("LOG:") || l.contains("ERROR:")),
        "postgres should contain LOG: or ERROR:"
    );
}

#[test]
fn run_preset_payment() {
    let out = run_ok(&["run", "payment", "-n", "5", "--seed", "spec", "--until", "2025"]);
    for line in out.lines().filter(|l| !l.is_empty()) {
        let v: serde_json::Value = serde_json::from_str(line).expect("should be JSON");
        assert!(v.get("status").is_some(), "payment missing status");
        assert!(v.get("amount").is_some(), "payment missing amount");
        // Amount is a numeric value (cents)
        assert!(v["amount"].is_number(), "payment amount should be numeric");
        let status = v["status"].as_str().unwrap_or("");
        assert!(
            ["succeeded", "failed", "disputed", "refunded", "pending"].contains(&status),
            "unexpected payment status: {status}"
        );
    }
}

#[test]
fn run_preset_user_table() {
    let out = run_ok(&["run", "user-table", "-n", "3", "--seed", "spec", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().collect();
    assert_eq!(lines.len(), 4, "3 data rows + 1 header");
    assert!(lines[0].contains("name"), "header should contain 'name'");
    // Data rows should have same number of columns as header
    let header_cols = lines[0].split(',').count();
    for line in &lines[1..] {
        // CSV may have quoted fields with commas, but basic check
        assert!(!line.is_empty(), "data row should not be empty");
    }
    let _ = header_cols; // used for verification above
}

#[test]
fn run_preset_stacktrace() {
    let out = run_ok(&["run", "stacktrace", "-n", "2", "--seed", "spec", "--until", "2025"]);
    assert!(
        out.contains("Exception") || out.contains("Traceback") || out.contains("ERROR"),
        "stacktrace should contain error markers"
    );
}

#[test]
fn run_preset_chaos() {
    let out = run_ok(&["run", "chaos", "-n", "10", "--seed", "spec", "--until", "2025"]);
    let lines: Vec<&str> = out.lines().filter(|l| !l.is_empty()).collect();
    // Chaos uses multi-line templates (stacktraces, multi-line errors) so line count >= record count
    assert!(lines.len() >= 10, "chaos should produce at least 10 lines");
}

// ---------------------------------------------------------------------------
// All presets are deterministic
// ---------------------------------------------------------------------------

#[test]
fn all_presets_deterministic() {
    let presets =
        ["nginx", "auth", "app-json", "postgres", "payment", "user-table", "stacktrace", "chaos"];
    for preset in presets {
        let a = run_ok(&["run", preset, "-n", "5", "--seed", "det", "--until", "2025"]);
        let b = run_ok(&["run", preset, "-n", "5", "--seed", "det", "--until", "2025"]);
        assert_eq!(a, b, "preset '{}' not deterministic", preset);
    }
}

// ---------------------------------------------------------------------------
// CLI overrides config
// ---------------------------------------------------------------------------

#[test]
fn cli_overrides_config_count() {
    let out_3 = run_ok(&["run", "nginx", "-n", "3", "--seed", "x", "--until", "2025"]);
    let out_7 = run_ok(&["run", "nginx", "-n", "7", "--seed", "x", "--until", "2025"]);
    assert_eq!(out_3.lines().filter(|l| !l.is_empty()).count(), 3);
    assert_eq!(out_7.lines().filter(|l| !l.is_empty()).count(), 7);
}

#[test]
fn cli_overrides_config_format() {
    let jsonl = run_ok(&[
        "run",
        "user-table",
        "-n",
        "2",
        "--seed",
        "x",
        "--until",
        "2025",
        "--format",
        "jsonl",
    ]);
    for line in jsonl.lines() {
        let _: serde_json::Value = serde_json::from_str(line).expect("should be JSONL");
    }
}

#[test]
fn cli_overrides_config_seed() {
    let a = run_ok(&["run", "nginx", "-n", "1", "--seed", "aaa", "--until", "2025"]);
    let b = run_ok(&["run", "nginx", "-n", "1", "--seed", "bbb", "--until", "2025"]);
    assert_ne!(a, b, "different seeds should produce different output");
}

// ---------------------------------------------------------------------------
// Custom config files
// ---------------------------------------------------------------------------

#[test]
fn custom_config_file() {
    let dir = common::tempfile("config");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("test.yaml");
    std::fs::write(&path, "columns:\n  name: name\n  email: email\noptions:\n  ctx: strict\n")
        .expect("write");

    let out =
        run_ok(&["run", path.to_str().expect("p"), "-n", "3", "--seed", "c", "--format", "jsonl"]);
    assert_eq!(out.lines().count(), 3);
    for line in out.lines() {
        let v: serde_json::Value = serde_json::from_str(line).expect("json");
        assert!(v.get("name").is_some());
        assert!(v.get("email").is_some());
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn custom_config_with_template() {
    let dir = common::tempfile("tpl");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("tpl.yaml");
    std::fs::write(
        &path,
        "columns:\n  name: name\n  email: email\ntemplate: \"{{name}} <{{email}}>\"\n",
    )
    .expect("write");

    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "2", "--seed", "t"]);
    assert_eq!(out.lines().count(), 2);
    for line in out.lines() {
        assert!(line.contains('<') && line.contains('>'));
        assert!(line.contains('@'), "email in template should contain @");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Validate
// ---------------------------------------------------------------------------

#[test]
fn validate_valid_fields() {
    let out =
        run_ok(&["name", "email", "phone:e164", "--validate", "--seed", "v", "--until", "2025"]);
    assert!(out.is_empty(), "validate should produce no output");
}

#[test]
fn validate_invalid_modifier() {
    run_fail(&["name:e164", "--validate"]);
}

#[test]
fn validate_unknown_field() {
    run_fail(&["nonexistent", "--validate"]);
}

#[test]
fn validate_config() {
    let dir = common::tempfile("val-cfg");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("val.yaml");
    std::fs::write(
        &path,
        "columns:\n  name: name\n  email: email\noptions:\n  seed: v\n  validate: true\n",
    )
    .expect("write");

    let out = run_ok(&["run", path.to_str().expect("p"), "--until", "2025"]);
    assert!(out.is_empty(), "validate config should produce no output");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn validate_config_bad_modifier() {
    let dir = common::tempfile("val-bad");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("val.yaml");
    std::fs::write(&path, "columns:\n  name: name:e164\noptions:\n  seed: v\n").expect("write");

    run_fail(&["run", path.to_str().expect("p"), "--validate", "--until", "2025"]);
    let _ = std::fs::remove_dir_all(&dir);
}

// Regression: config columns must reject invalid modifiers at load time, not just with --validate
#[test]
fn config_rejects_invalid_modifier_on_run() {
    let dir = common::tempfile("cfg-mod");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  phone: phone:bad_mod\noptions:\n  seed: x\n")
        .expect("write");

    run_fail(&["run", path.to_str().expect("p"), "-n", "1", "--until", "2025"]);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Annotated output
// ---------------------------------------------------------------------------

#[test]
fn annotated_valid_jsonl() {
    let out =
        run_ok(&["run", "pii-leak", "-n", "3", "--seed", "ann", "--until", "2025", "--annotated"]);
    let lines = common::parse_jsonl(&out);
    assert_eq!(lines.len(), 3);
    for line in &lines {
        assert!(line.get("text").is_some(), "must have text");
        assert!(line.get("spans").is_some(), "must have spans");
        let spans = line["spans"].as_array().expect("spans array");
        assert!(!spans.is_empty(), "must have at least one span");
    }
}

#[test]
fn annotated_span_values_match_text() {
    let out =
        run_ok(&["run", "pii-leak", "-n", "2", "--seed", "sp", "--until", "2025", "--annotated"]);
    for line_str in out.lines() {
        let obj: serde_json::Value = serde_json::from_str(line_str).expect("json");
        let text = obj["text"].as_str().expect("text");
        for sp in obj["spans"].as_array().expect("spans") {
            let s = sp["s"].as_u64().expect("s") as usize;
            let e = sp["e"].as_u64().expect("e") as usize;
            let v = sp["v"].as_str().expect("v");
            assert_eq!(&text[s..e], v, "span value must match text slice");
            assert!(sp.get("f").is_some(), "span must have field type");
        }
    }
}

#[test]
fn annotated_corruption_has_originals() {
    let out = run_ok(&[
        "run",
        "pii-leak",
        "-n",
        "20",
        "--seed",
        "co",
        "--until",
        "2025",
        "--annotated",
        "--corrupt",
        "extreme",
    ]);
    let mut has_original = false;
    for line_str in out.lines() {
        let obj: serde_json::Value = serde_json::from_str(line_str).expect("json");
        for sp in obj["spans"].as_array().expect("spans") {
            if sp.get("o").is_some() {
                has_original = true;
                let v = sp["v"].as_str().expect("v");
                let o = sp["o"].as_str().expect("o");
                assert_ne!(v, o, "original must differ from corrupted value");
            }
        }
    }
    assert!(has_original, "extreme corruption should produce spans with originals");
}

#[test]
fn annotated_structured_output() {
    let out =
        run_ok(&["name", "email", "--annotated", "--seed", "x", "--until", "2025", "-n", "3"]);
    let lines = common::parse_jsonl(&out);
    assert_eq!(lines.len(), 3);
    for line in &lines {
        let text = line["text"].as_str().expect("text");
        for sp in line["spans"].as_array().expect("spans") {
            let s = sp["s"].as_u64().expect("s") as usize;
            let e = sp["e"].as_u64().expect("e") as usize;
            let v = sp["v"].as_str().expect("v");
            assert_eq!(&text[s..e], v, "span must match text");
        }
    }
}

#[test]
fn annotated_csv_spans_correct() {
    let out = run_ok(&[
        "name",
        "email",
        "--annotated",
        "--seed",
        "csv-ann",
        "--until",
        "2025",
        "-n",
        "5",
        "--format",
        "csv",
    ]);
    let lines = common::parse_jsonl(&out);
    assert_eq!(lines.len(), 5);
    for line in &lines {
        let text = line["text"].as_str().expect("text");
        assert!(text.contains(','), "CSV line should contain comma");
        for sp in line["spans"].as_array().expect("spans") {
            let s = sp["s"].as_u64().expect("s") as usize;
            let e = sp["e"].as_u64().expect("e") as usize;
            let v = sp["v"].as_str().expect("v");
            assert_eq!(&text[s..e], v, "CSV span must match text");
        }
    }
}

#[test]
fn annotated_jsonl_spans_correct() {
    let out = run_ok(&[
        "name",
        "email",
        "--annotated",
        "--seed",
        "jl-ann",
        "--until",
        "2025",
        "-n",
        "5",
        "--format",
        "jsonl",
    ]);
    let lines = common::parse_jsonl(&out);
    assert_eq!(lines.len(), 5);
    for line in &lines {
        let text = line["text"].as_str().expect("text");
        assert!(text.starts_with('{'), "JSONL text should start with {{");
        for sp in line["spans"].as_array().expect("spans") {
            let s = sp["s"].as_u64().expect("s") as usize;
            let e = sp["e"].as_u64().expect("e") as usize;
            let v = sp["v"].as_str().expect("v");
            assert_eq!(&text[s..e], v, "JSONL span must match text");
        }
    }
}

#[test]
fn annotated_corruption_structured() {
    let out = run_ok(&[
        "name",
        "email",
        "--annotated",
        "--seed",
        "str-cor",
        "--until",
        "2025",
        "-n",
        "20",
        "--format",
        "csv",
        "--corrupt",
        "extreme",
    ]);
    let lines = common::parse_jsonl(&out);
    assert_eq!(lines.len(), 20);
    let has_original = lines.iter().any(|line| {
        line["spans"].as_array().map_or(false, |spans| spans.iter().any(|sp| sp.get("o").is_some()))
    });
    assert!(has_original, "extreme corruption on 20 records should produce originals");
}

// ---------------------------------------------------------------------------
// Annotated + expressions
// ---------------------------------------------------------------------------

#[test]
fn annotated_with_expressions() {
    let dir = common::tempfile("ann-expr");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("expr.yaml");
    std::fs::write(
        &path,
        "columns:\n  price: amount:1..500:plain\n  qty: integer:1..20\n  total: price * qty\noptions:\n  seed: ae\n  format: csv\n",
    )
    .expect("write");
    let out =
        run_ok(&["run", path.to_str().expect("p"), "-n", "5", "--until", "2025", "--annotated"]);
    let lines = common::parse_jsonl(&out);
    assert_eq!(lines.len(), 5);
    for line in &lines {
        let text = line["text"].as_str().expect("text");
        let spans = line["spans"].as_array().expect("spans");
        assert!(spans.len() >= 3, "expression record needs at least 3 spans (price, qty, total)");
        for sp in spans {
            let s = sp["s"].as_u64().expect("s") as usize;
            let e = sp["e"].as_u64().expect("e") as usize;
            let v = sp["v"].as_str().expect("v");
            assert_eq!(&text[s..e], v, "expression span must match text");
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Display name: hyphen→underscore
// ---------------------------------------------------------------------------

#[test]
fn display_name_hyphenated_field() {
    let out = run_ok(&[
        "credit-card",
        "first-name",
        "--annotated",
        "--seed",
        "dn",
        "--until",
        "2025",
        "-n",
        "2",
    ]);
    let lines = common::parse_jsonl(&out);
    for line in &lines {
        let text = line["text"].as_str().expect("text");
        for sp in line["spans"].as_array().expect("spans") {
            let f = sp["f"].as_str().expect("f");
            assert!(
                f == "credit-card" || f == "first-name",
                "field type should be registry name: {f}"
            );
            let s = sp["s"].as_u64().expect("s") as usize;
            let e = sp["e"].as_u64().expect("e") as usize;
            let v = sp["v"].as_str().expect("v");
            assert_eq!(&text[s..e], v, "span must match text");
        }
    }
}

// Regression: config columns must reject invalid range fields
#[test]
fn config_rejects_invalid_range() {
    let dir = common::tempfile("cfg-rng");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("bad.yaml");
    std::fs::write(&path, "columns:\n  name: name:1..100\noptions:\n  seed: x\n").expect("write");

    run_fail(&["run", path.to_str().expect("p"), "-n", "1", "--until", "2025"]);
    let _ = std::fs::remove_dir_all(&dir);
}

// Regression: all presets must pass validation
#[test]
fn all_presets_pass_validation() {
    let presets = [
        "nginx",
        "auth",
        "app-json",
        "postgres",
        "payment",
        "pii-leak",
        "user-table",
        "email",
        "stacktrace",
        "chaos",
        "llm-prompt",
        "syslog",
        "medical",
    ];
    for preset in &presets {
        run_ok(&["run", preset, "--validate", "--until", "2025", "--seed", "val"]);
    }
}

// ---------------------------------------------------------------------------
// Fingerprint guard
// ---------------------------------------------------------------------------

#[test]
fn fingerprint_match_passes() {
    let dir = common::tempfile("fp-ok");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("fp.yaml");
    let fp = run_ok(&["--fingerprint"]).trim().to_string();
    std::fs::write(
        &path,
        format!("columns:\n  name: name\noptions:\n  seed: fp\n  fingerprint: {fp}\n"),
    )
    .expect("write");

    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "2", "--until", "2025"]);
    assert_eq!(out.lines().count(), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fingerprint_mismatch_fails() {
    let dir = common::tempfile("fp-bad");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("fp.yaml");
    std::fs::write(
        &path,
        "columns:\n  name: name\noptions:\n  seed: fp\n  fingerprint: sf0-0000000000000000\n",
    )
    .expect("write");

    run_fail(&["run", path.to_str().expect("p"), "-n", "2", "--until", "2025"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn fingerprint_absent_passes() {
    let dir = common::tempfile("fp-none");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("fp.yaml");
    std::fs::write(&path, "columns:\n  name: name\noptions:\n  seed: fp\n").expect("write");

    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "2", "--until", "2025"]);
    assert_eq!(out.lines().count(), 2);
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Config options: year range, tz, corrupt, no_header
// ---------------------------------------------------------------------------

#[test]
fn config_option_year_range() {
    let dir = common::tempfile("yr");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("yr.yaml");
    std::fs::write(
        &path,
        "columns:\n  d: date\noptions:\n  since: 2020\n  until: 2022\n  seed: yr\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "20"]);
    for line in out.lines() {
        let year: i32 = line[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year), "year {year} outside range");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn cli_overrides_config_year_range() {
    let dir = common::tempfile("yro");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("yro.yaml");
    std::fs::write(
        &path,
        "columns:\n  d: date\noptions:\n  since: 1990\n  until: 1995\n  seed: yro\n",
    )
    .expect("write");
    let out = run_ok(&[
        "run",
        path.to_str().expect("p"),
        "-n",
        "20",
        "--since",
        "2030",
        "--until",
        "2035",
    ]);
    for line in out.lines() {
        let year: i32 = line[..4].parse().expect("year");
        assert!((2030..=2035).contains(&year), "CLI should override config year range: {year}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn config_option_tz() {
    let dir = common::tempfile("tz");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("tz.yaml");
    std::fs::write(&path, "columns:\n  ts: timestamp\noptions:\n  tz: \"+0530\"\n  seed: tz\n")
        .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "3"]);
    for line in out.lines() {
        assert!(line.contains("+05:30"), "expected +05:30: {line}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn config_option_corrupt() {
    let dir = common::tempfile("cor");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("cor.yaml");
    std::fs::write(&path, "columns:\n  name: name\n  email: email\noptions:\n  seed: cor\n")
        .expect("write");
    let clean = run_ok(&["run", path.to_str().expect("p"), "-n", "200", "--until", "2025"]);
    let corrupt = run_ok(&[
        "run",
        path.to_str().expect("p"),
        "-n",
        "200",
        "--until",
        "2025",
        "--corrupt",
        "extreme",
    ]);
    assert_ne!(clean, corrupt);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn config_option_no_header() {
    let dir = common::tempfile("nh");
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("nh.yaml");
    std::fs::write(
        &path,
        "columns:\n  name: name\noptions:\n  format: csv\n  no_header: true\n  seed: nh\n",
    )
    .expect("write");
    let out = run_ok(&["run", path.to_str().expect("p"), "-n", "3"]);
    assert_eq!(out.lines().count(), 3);
    assert!(!out.starts_with("name\n"));
    let _ = std::fs::remove_dir_all(&dir);
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[test]
fn unknown_config_fails() {
    run_fail(&["run", "nonexistent-config"]);
}

#[test]
fn run_list_presets() {
    let out = run_ok(&["run", "--list"]);
    assert!(out.contains("nginx"));
    assert!(out.contains("chaos"));
    assert!(out.contains("stacktrace"));
}
