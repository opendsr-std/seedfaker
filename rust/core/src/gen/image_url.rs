use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let (w, h) = match ctx.modifier {
        "1x1" => (512, 512),
        "4x3" => (800, 600),
        "3x2" => (900, 600),
        "16x9" => (1280, 720),
        "21x9" => (1260, 540),
        "9x16" => (720, 1280),
        "3x4" => (600, 800),
        "2x3" => (600, 900),
        _ => {
            let ratios: &[(i64, i64)] = &[
                (512, 512),
                (800, 600),
                (900, 600),
                (1280, 720),
                (1920, 1080),
                (720, 1280),
                (600, 800),
                (1260, 540),
            ];
            ratios[ctx.rng.urange(0, ratios.len() - 1)]
        }
    };
    let id = ctx.rng.range(1, 1000);
    // https://picsum.photos/seed/{id}/{w}/{h} — ~44 chars max
    buf.reserve(44);
    let _ = write!(buf, "https://picsum.photos/seed/{id}/{w}/{h}");
}
