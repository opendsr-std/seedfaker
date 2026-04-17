/// Determinism suite: identical seed + options MUST produce identical output.
/// Tests cover all axes: fields, groups, modifiers, transforms, formats,
/// locales, ctx modes, abc modes, corruption, templates, ranges, configs.
///
/// Cross-release stability is verified via golden.tsv and fingerprint.
/// Field independence guarantees that adding/removing fields does not
/// change existing columns.
mod common;
use common::run_ok;

// ---------------------------------------------------------------------------
// Same seed = byte-identical output
// ---------------------------------------------------------------------------

#[test]
fn determinism_all_groups() {
    let groups = [
        "core",
        "text",
        "time",
        "person",
        "contact",
        "location",
        "finance",
        "auth",
        "internet",
        "blockchain",
        "organization",
        "healthcare",
        "dev",
        "ops",
        "device",
        "gov-id",
    ];
    for g in &groups {
        let a = run_ok(&[g, "-n", "30", "--seed", "grp-det", "--until", "2030"]);
        let b = run_ok(&[g, "-n", "30", "--seed", "grp-det", "--until", "2030"]);
        assert_eq!(a, b, "group '{g}' not deterministic");
    }
}

#[test]
fn determinism_all_formats() {
    let a_csv = run_ok(&[
        "name", "email", "--format", "csv", "-n", "30", "--seed", "fmt-det", "--until", "2025",
    ]);
    let b_csv = run_ok(&[
        "name", "email", "--format", "csv", "-n", "30", "--seed", "fmt-det", "--until", "2025",
    ]);
    assert_eq!(a_csv, b_csv, "CSV not deterministic");

    let a_json = run_ok(&[
        "name", "email", "--format", "jsonl", "-n", "30", "--seed", "fmt-det", "--until", "2025",
    ]);
    let b_json = run_ok(&[
        "name", "email", "--format", "jsonl", "-n", "30", "--seed", "fmt-det", "--until", "2025",
    ]);
    assert_eq!(a_json, b_json, "JSONL not deterministic");

    let a_sql = run_ok(&[
        "name", "email", "--format", "sql=t", "-n", "30", "--seed", "fmt-det", "--until", "2025",
    ]);
    let b_sql = run_ok(&[
        "name", "email", "--format", "sql=t", "-n", "30", "--seed", "fmt-det", "--until", "2025",
    ]);
    assert_eq!(a_sql, b_sql, "SQL not deterministic");
}

#[test]
fn determinism_all_modifiers() {
    let modifiers = [
        "credit-card:space",
        "credit-card:plain",
        "ssn:plain",
        "phone:e164",
        "iban:plain",
        "mac:plain",
        "mac:plain:upper",
        "uuid:plain",
        "birthdate:us",
        "birthdate:eu",
        "amount:plain",
        "amount:usd",
        "amount:eur",
        "timestamp:unix",
        "timestamp:log",
        "country-code:alpha3",
        "country-code:numeric",
        "passport:international",
        "passport:internal",
        "password:pin",
        "password:memorable",
        "password:mixed",
        "date:us",
        "date:eu",
    ];
    for m in &modifiers {
        let a = run_ok(&[m, "-n", "30", "--seed", "mod-det", "--until", "2030"]);
        let b = run_ok(&[m, "-n", "30", "--seed", "mod-det", "--until", "2030"]);
        assert_eq!(a, b, "modifier '{m}' not deterministic");
    }
}

#[test]
fn determinism_transforms() {
    let transforms = ["name:upper", "name:lower", "email:capitalize"];
    for tr in &transforms {
        let a = run_ok(&[tr, "-n", "30", "--seed", "tr-det", "--until", "2025"]);
        let b = run_ok(&[tr, "-n", "30", "--seed", "tr-det", "--until", "2025"]);
        assert_eq!(a, b, "transform '{tr}' not deterministic");
    }
}

#[test]
fn determinism_corruption_levels() {
    for level in &["low", "mid", "high", "extreme"] {
        let a = run_ok(&[
            "name",
            "email",
            "phone",
            "--corrupt",
            level,
            "-n",
            "50",
            "--seed",
            "cor-det",
        ]);
        let b = run_ok(&[
            "name",
            "email",
            "phone",
            "--corrupt",
            level,
            "-n",
            "50",
            "--seed",
            "cor-det",
        ]);
        assert_eq!(a, b, "corrupt '{level}' not deterministic");
    }
}

#[test]
fn determinism_ctx_modes() {
    let a = run_ok(&[
        "name", "email", "iban", "--ctx", "strict", "--locale", "de,fr", "-n", "30", "--seed",
        "ctx-det",
    ]);
    let b = run_ok(&[
        "name", "email", "iban", "--ctx", "strict", "--locale", "de,fr", "-n", "30", "--seed",
        "ctx-det",
    ]);
    assert_eq!(a, b, "--ctx strict not deterministic");

    let a = run_ok(&[
        "name", "email", "--ctx", "loose", "-n", "30", "--seed", "ctx-det", "--until", "2025",
    ]);
    let b = run_ok(&[
        "name", "email", "--ctx", "loose", "-n", "30", "--seed", "ctx-det", "--until", "2025",
    ]);
    assert_eq!(a, b, "--ctx loose not deterministic");
}

