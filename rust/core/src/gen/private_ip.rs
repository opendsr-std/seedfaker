use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    buf.reserve(15);
    match ctx.rng.urange(0, 2) {
        0 => {
            let a = ctx.rng.range(0, 255);
            let b = ctx.rng.range(0, 255);
            let c = ctx.rng.range(1, 254);
            let _ = write!(buf, "10.{a}.{b}.{c}");
        }
        1 => {
            let a = ctx.rng.range(16, 31);
            let b = ctx.rng.range(0, 255);
            let c = ctx.rng.range(1, 254);
            let _ = write!(buf, "172.{a}.{b}.{c}");
        }
        _ => {
            let a = ctx.rng.range(0, 255);
            let b = ctx.rng.range(1, 254);
            let _ = write!(buf, "192.168.{a}.{b}");
        }
    }
}
