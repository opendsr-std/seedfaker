/// Corruption tests: validates all 6 levels, tier boundaries, rate ordering,
/// structural integrity, determinism, and output format compatibility.
mod common;
use common::{run_fail, run_ok, seedfaker};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn count_diff_fields(clean: &str, corrupted: &str) -> usize {
    clean
        .lines()
        .zip(corrupted.lines())
        .flat_map(|(cl, co)| cl.split('\t').zip(co.split('\t')))
        .filter(|(c, o)| c != o)
        .count()
}

fn collect_corrupted_values(clean: &str, corrupted: &str) -> Vec<String> {
    clean
        .lines()
        .zip(corrupted.lines())
        .flat_map(|(cl, co)| cl.split('\t').zip(co.split('\t')))
        .filter(|(c, o)| c != o)
        .map(|(_, o)| o.to_string())
        .collect()
}

const RATE_FIELDS: &[&str] = &[
    "name",
    "email",
    "phone",
    "address",
    "city",
    "ssn",
    "credit-card",
    "iban",
    "username",
    "ip",
    "mac",
    "uuid",
    "company-name",
    "first-name",
    "last-name",
];
const RATE_SEEDS: usize = 50;

// ---------------------------------------------------------------------------
// Acceptance: all levels accepted, invalid rejected
// ---------------------------------------------------------------------------

#[test]
fn all_corruption_levels_accepted() {
    for level in &["low", "mid", "high", "extreme"] {
        let out = seedfaker()
            .args([
                "name",
                "email",
                "-n",
                "3",
                "--seed",
                "corrupt-test",
                "--until",
                "2025",
                "--corrupt",
                level,
            ])
            .output()
            .expect("execute");
        assert!(out.status.success(), "--corrupt {level} should succeed");
        let stdout = String::from_utf8(out.stdout).expect("utf8");
        assert_eq!(stdout.lines().count(), 3, "--corrupt {level} should produce 3 lines");
    }
}

#[test]
fn invalid_corruption_level_rejected() {
    run_fail(&["name", "-n", "1", "--corrupt", "invalid"]);
    run_fail(&["name", "-n", "1", "--corrupt", "minimal"]);
    run_fail(&["name", "-n", "1", "--corrupt", "medium"]);
    run_fail(&["name", "-n", "1", "--corrupt", "heavy"]);
    run_fail(&["name", "-n", "1", "--corrupt", "light"]);
    run_fail(&["name", "-n", "1", "--corrupt", "moderate"]);
    run_fail(&["name", "-n", "1", "--corrupt", "severe"]);
}

// ---------------------------------------------------------------------------
// Inline snapshots: exact byte-for-byte output
// ---------------------------------------------------------------------------

#[test]
fn snapshot_corrupt_extreme() {
    let out = run_ok(&[
        "name",
        "email",
        "phone",
        "--corrupt",
        "extreme",
        "-n",
        "5",
        "--seed",
        "snap2",
        "--until",
        "2025",
    ]);
    assert_eq!(
        out,
        "\tsvetoslavm**k*v@dir.bg\tsvep0H\n\
         Salom\u{e9}ParedesSerrano\tgenevievekumar@cgi.comKU\t+1 (536) 646-9477e3b\n\
         Gustaw Raczynski\t\t+381 68\u{a0} 232\u{a0}3005\n\
         XXXXXXXXXXXXXXXXXXXXmad\t********************il.com\t+\n\
         Darren MartinAJJN\t\t6\n"
    );
}

#[test]
fn snapshot_corrupt_high() {
    let out = run_ok(&[
        "name",
        "email",
        "phone",
        "--corrupt",
        "high",
        "-n",
        "5",
        "--seed",
        "snap2",
        "--until",
        "2025",
    ]);
    assert_eq!(
        out,
        "Villads Gade\t\t959-4***7336\n\
         Salom\u{e9}ParedesSerrano\tgenevievekumar@cgi.comKU\t+1 (536) 646-9477\n\
         Gustaw Raczynski\t\t+381 68 2323005\n\
         Ghazi bin Anas Al-Hamad\tsaadetbayram1988@gmail.com\t+48 502 671 442\n\
         Darren MartinAJJN\t\t649-988-5228\n"
    );
}

// ---------------------------------------------------------------------------
// Rate ordering: higher levels corrupt more
// ---------------------------------------------------------------------------

