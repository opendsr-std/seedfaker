use std::fmt::Write;

use crate::ctx::GenContext;

use super::ascii_lower;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let opts: [(&str, i64); 4] =
        [("postgresql", 5432), ("mysql", 3306), ("mongodb", 27017), ("redis", 6379)];
    let (proto, port) = ctx.rng.choice(&opts);
    let user_raw = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
    let user = ascii_lower(&mut ctx.rng, user_raw);
    let pass = ctx.rng.alnum(14);
    let host = ctx.rng.range(1, 99);
    let domain = ctx.rng.choice(loc.domains);
    let dbs = ["production", "staging", "analytics", "auth", "main"];
    let db = dbs[ctx.rng.urange(0, dbs.len() - 1)];
    // proto(max 10) + :// + user(~10) + : + pass(14) + @db- + host(2) + .internal. + domain(~20) + : + port(5) + / + db(10) ~ 90
    buf.reserve(96);
    let _ = write!(buf, "{proto}://{user}:{pass}@db-{host}.internal.{domain}:{port}/{db}");
}
