/// Parse timezone offset string into total minutes.
/// Accepts: "Z", "+HHMM", "-HHMM", "+HH:MM", "-HH:MM"
pub fn parse(s: &str) -> Result<i32, String> {
    if s == "Z" || s == "z" {
        return Ok(0);
    }

    let sign = match s.as_bytes().first() {
        Some(b'+') => 1,
        Some(b'-') => -1,
        _ => return Err(format!("invalid timezone offset '{s}'; expected +HHMM, -HH:MM, or Z")),
    };

    let body = &s[1..];
    let (hours, minutes) = if body.len() == 4 && body.is_ascii() {
        let h = body[..2].parse::<i32>().map_err(|_| format!("invalid timezone offset '{s}'"))?;
        let m = body[2..].parse::<i32>().map_err(|_| format!("invalid timezone offset '{s}'"))?;
        (h, m)
    } else if body.len() == 5 && body.as_bytes()[2] == b':' {
        let h = body[..2].parse::<i32>().map_err(|_| format!("invalid timezone offset '{s}'"))?;
        let m = body[3..].parse::<i32>().map_err(|_| format!("invalid timezone offset '{s}'"))?;
        (h, m)
    } else {
        return Err(format!("invalid timezone offset '{s}'; expected +HHMM, -HH:MM, or Z"));
    };

    if hours > 99 {
        return Err(format!("invalid timezone offset '{s}'; hours must be 00-99"));
    }
    if minutes > 59 {
        return Err(format!("invalid timezone offset '{s}'; minutes must be 00-59"));
    }

    Ok(sign * (hours * 60 + minutes))
}
