use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let p = if ctx.rng.maybe(0.5) { "vc_prod_" } else { "vc_test_" };
    buf.reserve(8 + 32);
    buf.push_str(p);
    ctx.rng.push_alnum(buf, 32);
}
