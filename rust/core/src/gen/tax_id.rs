use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "en" => {
            buf.reserve(10);
            ctx.rng.push_digits(buf, 2);
            buf.push('-');
            ctx.rng.push_digits(buf, 7);
        }
        "de" => {
            buf.reserve(12);
            ctx.rng.push_digits(buf, 2);
            buf.push('/');
            ctx.rng.push_digits(buf, 3);
            buf.push('/');
            ctx.rng.push_digits(buf, 5);
        }
        "fr" => {
            buf.reserve(11);
            ctx.rng.push_digits(buf, 3);
            buf.push(' ');
            ctx.rng.push_digits(buf, 3);
            buf.push(' ');
            ctx.rng.push_digits(buf, 3);
        }
        "it" => ctx.rng.push_digits(buf, 11),
        "es" => {
            let l = ctx.rng.charset_string(b"XYZKLM", 1);
            buf.reserve(9);
            buf.push_str(&l);
            ctx.rng.push_digits(buf, 7);
            ctx.rng.push_upper(buf, 1);
        }
        "nl" => {
            buf.reserve(14);
            buf.push_str("NL");
            ctx.rng.push_digits(buf, 9);
            buf.push('B');
            ctx.rng.push_digits(buf, 2);
        }
        "pl" => ctx.rng.push_digits(buf, 10),
        "se" => {
            buf.reserve(12);
            ctx.rng.push_digits(buf, 10);
            buf.push_str("01");
        }
        _ => {
            let n = ctx.rng.urange(9, 11);
            ctx.rng.push_digits(buf, n);
        }
    }
}
