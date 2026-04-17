use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let postal = if let Some(id) = ctx.identity {
        &*id.postal
    } else {
        let loc = ctx.pick_locale();
        let i = ctx.rng.urange(0, loc.cities.len() - 1);
        loc.cities[i].postal
    };
    // Replace last 2 digits with tag-derived values for uniqueness,
    // preserving the postal format prefix (area code / district).
    let bytes = postal.as_bytes();
    let mut last_digit_pos = bytes.len();
    let mut digits_found = 0;
    for i in (0..bytes.len()).rev() {
        if bytes[i].is_ascii_digit() {
            last_digit_pos = i;
            digits_found += 1;
            if digits_found >= 2 {
                break;
            }
        } else if digits_found > 0 {
            break; // non-digit between digit groups, stop
        }
    }
    if digits_found >= 2 {
        buf.push_str(&postal[..last_digit_pos]);
        let tag = ctx.rng.record().wrapping_mul(0x9E37_79B9);
        buf.push((b'0' + (tag % 10) as u8) as char);
        buf.push((b'0' + ((tag / 10) % 10) as u8) as char);
        // Append any trailing non-digit chars (e.g. UK "XX## #XX" format)
        let after = last_digit_pos + 2;
        if after < postal.len() {
            buf.push_str(&postal[after..]);
        }
    } else {
        // No digit positions to vary — use as-is
        buf.push_str(postal);
    }
}
