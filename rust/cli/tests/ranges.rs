/// Range mechanics: parsing, bounds, partial fill, resolution priority,
/// template/config integration, error rejection.
mod common;
use common::{run_fail, run_ok, tempfile};

// ---------------------------------------------------------------------------
// Integer ranges
// ---------------------------------------------------------------------------

#[test]
fn integer_range_values_in_bounds() {
    let out = run_ok(&["integer:1..100", "-n", "200", "--seed", "rng", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((1..=100).contains(&v), "value {v} out of range 1..100");
    }
}

#[test]
fn integer_range_short_form_to() {
    let out = run_ok(&["integer:..50", "-n", "200", "--seed", "rng", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((0..=50).contains(&v), "value {v} out of range 0..50");
    }
}

#[test]
fn integer_range_short_form_from() {
    let out = run_ok(&["integer:999990..", "-n", "50", "--seed", "rng", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((999_990..=999_999).contains(&v), "value {v} out of range 999990..999999");
    }
}

#[test]
fn integer_negative_range() {
    let out = run_ok(&["integer:-100..100", "-n", "200", "--seed", "neg", "--until", "2025"]);
    let mut has_neg = false;
    let mut has_pos = false;
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((-100..=100).contains(&v), "value {v} out of range -100..100");
        if v < 0 {
            has_neg = true;
        }
        if v > 0 {
            has_pos = true;
        }
    }
    assert!(has_neg, "should have negative values");
    assert!(has_pos, "should have positive values");
}

#[test]
fn integer_large_range() {
    let out = run_ok(&["integer:0..1000000000", "-n", "10", "--seed", "big", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((0..=1_000_000_000).contains(&v), "value {v} out of range");
    }
}

#[test]
fn integer_default_range() {
    let out = run_ok(&["integer", "-n", "200", "--seed", "idef", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((0..=999_999).contains(&v), "default integer {v} outside [0, 999999]");
    }
}

#[test]
fn integer_partial_fills_defaults() {
    let out = run_ok(&["integer:500..", "-n", "100", "--seed", "ipf", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("int");
        assert!((500..=999_999).contains(&v), "value {v} out of 500..999999");
    }
    let out = run_ok(&["integer:..100", "-n", "100", "--seed", "ipf2", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("int");
        assert!((0..=100).contains(&v), "value {v} out of 0..100");
    }
}

// ---------------------------------------------------------------------------
// Amount ranges
// ---------------------------------------------------------------------------

#[test]
fn amount_range_plain() {
    let out = run_ok(&["amount:100..500:plain", "-n", "100", "--seed", "amt", "--until", "2025"]);
    for line in out.lines() {
        let v: f64 = line.parse().expect("should be number");
        assert!(v >= 100.0 && v < 501.0, "amount {v} out of range 100..500");
    }
}

#[test]
fn amount_negative_formatting() {
    let out = run_ok(&["amount:-1000..1000:usd", "-n", "50", "--seed", "neg", "--until", "2025"]);
    for line in out.lines() {
        if line.contains('-') {
            assert!(line.starts_with("-$"), "negative amount should start with -$: {line}");
        }
    }
}

#[test]
fn amount_default_range() {
    let out = run_ok(&["amount:plain", "-n", "100", "--seed", "amdef", "--until", "2025"]);
    for line in out.lines() {
        let v: f64 = line.parse().expect("should be number");
        assert!(v >= 0.0, "default amount >= 0: {v}");
        assert!(v < 1_000_000.0, "default amount < 1M: {v}");
    }
}

// ---------------------------------------------------------------------------
// Date/timestamp ranges (range mechanics only, not format)
// ---------------------------------------------------------------------------

#[test]
fn date_range_years() {
    let out = run_ok(&["date:2020..2022", "-n", "100", "--seed", "dt", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year), "year {year} out of range 2020..2022");
    }
}

#[test]
fn date_range_with_eu_modifier() {
    let out = run_ok(&["date:2020..2022:eu", "-n", "50", "--seed", "dt", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[6..].parse().expect("EU year");
        assert!((2020..=2022).contains(&year), "year {year} out of range");
    }
}

#[test]
fn date_range_with_us_modifier() {
    let out = run_ok(&["date:2020..2022:us", "-n", "50", "--seed", "dtus", "--until", "2025"]);
    for line in out.lines() {
        let parts: Vec<&str> = line.split('/').collect();
        let year: i64 = parts[2].parse().expect("year");
        assert!((2020..=2022).contains(&year), "year {year} out of range");
    }
}

#[test]
fn birthdate_range_uniform() {
    let out = run_ok(&["birthdate:1990..2005", "-n", "100", "--seed", "bd", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((1990..=2005).contains(&year), "year {year} out of range 1990..2005");
    }
}

#[test]
fn birthdate_range_with_ctx_strict() {
    let out = run_ok(&[
        "birthdate:1990..2000",
        "--ctx",
        "strict",
        "--locale",
        "en",
        "-n",
        "50",
        "--seed",
        "bdc",
    ]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((1990..=2000).contains(&year), "ctx strict year {year} out of range");
    }
}

#[test]
fn timestamp_range_log() {
    let out = run_ok(&["timestamp:2024..2025:log", "-n", "50", "--seed", "ts", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[7..11].parse().expect("year");
        // epoch_to_parts is approximate; boundary year±1 tolerated
        assert!((2024..=2026).contains(&year), "timestamp year {year} out of range");
    }
}

#[test]
fn timestamp_range_iso() {
    let out = run_ok(&["timestamp:2024..2025", "-n", "50", "--seed", "tsiso", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2024..=2026).contains(&year), "timestamp year {year} out of range");
    }
}

#[test]
fn timestamp_range_unix() {
    let out =
        run_ok(&["timestamp:2024..2025:unix", "-n", "50", "--seed", "tsunix", "--until", "2025"]);
    let epoch_2024 = (2024 - 1970) * 31_557_600;
    let epoch_2026 = (2026 - 1970) * 31_557_600;
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be numeric");
        assert!(v >= epoch_2024 - 100_000, "unix ts {v} < 2024 epoch");
        assert!(v <= epoch_2026 + 100_000, "unix ts {v} > 2026 epoch");
    }
}

// ---------------------------------------------------------------------------
// Range priority
// ---------------------------------------------------------------------------

#[test]
fn range_inline_overrides_global() {
    let out = run_ok(&[
        "date:2020..2022",
        "-n",
        "50",
        "--seed",
        "prio",
        "--since",
        "1990",
        "--until",
        "2030",
    ]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year), "inline range should win: {year}");
    }
}

#[test]
fn range_partial_fills_from_global_left() {
    let out = run_ok(&["date:..2022", "-n", "50", "--seed", "prio", "--since", "2020"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2020..=2022).contains(&year), "partial ..TO should fill from global: {year}");
    }
}

#[test]
fn range_partial_fills_from_global_right() {
    let out = run_ok(&["date:2020..", "-n", "100", "--seed", "prio", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2020..=2025).contains(&year), "partial FROM.. should fill from global: {year}");
    }
}

// ---------------------------------------------------------------------------
// Range in template and config
// ---------------------------------------------------------------------------

#[test]
fn range_in_template() {
    let out = run_ok(&["-t", "{{integer:1..10}}", "-n", "50", "--seed", "tpl", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((1..=10).contains(&v), "template range value {v} out of 1..10");
    }
}

#[test]
fn date_range_in_template() {
    let out =
        run_ok(&["-t", "{{date:2024..2025}}", "-n", "50", "--seed", "dtpl", "--until", "2025"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2024..=2025).contains(&year), "template date year {year} out of range");
    }
}

#[test]
fn range_in_config_yaml() {
    let path = tempfile("range");
    std::fs::write(&path, "columns:\n  n: integer:1..50\noptions:\n  seed: cfg\n  n: 100\n")
        .expect("write config");
    let out = run_ok(&["run", path.to_str().unwrap()]);
    std::fs::remove_file(&path).ok();
    for line in out.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        let v: i64 = line.trim().parse().expect("should be integer");
        assert!((1..=50).contains(&v), "config range value {v} out of 1..50");
    }
}

// ---------------------------------------------------------------------------
// Range errors
// ---------------------------------------------------------------------------

#[test]
fn range_unsupported_field() {
    run_fail(&["phone:1..100", "-n", "1", "--seed", "x", "--until", "2025"]);
}

#[test]
fn range_invalid_order() {
    run_fail(&["date:2025..2020", "-n", "1", "--seed", "x", "--until", "2025"]);
}

#[test]
fn range_invalid_bound() {
    run_fail(&["integer:abc..def", "-n", "1", "--seed", "x", "--until", "2025"]);
}

#[test]
fn range_resolved_invalid() {
    run_fail(&["integer:..-1", "-n", "1", "--seed", "x", "--until", "2025"]);
}

// ---------------------------------------------------------------------------
// Zipf distribution
// ---------------------------------------------------------------------------

#[test]
fn zipf_values_in_bounds() {
    let out = run_ok(&["integer:1..100:zipf", "-n", "500", "--seed", "z1", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((1..=100).contains(&v), "zipf value {v} out of range 1..100");
    }
}

#[test]
fn zipf_custom_exponent_values_in_bounds() {
    let out =
        run_ok(&["integer:1..1000:zipf=0.8", "-n", "500", "--seed", "z08", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be integer");
        assert!((1..=1000).contains(&v), "zipf=0.8 value {v} out of range 1..1000");
    }
}

#[test]
fn zipf_skew_low_values_dominate() {
    let out = run_ok(&["integer:1..1000:zipf", "-n", "1000", "--seed", "zskew", "--until", "2025"]);
    let values: Vec<i64> = out.lines().map(|l| l.parse().unwrap()).collect();
    let below_10 = values.iter().filter(|&&v| v <= 10).count();
    // With s=1.0 and n=1000, ~34% of values should be in [1,10].
    assert!(below_10 > 150, "zipf s=1.0: expected >150 values ≤10, got {below_10}");
}

#[test]
fn zipf_high_exponent_concentrates() {
    let out = run_ok(&["integer:1..100:zipf=2", "-n", "500", "--seed", "z2", "--until", "2025"]);
    let values: Vec<i64> = out.lines().map(|l| l.parse().unwrap()).collect();
    let is_one = values.iter().filter(|&&v| v == 1).count();
    // With s=2.0, rank 1 gets ~61% of probability mass.
    assert!(is_one > 200, "zipf s=2.0: expected >200 values =1, got {is_one}");
}

#[test]
fn zipf_deterministic() {
    let out1 = run_ok(&["integer:1..100:zipf=0.8", "-n", "50", "--seed", "det", "--until", "2025"]);
    let out2 = run_ok(&["integer:1..100:zipf=0.8", "-n", "50", "--seed", "det", "--until", "2025"]);
    assert_eq!(out1, out2, "zipf must be deterministic with same seed");
}

#[test]
fn zipf_with_omit() {
    let out =
        run_ok(&["integer:1..100:zipf:omit=50", "-n", "100", "--seed", "zomit", "--until", "2025"]);
    let empty = out.lines().filter(|l| l.is_empty()).count();
    let filled: Vec<i64> =
        out.lines().filter(|l| !l.is_empty()).map(|l| l.parse().unwrap()).collect();
    assert!(empty > 20, "omit=50 should produce >20 empty lines, got {empty}");
    for v in &filled {
        assert!((1..=100).contains(v), "zipf+omit value {v} out of range");
    }
}

#[test]
fn zipf_requires_range() {
    run_fail(&["integer:zipf", "-n", "1", "--seed", "x", "--until", "2025"]);
}

#[test]
fn zipf_invalid_exponent() {
    run_fail(&["integer:1..100:zipf=0", "-n", "1", "--seed", "x", "--until", "2025"]);
    run_fail(&["integer:1..100:zipf=-1", "-n", "1", "--seed", "x", "--until", "2025"]);
}

#[test]
fn zipf_float_range() {
    let out = run_ok(&["float:1..100:zipf", "-n", "200", "--seed", "zf", "--until", "2025"]);
    for line in out.lines() {
        let v: f64 = line.parse().expect("should be float");
        assert!(v >= 1.0 && v < 101.0, "zipf float {v} out of range");
    }
}

#[test]
fn zipf_amount_range() {
    let out = run_ok(&["amount:1..100:zipf:plain", "-n", "200", "--seed", "za", "--until", "2025"]);
    for line in out.lines() {
        let v: f64 = line.parse().expect("should be amount");
        assert!(v >= 1.0 && v < 101.0, "zipf amount {v} out of range");
    }
}
