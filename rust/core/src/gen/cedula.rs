use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "uy" => {
            let n = ctx.rng.range(1_000_000, 9_999_999);
            buf.reserve(11);
            let _ = write!(
                buf,
                "{}.{}.{}-{}",
                n / 1_000_000,
                (n / 1000) % 1000,
                n % 1000,
                ctx.rng.range(0, 9)
            );
        }
        "co" => {
            let n = ctx.rng.urange(8, 10);
            ctx.rng.push_digits(buf, n);
        }
        "ec" => ctx.rng.push_digits(buf, 10),
        "py" => {
            let n = ctx.rng.range(1_000_000, 9_999_999);
            buf.reserve(9);
            let _ = write!(buf, "{}.{}.{}", n / 1_000_000, (n / 1000) % 1000, n % 1000);
        }
        "cr" => {
            buf.reserve(11);
            let _ = write!(buf, "{}-", ctx.rng.range(1, 9));
            ctx.rng.push_digits(buf, 4);
            buf.push('-');
            ctx.rng.push_digits(buf, 4);
        }
        "ve" => {
            buf.reserve(11);
            let _ = write!(buf, "V-{}", ctx.rng.range(5_000_000, 30_000_000));
        }
        "bo" => {
            buf.reserve(10);
            ctx.rng.push_digits(buf, 7);
            buf.push('-');
            ctx.rng.push_upper(buf, 2);
        }
        _ => ctx.rng.push_digits(buf, 8),
    }
}
