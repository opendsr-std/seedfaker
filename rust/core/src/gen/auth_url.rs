use crate::ctx::GenContext;

use super::ascii_lower;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let user_raw = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
    let user = ascii_lower(&mut ctx.rng, user_raw);
    // Must preserve RNG order: alnum(12), then choice(domains)
    buf.reserve(48);
    buf.push_str("https://");
    buf.push_str(&user);
    buf.push(':');
    ctx.rng.push_alnum(buf, 12);
    let domain = ctx.rng.choice(loc.domains);
    buf.push('@');
    buf.push_str(domain);
    buf.push_str("/admin");
}
