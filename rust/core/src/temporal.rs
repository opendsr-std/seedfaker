//! Temporal value parsing: year, date, datetime, epoch seconds.
//!
//! All temporal values are represented as Unix epoch seconds (`i64`).
//! Supported input formats: `2025`, `2025-03-28`, `2025-03-28T14:00`,
//! `2025-03-28T14:00:30`, or raw epoch seconds (`1711630800`).

/// Epoch seconds for `1900-01-01T00:00:00Z`.
pub const DEFAULT_SINCE: i64 = -2_208_988_800;

/// Convert calendar components to Unix epoch seconds (UTC).
/// Uses the proleptic Gregorian calendar. No leap-second handling.
pub fn date_to_epoch(y: i64, m: i64, d: i64, h: i64, min: i64, s: i64) -> i64 {
    // Howard Hinnant's civil_from_days algorithm
    let (mut yr, mut mo) = (y, m);
    if mo <= 2 {
        yr -= 1;
        mo += 9;
    } else {
        mo -= 3;
    }
    let era = if yr >= 0 { yr } else { yr - 399 } / 400;
    let yoe = yr - era * 400;
    let doy = (153 * mo + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    days * 86_400 + h * 3600 + min * 60 + s
}

/// Compute the default `--until` value: start of next year in epoch seconds.
/// Current Unix timestamp — default `--until`. No future dates.
pub fn default_until() -> i64 {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    i64::try_from(secs).unwrap_or(i64::MAX)
}

/// Extract the year from an epoch value.
///
/// Inverse of `date_to_epoch(year, 1, 1, 0, 0, 0)`:
/// `epoch_to_year(date_to_epoch(y, 1, 1, 0, 0, 0)) == y` for any valid year.
pub fn epoch_to_year(epoch: i64) -> i64 {
    // Approximate, then correct by checking boundary
    let approx = 1970 + epoch / 31_557_600;
    if date_to_epoch(approx + 1, 1, 1, 0, 0, 0) <= epoch {
        approx + 1
    } else if date_to_epoch(approx, 1, 1, 0, 0, 0) > epoch {
        approx - 1
    } else {
        approx
    }
}

/// Parse a temporal string into epoch seconds.
///
/// Accepts: year (`2025`), date (`2025-03-28`), datetime (`2025-03-28T14:00`
/// or `2025-03-28T14:00:30`), or raw epoch seconds (`1711630800`).
pub fn parse(s: &str) -> Result<i64, String> {
    let s = s.trim();

    // Try raw integer first
    if let Ok(v) = s.parse::<i64>() {
        if v > 100_000 {
            // Epoch seconds
            return Ok(v);
        }
        if (1..=9999).contains(&v) {
            // Year
            return Ok(date_to_epoch(v, 1, 1, 0, 0, 0));
        }
        return Err(format!(
            "ambiguous temporal value: {v}; use a year (1-9999) or epoch seconds (>100000)"
        ));
    }

    // YYYY-MM-DD or YYYY-MM-DDTHH:MM or YYYY-MM-DDTHH:MM:SS
    let parts: Vec<&str> = s.splitn(2, 'T').collect();
    let date_part = parts[0];
    let time_part = if parts.len() > 1 { parts[1] } else { "" };

    let date_segs: Vec<&str> = date_part.split('-').collect();
    if date_segs.len() != 3 {
        return Err(format!(
            "invalid temporal format '{s}'; expected: YYYY, YYYY-MM-DD, YYYY-MM-DDTHH:MM, YYYY-MM-DDTHH:MM:SS, or epoch seconds"
        ));
    }

    let y = date_segs[0].parse::<i64>().map_err(|_| format!("invalid year in '{s}'"))?;
    let m = date_segs[1].parse::<i64>().map_err(|_| format!("invalid month in '{s}'"))?;
    let d = date_segs[2].parse::<i64>().map_err(|_| format!("invalid day in '{s}'"))?;

    if !(1..=12).contains(&m) {
        return Err(format!("month out of range in '{s}'"));
    }
    if !(1..=31).contains(&d) {
        return Err(format!("day out of range in '{s}'"));
    }

    if time_part.is_empty() {
        return Ok(date_to_epoch(y, m, d, 0, 0, 0));
    }

    let time_segs: Vec<&str> = time_part.split(':').collect();
    let h = time_segs
        .first()
        .and_then(|s| s.parse::<i64>().ok())
        .ok_or_else(|| format!("invalid hour in '{s}'"))?;
    let min = time_segs
        .get(1)
        .and_then(|s| s.parse::<i64>().ok())
        .ok_or_else(|| format!("invalid minute in '{s}'"))?;
    // Seconds are optional in HH:MM format (ISO 8601); default to 0.
    let sec = match time_segs.get(2) {
        Some(s) => s.parse::<i64>().map_err(|_| format!("invalid seconds in '{s}'"))?,
        None => 0,
    };

    if !(0..=23).contains(&h) {
        return Err(format!("hour out of range in '{s}'"));
    }
    if !(0..=59).contains(&min) {
        return Err(format!("minute out of range in '{s}'"));
    }

    Ok(date_to_epoch(y, m, d, h, min, sec))
}

/// Parse a temporal string for use as an upper bound ("until").
///
/// Year → end of year (start of next). Date → end of day (start of next).
/// Datetime and epoch → same as `parse()`.
pub fn parse_until(s: &str) -> Result<i64, String> {
    let s = s.trim();
    if let Ok(v) = s.parse::<i64>() {
        if v > 100_000 {
            return Ok(v);
        }
        if (1..=9999).contains(&v) {
            return Ok(date_to_epoch(v + 1, 1, 1, 0, 0, 0));
        }
    }
    // Date without time → end of day
    if !s.contains('T') && s.contains('-') {
        let e = parse(s)?;
        return Ok(e + 86_400);
    }
    parse(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_year() {
        let e = parse("2025").unwrap();
        assert_eq!(epoch_to_year(e), 2025);
    }

    #[test]
    fn parse_date() {
        let e = parse("2025-01-01").unwrap();
        assert_eq!(e, date_to_epoch(2025, 1, 1, 0, 0, 0));
    }

    #[test]
    fn parse_datetime() {
        let e = parse("2025-03-28T14:00").unwrap();
        assert_eq!(e, date_to_epoch(2025, 3, 28, 14, 0, 0));
    }

    #[test]
    fn parse_datetime_seconds() {
        let e = parse("2025-03-28T14:00:30").unwrap();
        assert_eq!(e, date_to_epoch(2025, 3, 28, 14, 0, 30));
    }

    #[test]
    fn parse_epoch() {
        assert_eq!(parse("1711630800").unwrap(), 1_711_630_800);
    }

    #[test]
    fn parse_invalid() {
        assert!(parse("abc").is_err());
        assert!(parse("2025-13-01").is_err());
        assert!(parse("2025-01-32").is_err());
    }

    #[test]
    fn default_since_is_1900() {
        assert_eq!(epoch_to_year(DEFAULT_SINCE), 1900);
    }

    // --- parse_until edge cases ---

    #[test]
    fn until_year_is_exclusive() {
        let e = parse_until("2025").unwrap();
        assert_eq!(e, date_to_epoch(2026, 1, 1, 0, 0, 0));
        assert_eq!(epoch_to_year(e - 1), 2025);
    }

    #[test]
    fn until_date_is_end_of_day() {
        let e = parse_until("2025-03-28").unwrap();
        let start = parse("2025-03-28").unwrap();
        assert_eq!(e, start + 86_400);
    }

    #[test]
    fn until_datetime_is_exact() {
        let e = parse_until("2025-03-28T16:00").unwrap();
        assert_eq!(e, date_to_epoch(2025, 3, 28, 16, 0, 0));
    }

    #[test]
    fn until_epoch_is_exact() {
        assert_eq!(parse_until("1711638000").unwrap(), 1_711_638_000);
    }

    // --- roundtrip edge cases ---

    #[test]
    fn roundtrip_year_boundaries() {
        for y in [1900, 1970, 1999, 2000, 2001, 2024, 2025, 2038, 2100] {
            let e = parse(&y.to_string()).unwrap();
            assert_eq!(epoch_to_year(e), y, "roundtrip failed for year {y}");
        }
    }

    #[test]
    fn roundtrip_dates() {
        let cases = [
            ("2025-01-01", 2025, 1, 1),
            ("2000-02-29", 2000, 2, 29),  // leap year
            ("1970-01-01", 1970, 1, 1),   // epoch zero
            ("2025-12-31", 2025, 12, 31), // end of year
        ];
        for (input, y, m, d) in cases {
            let e = parse(input).unwrap();
            assert_eq!(e, date_to_epoch(y, m, d, 0, 0, 0), "parse failed for {input}");
        }
    }

    #[test]
    fn midnight_vs_2359() {
        let midnight = parse("2025-03-28T00:00").unwrap();
        let eod = parse("2025-03-28T23:59").unwrap();
        assert_eq!(eod - midnight, 23 * 3600 + 59 * 60);
    }

    #[test]
    fn whitespace_trimmed() {
        assert_eq!(parse("  2025  ").unwrap(), parse("2025").unwrap());
        assert_eq!(parse(" 2025-03-28 ").unwrap(), parse("2025-03-28").unwrap());
    }

    #[test]
    fn invalid_time_components() {
        assert!(parse("2025-01-01T25:00").is_err()); // hour 25
        assert!(parse("2025-01-01T12:60").is_err()); // minute 60
    }

    #[test]
    fn ambiguous_small_number() {
        assert!(parse("50000").is_err()); // between year and epoch — ambiguous
    }
}
