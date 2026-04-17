/// CLI behavioral tests: snapshots, locale/script, context correlation,
/// argument validation, replace mode, timezone, rate.
mod common;
use common::{run_fail, run_ok, seedfaker};

// ---------------------------------------------------------------------------
// Inline snapshots (exact output verification)
// ---------------------------------------------------------------------------

#[test]
fn snapshot_locale_de() {
    let out = run_ok(&["name", "--locale", "de", "-n", "3", "--seed", "snap2", "--until", "2025"]);
    assert_eq!(out, "Luisa Heim\nRegina Beck\nIsolde Jansen\n");
}

#[test]
fn snapshot_abc_native_sr() {
    let out = run_ok(&[
        "name", "--locale", "sr", "--abc", "native", "-n", "3", "--seed", "snap2", "--until",
        "2025",
    ]);
    assert_eq!(out, "Огњен Дурић\nВида Цветковић\nРада Станишић\n");
}

#[test]
fn snapshot_ctx_strict() {
    let out = run_ok(&[
        "name", "email", "--ctx", "strict", "--locale", "de", "-n", "3", "--seed", "snap2",
        "--until", "2025",
    ]);
    assert_eq!(
        out,
        "Ludger Jahn\tludgerj@thyssen-krupp.de\n\
         Petra Haag\tcobra.grind_900@continental.de\n\
         Jonas Bittner\tjonas-bittnerz@man.de\n"
    );
}

// ---------------------------------------------------------------------------
// Locale and script behavior
// ---------------------------------------------------------------------------

#[test]
fn locale_de_produces_german_names() {
    let de = run_ok(&["name", "--locale", "de", "-n", "10", "--seed", "loc", "--until", "2025"]);
    let en = run_ok(&["name", "--locale", "en", "-n", "10", "--seed", "loc", "--until", "2025"]);
    assert_ne!(de, en, "German and English locales should differ");
}

#[test]
fn abc_native_produces_non_latin() {
    let out = run_ok(&[
        "name", "--locale", "sr", "--abc", "native", "-n", "10", "--seed", "abc", "--until", "2025",
    ]);
    let has_cyrillic = out.chars().any(|c| ('\u{0400}'..='\u{04FF}').contains(&c));
    assert!(has_cyrillic, "Serbian native should contain Cyrillic");
}

#[test]
fn abc_mixed_contains_both_scripts() {
    let latin = run_ok(&["name", "--locale", "uk", "-n", "20", "--seed", "abc", "--until", "2025"]);
    let mixed = run_ok(&[
        "name", "--locale", "uk", "--abc", "mixed", "-n", "20", "--seed", "abc", "--until", "2025",
    ]);
    assert_ne!(latin, mixed, "mixed should differ from default Latin");
}

// ---------------------------------------------------------------------------
// Context correlation
// ---------------------------------------------------------------------------

#[test]
fn ctx_strict_email_contains_name() {
    let out = run_ok(&[
        "name", "email", "--ctx", "strict", "-n", "20", "--seed", "corr", "--until", "2025",
    ]);
    let mut name_in_email = 0;
    for line in out.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let name_lower = parts[0].to_lowercase();
            let first = name_lower.split_whitespace().next().unwrap_or("");
            if parts[1].to_lowercase().contains(first) && first.len() >= 2 {
                name_in_email += 1;
            }
        }
    }
    assert!(
        name_in_email >= 10,
        "ctx strict: most emails should contain first name ({name_in_email}/20)"
    );
}

#[test]
fn no_ctx_fields_mostly_independent() {
    let out = run_ok(&["name", "email", "-n", "20", "--seed", "noctx", "--until", "2025"]);
    let mut name_in_email = 0;
    for line in out.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() == 2 {
            let name_lower = parts[0].to_lowercase();
            let first = name_lower.split_whitespace().next().unwrap_or("");
            if parts[1].to_lowercase().contains(first) && !first.is_empty() {
                name_in_email += 1;
            }
        }
    }
    assert!(name_in_email < 20, "without ctx, not all emails should match name");
}

// ---------------------------------------------------------------------------
// Argument validation
// ---------------------------------------------------------------------------

#[test]
fn rate_flag_accepted() {
    let out = seedfaker()
        .args(["name", "-n", "3", "--seed", "rate", "--until", "2025", "--rate", "10"])
        .output()
        .expect("execute");
    assert!(out.status.success());
}

#[test]
fn rate_zero_rejected() {
    run_fail(&["name", "-n", "3", "--rate", "0"]);
}

#[test]
fn unknown_field_rejected() {
    run_fail(&["nonexistent-field", "-n", "1"]);
}

#[test]
fn no_fields_rejected() {
    run_fail(&["-n", "1"]);
}

#[test]
fn unknown_locale_rejected() {
    run_fail(&["name", "--locale", "zz", "-n", "1"]);
}

#[test]
fn unknown_modifier_rejected() {
    run_fail(&["name:foobar", "-n", "1"]);
    run_fail(&["name:reverse", "-n", "1"]);
}

#[test]
fn n_exceeds_limit_rejected() {
    run_fail(&["name", "-n", "10000000001"]);
}

#[test]
fn zip_field_rejected() {
    run_fail(&["zip", "-n", "1"]);
}

