use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "en" => {
            let n = ctx.rng.urange(8, 17);
            ctx.rng.push_digits(buf, n);
        }
        "ie" => {
            buf.reserve(17);
            ctx.rng.push_digits(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 2);
            buf.push(' ');
            ctx.rng.push_digits(buf, 8);
        }
        "de" | "fr" | "it" | "es" | "nl" | "pt" => {
            let n = ctx.rng.urange(12, 20);
            buf.reserve(2 + n);
            ctx.rng.push_upper(buf, 2);
            ctx.rng.push_digits(buf, n);
        }
        _ => {
            let n = ctx.rng.urange(8, 16);
            ctx.rng.push_digits(buf, n);
        }
    }
}
