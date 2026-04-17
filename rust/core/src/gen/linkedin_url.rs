use crate::ctx::GenContext;

use super::ascii_lower;
use super::helpers::handle::unique_tag;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let (first, last) = if let Some(id) = ctx.identity {
        (id.first_ascii.clone(), id.last_ascii.clone())
    } else {
        let loc = ctx.locale();
        let f_raw = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
        let l_raw = loc.last_names[ctx.rng.urange(0, loc.last_names.len() - 1)];
        (ascii_lower(&mut ctx.rng, f_raw), ascii_lower(&mut ctx.rng, l_raw))
    };
    let tag = unique_tag(ctx.rng.record(), 0x0E01);
    buf.push_str("https://linkedin.com/in/");
    buf.push_str(&first);
    buf.push('-');
    buf.push_str(&last);
    buf.push('-');
    // 6 hex digits from tag (bijective, unique per record)
    let hex = format!("{:06x}", tag & 0x00FF_FFFF);
    buf.push_str(&hex);
}
