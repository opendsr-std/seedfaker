/// Date, birthdate, timestamp: default values, format modifiers,
/// year flags, weighted birthdate distribution.
mod common;
use common::run_ok;

// ---------------------------------------------------------------------------
// Date: default format and range
// ---------------------------------------------------------------------------

#[test]
fn date_default_iso_format_and_year_range() {
    let out = run_ok(&["date", "-n", "100", "--seed", "dt", "--until", "2030"]);
    for line in out.lines() {
        assert_eq!(line.len(), 10, "YYYY-MM-DD: {line}");
        assert_eq!(&line[4..5], "-");
        assert_eq!(&line[7..8], "-");
        let year: i64 = line[..4].parse().expect("year");
        assert!((1900..=2030).contains(&year), "year {year} outside [1900, 2030]");
        let month: i64 = line[5..7].parse().expect("month");
        assert!((1..=12).contains(&month), "month {month}: {line}");
        let day: i64 = line[8..10].parse().expect("day");
        assert!((1..=28).contains(&day), "day {day}: {line}");
    }
}

// ---------------------------------------------------------------------------
// Date: format modifiers
// ---------------------------------------------------------------------------

#[test]
fn date_us_format() {
    let out = run_ok(&["date:us", "-n", "20", "--seed", "dt", "--until", "2025"]);
    for line in out.lines() {
        let parts: Vec<&str> = line.split('/').collect();
        assert_eq!(parts.len(), 3, "MM/DD/YYYY: {line}");
        let month: i64 = parts[0].parse().expect("month");
        let day: i64 = parts[1].parse().expect("day");
        let year: i64 = parts[2].parse().expect("year");
        assert!((1..=12).contains(&month));
        assert!((1..=28).contains(&day));
        assert!((1900..=2030).contains(&year));
    }
}

#[test]
fn date_eu_format() {
    let out = run_ok(&["date:eu", "-n", "20", "--seed", "dt", "--until", "2025"]);
    for line in out.lines() {
        let parts: Vec<&str> = line.split('.').collect();
        assert_eq!(parts.len(), 3, "DD.MM.YYYY: {line}");
        let day: i64 = parts[0].parse().expect("day");
        let month: i64 = parts[1].parse().expect("month");
        let year: i64 = parts[2].parse().expect("year");
        assert!((1..=28).contains(&day));
        assert!((1..=12).contains(&month));
        assert!((1900..=2030).contains(&year));
    }
}

// ---------------------------------------------------------------------------
// Date: year flags
// ---------------------------------------------------------------------------

#[test]
fn since_to_flags() {
    let out = run_ok(&["date", "-n", "100", "--seed", "yr", "--since", "2030", "--until", "2035"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((2030..=2035).contains(&year), "date should be in 2030-2035: {line}");
    }
}

#[test]
fn since_only_uses_default_to() {
    // Pin --until because default is system year (may be < 2030)
    let out = run_ok(&["date", "-n", "100", "--seed", "yr", "--since", "2030", "--until", "2038"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!(year >= 2030, "date >= 2030: {line}");
        assert!(year <= 2038, "date <= until: {line}");
    }
}

#[test]
fn until_only_uses_default_from() {
    let out = run_ok(&["date", "-n", "100", "--seed", "yr", "--until", "1980"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!(year >= 1900, "date >= DEFAULT_YEAR_FROM: {line}");
        assert!(year <= 1980, "date <= 1980: {line}");
    }
}

// ---------------------------------------------------------------------------
// Birthdate: default weighted distribution
// ---------------------------------------------------------------------------

#[test]
fn birthdate_default_weighted_distribution() {
    let out = run_ok(&["birthdate", "-n", "1000", "--seed", "bd-default", "--until", "2026"]);
    let mut in_range = 0;
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!((1900..=2026).contains(&year), "birthdate year {year} outside [1900, 2026]");
        // young adults born ~1981-2008
        if year >= 1980 && year <= 2010 {
            in_range += 1;
        }
    }
    assert!(in_range > 500, "young adults should dominate: {in_range}/1000");
}

#[test]
fn birthdate_weighted_has_young_and_old() {
    let out =
        run_ok(&["birthdate", "-n", "5000", "--seed", "age", "--since", "1920", "--until", "2025"]);
    let mut young = 0;
    let mut old = 0;
    for line in out.lines() {
        let year: i64 = line[..4].parse().unwrap_or(0);
        if (1990..=2007).contains(&year) {
            young += 1;
        }
        if (1925..=1949).contains(&year) {
            old += 1;
        }
    }
    assert!(young > 2000, "young (18-35) should be ~50% but got {young}/5000");
    assert!(old > 20, "old (76-100) should be >0.5% but got {old}/5000");
    assert!(young > old * 10, "young should greatly outnumber old: {young} vs {old}");
}

#[test]
fn birthdate_weighted_all_in_range() {
    let out =
        run_ok(&["birthdate", "-n", "2000", "--seed", "bwb", "--since", "1950", "--until", "2020"]);
    for line in out.lines() {
        let year: i64 = line[..4].parse().expect("year");
        assert!(
            (1950..=2020).contains(&year),
            "weighted birthdate year {year} outside [1950, 2020]"
        );
    }
}

// ---------------------------------------------------------------------------
// Birthdate: format modifiers
// ---------------------------------------------------------------------------

#[test]
fn birthdate_us_eu_formats() {
    let us = run_ok(&["birthdate:us", "-n", "5", "--seed", "dob", "--until", "2025"]);
    for line in us.lines() {
        let parts: Vec<&str> = line.split('/').collect();
        assert_eq!(parts.len(), 3, "birthdate:us MM/DD/YYYY: {line}");
    }
    let eu = run_ok(&["birthdate:eu", "-n", "5", "--seed", "dob", "--until", "2025"]);
    for line in eu.lines() {
        let parts: Vec<&str> = line.split('.').collect();
        assert_eq!(parts.len(), 3, "birthdate:eu DD.MM.YYYY: {line}");
    }
}

// ---------------------------------------------------------------------------
// Timestamp: default format and range
// ---------------------------------------------------------------------------

#[test]
fn timestamp_iso_default() {
    let out = run_ok(&["timestamp", "-n", "50", "--seed", "ts", "--until", "2030"]);
    for line in out.lines() {
        assert!(line.contains('T'), "ISO should contain T: {line}");
        assert!(line.ends_with('Z'), "default TZ is Z: {line}");
        assert_eq!(line.len(), 20, "ISO timestamp 20 chars: {line}");
        let year: i64 = line[..4].parse().expect("year");
        assert!((1900..=2030).contains(&year), "timestamp year {year} outside [1900, 2030]");
    }
}

#[test]
fn timestamp_unix_default_range() {
    let out = run_ok(&["timestamp:unix", "-n", "50", "--seed", "ts", "--until", "2025"]);
    for line in out.lines() {
        let v: i64 = line.parse().expect("should be numeric");
        assert!(v >= 0, "unix ts >= 0: {v}");
        // System year default (~2026), allow some margin
        let max_epoch = (2030 - 1970) * 31_557_600;
        assert!(v <= max_epoch, "unix ts <= 2039 epoch: {v}");
    }
}

#[test]
fn timestamp_log_format() {
    let out = run_ok(&["timestamp:log", "-n", "20", "--seed", "ts", "--until", "2025"]);
    for line in out.lines() {
        assert!(line.contains('/'), "log format has /: {line}");
        assert!(line.contains('+') || line.contains('-'), "log format has TZ: {line}");
    }
}