#[test]
fn determinism_abc_modes() {
    let a = run_ok(&[
        "name", "phone", "--locale", "sr", "--abc", "native", "-n", "30", "--seed", "abc-det",
        "--until", "2025",
    ]);
    let b = run_ok(&[
        "name", "phone", "--locale", "sr", "--abc", "native", "-n", "30", "--seed", "abc-det",
        "--until", "2025",
    ]);
    assert_eq!(a, b, "--abc native not deterministic");

    let a = run_ok(&[
        "name", "phone", "--locale", "uk", "--abc", "mixed", "-n", "30", "--seed", "abc-det",
        "--until", "2025",
    ]);
    let b = run_ok(&[
        "name", "phone", "--locale", "uk", "--abc", "mixed", "-n", "30", "--seed", "abc-det",
        "--until", "2025",
    ]);
    assert_eq!(a, b, "--abc mixed not deterministic");
}

#[test]
fn determinism_combo() {
    let args = &[
        "name",
        "email",
        "phone",
        "--locale",
        "de,fr",
        "--ctx",
        "strict",
        "--abc",
        "mixed",
        "--corrupt",
        "mid",
        "-n",
        "30",
        "--seed",
        "combo-det",
    ];
    let a = run_ok(args);
    let b = run_ok(args);
    assert_eq!(a, b, "combo flags not deterministic");
}

#[test]
fn determinism_template() {
    let args = &[
        "-t",
        "{{name}} {{phone:e164}} {{credit-card:space}}",
        "-n",
        "30",
        "--seed",
        "tpl-det",
        "--until",
        "2025",
    ];
    let a = run_ok(args);
    let b = run_ok(args);
    assert_eq!(a, b, "template not deterministic");
}

#[test]
fn determinism_ranges() {
    let cases: &[&[&str]] = &[
        &["integer:1..100", "-n", "50", "--seed", "r1", "--until", "2025"],
        &["date:2020..2025", "-n", "50", "--seed", "r2", "--until", "2025"],
        &["amount:100..500:usd", "-n", "50", "--seed", "r3", "--until", "2025"],
        &["birthdate:1990..2000:eu", "-n", "50", "--seed", "r4", "--until", "2025"],
        &["timestamp:2024..2025:log", "-n", "50", "--seed", "r5", "--until", "2025"],
    ];
    for args in cases {
        let a = run_ok(args);
        let b = run_ok(args);
        assert_eq!(a, b, "range args {:?} not deterministic", args);
    }
}

#[test]
fn determinism_all_locales() {
    let locales = &[
        "en", "en-gb", "en-ca", "en-au", "en-nz", "en-sg", "en-za", "en-ng", "de", "de-at", "fr",
        "fr-ca", "fr-be", "ja", "es", "pt-br", "it", "nl", "nl-be", "pl", "se", "tr", "uk", "be",
        "sr", "ar", "ar-sa", "ar-ae", "ro", "hr", "bg", "cs", "sk", "hu", "fi", "da", "no", "el",
        "pt", "mx", "cl", "co", "sl", "et", "lt", "lv", "ie", "pe", "uy", "hi", "vi", "zh", "ko",
        "id", "th", "ms", "tl", "tw", "ve", "ec", "pk", "bd", "eg", "cy", "mt", "lb", "he",
    ];
    for loc in locales {
        let a = run_ok(&[
            "name", "email", "phone", "--locale", loc, "-n", "10", "--seed", "loc-det", "--until",
            "2025",
        ]);
        let b = run_ok(&[
            "name", "email", "phone", "--locale", loc, "-n", "10", "--seed", "loc-det", "--until",
            "2025",
        ]);
        assert_eq!(a, b, "locale '{loc}' not deterministic");
    }
}

// ---------------------------------------------------------------------------
// Different seeds = different output
// ---------------------------------------------------------------------------

#[test]
fn different_seed_different_output() {
    let a = run_ok(&["name", "email", "-n", "10", "--seed", "alpha", "--until", "2025"]);
    let b = run_ok(&["name", "email", "-n", "10", "--seed", "beta", "--until", "2025"]);
    assert_ne!(a, b, "different seeds should produce different output");
}

// ---------------------------------------------------------------------------
// Field independence
// ---------------------------------------------------------------------------