#[test]
fn corruption_rate_ordering() {
    let levels = ["low", "mid", "high", "extreme"];
    let mut diffs = vec![0usize; levels.len()];

    for seed_num in 0..RATE_SEEDS {
        let seed = format!("ro-{seed_num}");
        let mut clean_args: Vec<&str> = RATE_FIELDS.to_vec();
        clean_args.extend(["-n", "1", "--seed", &seed]);
        let clean = run_ok(&clean_args);

        for (li, level) in levels.iter().enumerate() {
            let mut args: Vec<&str> = RATE_FIELDS.to_vec();
            args.extend(["-n", "1", "--seed", &seed, "--corrupt", level]);
            let corrupted = run_ok(&args);
            diffs[li] += count_diff_fields(&clean, &corrupted);
        }
    }

    for i in 1..levels.len() {
        assert!(
            diffs[i] > diffs[i - 1],
            "corrupt '{}' ({} diffs) must exceed '{}' ({} diffs)",
            levels[i],
            diffs[i],
            levels[i - 1],
            diffs[i - 1]
        );
    }
}

// ---------------------------------------------------------------------------
// Approximate percentages
// ---------------------------------------------------------------------------

#[test]
fn corruption_rate_approximate_percentages() {
    let total = RATE_FIELDS.len() * RATE_SEEDS; // 750
    let expected: &[(&str, f64, f64)] =
        &[("low", 0.01, 0.14), ("mid", 0.06, 0.30), ("high", 0.28, 0.65), ("extreme", 0.75, 1.00)];

    for &(level, min_rate, max_rate) in expected {
        let mut diff_count = 0usize;
        for seed_num in 0..RATE_SEEDS {
            let seed = format!("rp-{seed_num}");
            let mut clean_args: Vec<&str> = RATE_FIELDS.to_vec();
            clean_args.extend(["-n", "1", "--seed", &seed]);
            let clean = run_ok(&clean_args);

            let mut args: Vec<&str> = RATE_FIELDS.to_vec();
            args.extend(["-n", "1", "--seed", &seed, "--corrupt", level]);
            let corrupted = run_ok(&args);
            diff_count += count_diff_fields(&clean, &corrupted);
        }
        let rate = diff_count as f64 / total as f64;
        assert!(
            rate >= min_rate && rate <= max_rate,
            "corrupt '{}': rate {:.3} ({}/{}) outside [{:.3}, {:.3}]",
            level,
            rate,
            diff_count,
            total,
            min_rate,
            max_rate
        );
    }
}

// ---------------------------------------------------------------------------
// Tier boundaries: type pool restrictions
// ---------------------------------------------------------------------------

#[test]
fn corruption_low_only_light_types() {
    let n = "500";
    let seed = "tier-light";
    let clean = run_ok(&["name", "email", "phone", "-n", n, "--seed", seed]);

    for level in &["low"] {
        let corrupted =
            run_ok(&["name", "email", "phone", "--corrupt", level, "-n", n, "--seed", seed]);
        let vals = collect_corrupted_values(&clean, &corrupted);

        assert!(!vals.is_empty(), "corrupt '{}' should produce some corrupted values", level);
        for val in &vals {
            assert!(
                !val.is_empty(),
                "corrupt '{level}' must not blank values (empty = heavy type)"
            );
            let x_prefix = val.chars().take_while(|&c| c == 'X').count();
            assert!(x_prefix < 4, "corrupt '{level}' must not produce X-masking: {val}");
            let star_count = val.chars().filter(|&c| c == '*').count();
            assert!(star_count == 0, "corrupt '{}' must not produce stars: {}", level, val);
            assert!(
                !val.contains("&amp;") && !val.contains("&#39;") && !val.contains("&lt;"),
                "corrupt '{}' must not produce HTML entities: {}",
                level,
                val
            );
        }
    }
}

#[test]
fn corruption_mid_no_heavy_types() {
    let n = "500";
    let seed = "tier-med";
    let clean = run_ok(&["name", "email", "phone", "-n", n, "--seed", seed]);
    let corrupted =
        run_ok(&["name", "email", "phone", "--corrupt", "mid", "-n", n, "--seed", seed]);
    let vals = collect_corrupted_values(&clean, &corrupted);

    assert!(!vals.is_empty(), "mid should produce some corrupted values");
    for val in &vals {
        assert!(!val.is_empty(), "mid must not blank values (empty = heavy type)");
        let x_prefix = val.chars().take_while(|&c| c == 'X').count();
        assert!(x_prefix < 4, "mid must not produce X-masking: {val}");
    }
}

#[test]
fn corruption_extreme_produces_heavy_types() {
    let n = "500";
    let seed = "tier-heavy";
    let clean = run_ok(&["name", "email", "phone", "-n", n, "--seed", seed]);
    let corrupted =
        run_ok(&["name", "email", "phone", "--corrupt", "extreme", "-n", n, "--seed", seed]);

    let has_empty = corrupted.lines().any(|l| l.split('\t').any(|f| f.is_empty()));
    let vals = collect_corrupted_values(&clean, &corrupted);
    let has_x_mask = vals.iter().any(|v| v.chars().take_while(|&c| c == 'X').count() >= 4);
    let has_stars = vals.iter().any(|v| v.contains('*'));

    assert!(has_empty, "extreme should produce empty values");
    assert!(has_x_mask, "extreme should produce X-masking");
    assert!(has_stars, "extreme should produce star corruption");
}

