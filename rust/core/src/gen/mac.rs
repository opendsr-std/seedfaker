use crate::ctx::GenContext;

const HEX: &[u8; 16] = b"0123456789abcdef";

fn push_hex2(buf: &mut String, v: u8) {
    buf.push(HEX[(v >> 4) as usize] as char);
    buf.push(HEX[(v & 0xf) as usize] as char);
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let mut octets = [0u8; 6];
    for o in &mut octets {
        *o = ctx.rng.range(0, 255) as u8;
    }
    match ctx.modifier {
        "plain" => {
            for o in &octets {
                push_hex2(buf, *o);
            }
        }
        "dot" => {
            for (i, o) in octets.iter().enumerate() {
                if i == 2 || i == 4 {
                    buf.push('.');
                }
                push_hex2(buf, *o);
            }
        }
        _ => {
            for (i, o) in octets.iter().enumerate() {
                if i > 0 {
                    buf.push('-');
                }
                push_hex2(buf, *o);
            }
        }
    }
}