#[test]
fn field_independence() {
    let name_only = run_ok(&["name", "--seed", "indep", "--until", "2025", "-n", "3"]);
    let name_with_email =
        run_ok(&["name", "email", "--seed", "indep", "--until", "2025", "-n", "3"]);
    let name_with_many = run_ok(&[
        "name",
        "email",
        "phone",
        "ssn",
        "credit-card",
        "uuid",
        "--seed",
        "indep",
        "-n",
        "3",
    ]);
    let name_with_csv = run_ok(&[
        "name",
        "--seed",
        "indep",
        "--until",
        "2025",
        "-n",
        "3",
        "--format",
        "csv",
        "--no-header",
    ]);

    let extract_names = |s: &str| -> Vec<String> {
        s.lines().map(|l| l.split('\t').next().unwrap_or("").to_string()).collect()
    };

    let base = extract_names(&name_only);
    assert_eq!(base, extract_names(&name_with_email), "adding email changed name");
    assert_eq!(base, extract_names(&name_with_many), "adding many fields changed name");
    let from_csv: Vec<String> = name_with_csv.lines().map(String::from).collect();
    assert_eq!(base, from_csv, "CSV format changed name");
}

#[test]
fn ctx_identity_independence() {
    let name_email = run_ok(&[
        "name", "email", "--seed", "ctx-ind", "--until", "2025", "--ctx", "strict", "--locale",
        "de", "-n", "3",
    ]);
    let name_email_phone = run_ok(&[
        "name", "email", "phone", "--seed", "ctx-ind", "--until", "2025", "--ctx", "strict",
        "--locale", "de", "-n", "3",
    ]);

    let extract = |s: &str| -> Vec<(String, String)> {
        s.lines()
            .map(|l| {
                let p: Vec<&str> = l.split('\t').collect();
                (p[0].to_string(), p.get(1).unwrap_or(&"").to_string())
            })
            .collect()
    };

    assert_eq!(
        extract(&name_email),
        extract(&name_email_phone),
        "adding phone changed name+email with ctx strict"
    );
}

// ---------------------------------------------------------------------------
// Cross-release stability
// ---------------------------------------------------------------------------

#[test]
fn golden_file_cross_release() {
    let golden = include_str!("golden.tsv");
    let mut failures: Vec<String> = Vec::new();
    for line in golden.lines() {
        let Some((field, expected)) = line.split_once('\t') else {
            continue;
        };
        // enum requires user-supplied values — tested separately
        if field == "enum" {
            continue;
        }
        let actual =
            run_ok(&[field, "--locale", "en", "--seed", "golden", "--until", "2038", "-n", "1"]);
        let first_line = actual.lines().next().unwrap_or("");
        if first_line != expected {
            failures
                .push(format!("  {field}:\n    expected: {expected}\n    actual:   {first_line}"));
        }
    }
    assert!(
        failures.is_empty(),
        "golden file mismatch — cross-release determinism broken:\n{}",
        failures.join("\n")
    );
}

#[test]
fn fingerprint_matches() {
    let out = run_ok(&["--fingerprint"]);
    assert_eq!(
        out.trim(),
        "sf0-158dc9f79ce46b43",
        "fingerprint changed — run 'make stamp-fingerprint' to update"
    );
}

// ---------------------------------------------------------------------------
// Cross-mode determinism: TSV, template, config produce identical values
// ---------------------------------------------------------------------------

#[test]
fn cross_mode_determinism() {
    let tsv = run_ok(&["email", "-n", "5", "--seed", "xmode", "--until", "2025"]);
    let tpl =
        run_ok(&["email", "-t", "{{email}}", "-n", "5", "--seed", "xmode", "--until", "2025"]);

    let dir = std::env::temp_dir().join(format!("seedfaker-xmode-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("mkdir");
    let path = dir.join("x.yaml");
    std::fs::write(&path, "columns:\n  email: email\ntemplate: '{{email}}'\n").expect("write");
    let cfg = run_ok(&[
        "run",
        path.to_str().expect("p"),
        "-n",
        "5",
        "--seed",
        "xmode",
        "--until",
        "2025",
    ]);
    let _ = std::fs::remove_dir_all(&dir);

    let tsv_lines: Vec<&str> = tsv.lines().collect();
    let tpl_lines: Vec<&str> = tpl.lines().collect();
    let cfg_lines: Vec<&str> = cfg.lines().collect();

    assert_eq!(tsv_lines, tpl_lines, "TSV vs template mismatch");
    assert_eq!(tsv_lines, cfg_lines, "TSV vs config mismatch");
}

// ---------------------------------------------------------------------------
// Cross-mode: date fields use var reference (not {{new}}) for consistency
// ---------------------------------------------------------------------------

#[test]
fn cross_mode_date_with_var_reference() {
    // When date is a var (not {{new}}), TSV and template use same domain key
    let tsv = run_ok(&["date:2020..2022", "-n", "10", "--seed", "xdate", "--until", "2025"]);
    let tpl = run_ok(&[
        "date:2020..2022",
        "-t",
        "{{date}}",
        "-n",
        "10",
        "--seed",
        "xdate",
        "--until",
        "2025",
    ]);
    let tsv_lines: Vec<&str> = tsv.lines().collect();
    let tpl_lines: Vec<&str> = tpl.lines().collect();
    assert_eq!(tsv_lines, tpl_lines, "date var: TSV vs template mismatch");
}
