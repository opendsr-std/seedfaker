use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "ru" => {
            buf.reserve(11);
            ctx.rng.push_digits(buf, 4);
            buf.push(' ');
            ctx.rng.push_digits(buf, 6);
        }
        "uk" => ctx.rng.push_digits(buf, 9),
        "be" => {
            buf.reserve(9);
            ctx.rng.push_upper(buf, 2);
            ctx.rng.push_digits(buf, 7);
        }
        _ => {
            buf.reserve(10);
            ctx.rng.push_digits(buf, 4);
            buf.push(' ');
            ctx.rng.push_digits(buf, 6);
        }
    }
}
