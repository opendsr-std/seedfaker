use crate::ctx::GenContext;
use crate::rng::Rng;

fn push_pad(buf: &mut String, v: i64, width: usize) {
    let mut ib = itoa::Buffer::new();
    let s = ib.format(v);
    for _ in 0..width.saturating_sub(s.len()) {
        buf.push('0');
    }
    buf.push_str(s);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    match loc.code {
        "en" | "en-ca" | "en-au" | "en-nz" | "en-sg" | "en-za" | "en-ng" | "en-gb" => {
            gen_us_ssn_buf(&mut ctx.rng, ctx.modifier, buf);
        }
        _ => {
            super::national_id::gen(ctx, buf);
        }
    }
}

pub fn gen_us_ssn_buf(rng: &mut Rng, modifier: &str, buf: &mut String) {
    let mut area = rng.range(100, 899);
    if area == 666 {
        area = 665;
    }
    let group = rng.range(1, 99);
    let serial = rng.range(1, 9999);
    if modifier == "plain" {
        push_pad(buf, area, 3);
        push_pad(buf, group, 2);
        push_pad(buf, serial, 4);
    } else {
        push_pad(buf, area, 3);
        buf.push('-');
        push_pad(buf, group, 2);
        buf.push('-');
        push_pad(buf, serial, 4);
    }
}
