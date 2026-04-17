use crate::ctx::GenContext;

/// Generate a random value from a user-supplied comma-separated list.
///
/// Syntax:
///   `enum:a,b,c`       — uniform random pick
///   `enum:a=3,b=1`     — weighted: `a` appears 3x more often than `b`
///
/// Values must match `[a-zA-Z0-9_-]+`. Weights must be positive integers.
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if ctx.modifier.is_empty() {
        return;
    }
    let entries: Vec<&str> = ctx.modifier.split(',').collect();
    let has_weights = entries.iter().any(|e| e.contains('='));

    if has_weights {
        let mut values = Vec::with_capacity(entries.len());
        let mut weights = Vec::with_capacity(entries.len());
        for entry in &entries {
            if let Some((val, w_str)) = entry.split_once('=') {
                let w: u32 = w_str.parse().unwrap_or(1);
                values.push(val);
                weights.push(w.max(1));
            } else {
                // No weight specified — default to 1
                values.push(entry);
                weights.push(1);
            }
        }
        let total: u32 = weights.iter().sum();
        let roll = ctx.rng.urange(0, total as usize - 1) as u32;
        let mut acc = 0;
        for (i, &w) in weights.iter().enumerate() {
            acc += w;
            if roll < acc {
                buf.push_str(values[i]);
                return;
            }
        }
        if let Some(last) = values.last() {
            buf.push_str(last);
        }
    } else {
        buf.push_str(entries[ctx.rng.urange(0, entries.len() - 1)]);
    }
}

/// Validate enum modifier at parse time. Returns `Err` on invalid syntax.
pub fn validate_enum(modifier: &str) -> Result<(), String> {
    if modifier.is_empty() {
        return Err("enum requires values: enum:a,b,c or enum:yes=3,no=1".into());
    }
    let entries: Vec<&str> = modifier.split(',').collect();
    for entry in &entries {
        let (val, weight) =
            if let Some((v, w)) = entry.split_once('=') { (v, Some(w)) } else { (*entry, None) };
        // Validate value: [a-zA-Z0-9_-]+
        if val.is_empty() {
            return Err(format!("empty value in enum: '{modifier}'"));
        }
        if !val.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-' || b == b'.') {
            return Err(format!(
                "enum value '{val}' contains invalid characters; allowed: a-z, A-Z, 0-9, _, -, ."
            ));
        }
        // Validate weight if present
        if let Some(w) = weight {
            if w.parse::<u32>().is_err() {
                return Err(format!(
                    "invalid weight '{w}' in enum entry '{entry}'; weight must be a positive integer"
                ));
            }
            if w == "0" {
                return Err(format!("weight cannot be 0 in enum entry '{entry}'"));
            }
        }
    }
    Ok(())
}
