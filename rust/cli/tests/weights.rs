/// Weight mechanics: enum weighted distribution, locale weights.
mod common;
use common::run_ok;

// ---------------------------------------------------------------------------
// Enum weights
// ---------------------------------------------------------------------------

#[test]
fn enum_weights_distribution() {
    let out = run_ok(&["enum:a=9,b=1", "-n", "1000", "--seed", "ew", "--until", "2025"]);
    let a_count = out.lines().filter(|l| *l == "a").count();
    let b_count = out.lines().filter(|l| *l == "b").count();
    assert!(a_count > 800, "a should be ~90% but got {a_count}/1000");
    assert!(b_count > 50, "b should be ~10% but got {b_count}/1000");
}

#[test]
fn enum_weights_all_equal() {
    let weighted = run_ok(&["enum:x=1,y=1,z=1", "-n", "100", "--seed", "eq", "--until", "2025"]);
    let plain = run_ok(&["enum:x,y,z", "-n", "100", "--seed", "eq", "--until", "2025"]);
    for line in weighted.lines() {
        assert!(["x", "y", "z"].contains(&line), "unexpected value: {line}");
    }
    for line in plain.lines() {
        assert!(["x", "y", "z"].contains(&line), "unexpected value: {line}");
    }
}

#[test]
fn enum_weights_default_is_one() {
    let out = run_ok(&["enum:a=5,b", "-n", "600", "--seed", "dw", "--until", "2025"]);
    let a_count = out.lines().filter(|l| *l == "a").count();
    let b_count = out.lines().filter(|l| *l == "b").count();
    assert!(a_count > 400, "a should be ~83% but got {a_count}/600");
    assert!(b_count > 50, "b should be ~17% but got {b_count}/600");
}

// ---------------------------------------------------------------------------
// Locale weights
// ---------------------------------------------------------------------------

#[test]
fn locale_weights_single_locale() {
    let weighted =
        run_ok(&["name", "--locale", "en=1", "-n", "5", "--seed", "s1", "--until", "2025"]);
    let plain = run_ok(&["name", "--locale", "en", "-n", "5", "--seed", "s1", "--until", "2025"]);
    assert_eq!(weighted, plain, "en=1 should behave identically to en");
}