// ---------------------------------------------------------------------------
// Determinism
// ---------------------------------------------------------------------------

#[test]
fn corruption_determinism_all_levels() {
    for level in &["low", "mid", "high", "extreme"] {
        let args = &[
            "name",
            "email",
            "phone",
            "address",
            "--corrupt",
            level,
            "-n",
            "100",
            "--seed",
            "det-all",
        ];
        let a = run_ok(args);
        let b = run_ok(args);
        assert_eq!(a, b, "corrupt '{}' not deterministic", level);
    }
}

#[test]
fn corruption_different_seeds_differ() {
    let a = run_ok(&[
        "name",
        "email",
        "--corrupt",
        "high",
        "-n",
        "50",
        "--seed",
        "seed-a",
        "--until",
        "2025",
    ]);
    let b = run_ok(&[
        "name",
        "email",
        "--corrupt",
        "high",
        "-n",
        "50",
        "--seed",
        "seed-b",
        "--until",
        "2025",
    ]);
    assert_ne!(a, b, "different seeds should differ");
}

// ---------------------------------------------------------------------------
// Structural integrity: record/field counts preserved
// ---------------------------------------------------------------------------

#[test]
fn corruption_preserves_record_count() {
    for level in &["low", "mid", "high", "extreme"] {
        let out = run_ok(&[
            "name",
            "email",
            "phone",
            "--corrupt",
            level,
            "-n",
            "100",
            "--seed",
            "cnt",
            "--until",
            "2025",
        ]);
        assert_eq!(out.lines().count(), 100, "corrupt '{}' changed record count", level);
    }
}

#[test]
fn corruption_preserves_field_count() {
    for level in &["low", "mid", "high", "extreme"] {
        let out = run_ok(&[
            "name",
            "email",
            "phone",
            "--corrupt",
            level,
            "-n",
            "200",
            "--seed",
            "fcnt",
            "--until",
            "2025",
        ]);
        for (i, line) in out.lines().enumerate() {
            assert_eq!(
                line.split('\t').count(),
                3,
                "corrupt '{}' record {} has wrong field count: {:?}",
                level,
                i,
                line
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn corruption_single_field_no_panic() {
    for level in &["low", "mid", "high", "extreme"] {
        let out = run_ok(&[
            "email",
            "--corrupt",
            level,
            "-n",
            "100",
            "--seed",
            "single-f",
            "--until",
            "2025",
        ]);
        assert_eq!(out.lines().count(), 100);
    }
}

// ---------------------------------------------------------------------------
// Corruption with output formats
// ---------------------------------------------------------------------------

#[test]
fn corruption_with_template() {
    let clean = run_ok(&[
        "-t",
        "{{name}} <{{email}}>",
        "-n",
        "200",
        "--seed",
        "tpl-cor",
        "--until",
        "2025",
    ]);
    let corrupted = run_ok(&[
        "-t",
        "{{name}} <{{email}}>",
        "--corrupt",
        "high",
        "-n",
        "200",
        "--seed",
        "tpl-cor",
    ]);
    assert_ne!(clean, corrupted, "template corrupt should differ from clean");
    assert_eq!(corrupted.lines().count(), 200);
}

#[test]
fn corruption_with_csv() {
    let clean = run_ok(&[
        "name",
        "email",
        "--format",
        "csv",
        "--no-header",
        "-n",
        "200",
        "--seed",
        "csv-cor",
    ]);
    let corrupted = run_ok(&[
        "name",
        "email",
        "--format",
        "csv",
        "--no-header",
        "--corrupt",
        "high",
        "-n",
        "200",
        "--seed",
        "csv-cor",
    ]);
    assert_ne!(clean, corrupted, "CSV corrupt should differ");
    assert_eq!(corrupted.lines().count(), 200);
}

#[test]
fn corruption_with_jsonl() {
    let corrupted = run_ok(&[
        "name",
        "email",
        "--format",
        "jsonl",
        "--corrupt",
        "mid",
        "-n",
        "50",
        "--seed",
        "jl-cor",
    ]);
    assert_eq!(corrupted.lines().count(), 50);
    for line in corrupted.lines() {
        let v: serde_json::Value =
            serde_json::from_str(line).expect("corrupt JSONL must be valid JSON");
        assert!(v.get("name").is_some());
        assert!(v.get("email").is_some());
    }
}