#[test]
fn n_equals_one() {
    let out = run_ok(&["name", "-n", "1", "--seed", "one", "--until", "2025"]);
    assert_eq!(out.lines().count(), 1);
    assert!(!out.trim().is_empty());
}

// ---------------------------------------------------------------------------
// Replace mode
// ---------------------------------------------------------------------------

fn run_replace(input: &[u8], args: &[&str]) -> (bool, String) {
    let mut cmd = seedfaker();
    cmd.args(args);
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().expect("spawn");
    {
        use std::io::Write;
        // Process may exit before reading all input (e.g. validation error) — ignore BrokenPipe
        let _ = child.stdin.as_mut().expect("stdin").write_all(input);
    }
    let out = child.wait_with_output().expect("wait");
    (out.status.success(), String::from_utf8(out.stdout).expect("utf8"))
}

#[test]
fn replace_substitutes_specified_columns() {
    let (ok, stdout) = run_replace(
        b"name,email\nAlice,alice@example.com\nBob,bob@example.com\n",
        &["replace", "email", "--seed", "rep", "--until", "2025"],
    );
    assert!(ok);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "1 header + 2 rows");
    assert_eq!(lines[0], "name,email");
    assert!(lines[1].starts_with("Alice,"), "name preserved");
    assert!(!lines[1].contains("alice@example.com"), "email replaced");
    assert!(lines[1].contains('@'), "replacement is valid email");
}

#[test]
fn replace_multi_column() {
    let (ok, stdout) = run_replace(
        b"name,email,phone,ssn\nAlice,a@x.com,555-1234,123-45-6789\n",
        &["replace", "email", "phone", "ssn", "--seed", "multi", "--until", "2025"],
    );
    assert!(ok);
    let lines: Vec<&str> = stdout.lines().collect();
    let cols: Vec<&str> = lines[1].split(',').collect();
    assert_eq!(cols[0], "Alice", "name preserved");
    assert_ne!(cols[1], "a@x.com", "email replaced");
    assert_ne!(cols[2], "555-1234", "phone replaced");
    assert_ne!(cols[3], "123-45-6789", "ssn replaced");
}

#[test]
fn replace_deterministic() {
    let input = b"name,email\nAlice,alice@example.com\nBob,bob@example.com\n";
    let (_, a) = run_replace(input, &["replace", "email", "--seed", "rep-det", "--until", "2025"]);
    let (_, b) = run_replace(input, &["replace", "email", "--seed", "rep-det", "--until", "2025"]);
    assert_eq!(a, b, "replace must be deterministic");
}

#[test]
fn replace_empty_input() {
    let (ok, stdout) =
        run_replace(b"email\n", &["replace", "email", "--seed", "empty", "--until", "2025"]);
    assert!(ok);
    assert_eq!(stdout.trim(), "email", "header-only input = header-only output");
}

#[test]
fn replace_modifier_rejected() {
    let (ok, _) = run_replace(
        b"email\na@b.com\n",
        &["replace", "email:space", "--seed", "mod", "--until", "2025"],
    );
    assert!(!ok, "replace with modifier should fail");
}

#[test]
fn replace_auto_detect_csv() {
    let (ok, text) = run_replace(
        b"name,email\nAlice,alice@x.com\nBob,bob@y.com\n",
        &["replace", "email", "--seed", "rep", "--until", "2025"],
    );
    assert!(ok);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "name,email");
    assert!(lines[1].starts_with("Alice,"));
    assert!(lines[1].contains('@'));
}

#[test]
fn replace_auto_detect_jsonl() {
    let input = br#"{"name":"Alice","email":"alice@x.com"}
{"name":"Bob","email":"bob@y.com"}
"#;
    let (ok, text) = run_replace(input, &["replace", "email", "--seed", "rep", "--until", "2025"]);
    assert!(ok);
    for line in text.lines() {
        let v: serde_json::Value = serde_json::from_str(line).expect("json");
        assert!(v.get("name").is_some());
        assert!(v.get("email").is_some());
        assert_ne!(v["email"], "alice@x.com");
    }
}

// ---------------------------------------------------------------------------
// Timezone
// ---------------------------------------------------------------------------

#[test]
fn tz_default_is_utc() {
    let out = run_ok(&["timestamp", "--seed", "tz1", "--until", "2025"]);
    assert!(out.contains('Z'), "default should use Z suffix");
}

#[test]
fn tz_positive_offset() {
    let out = run_ok(&["timestamp", "--seed", "tz1", "--until", "2025", "--tz", "+0530"]);
    assert!(out.contains("+05:30"), "should contain +05:30");
}

#[test]
fn tz_negative_offset() {
    let out = run_ok(&["timestamp", "--seed", "tz1", "--until", "2025", "--tz", "-08:00"]);
    assert!(out.contains("-08:00"), "should contain -08:00");
}

#[test]
fn tz_log_format() {
    let out = run_ok(&["timestamp:log", "--seed", "tz1", "--until", "2025", "--tz", "+0300"]);
    assert!(out.contains("+0300"), "log format should use +HHMM");
}

#[test]
fn tz_invalid_rejected() {
    run_fail(&["timestamp", "--tz", "banana"]);
}

#[test]
fn tz_minutes_validation() {
    run_fail(&["timestamp", "--tz", "+0060"]);
}
