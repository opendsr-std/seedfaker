use std::fmt::Write;

use crate::ctx::GenContext;

const NAMES: &[&str] = &[
    "red",
    "blue",
    "green",
    "yellow",
    "orange",
    "purple",
    "pink",
    "brown",
    "black",
    "white",
    "gray",
    "cyan",
    "magenta",
    "lime",
    "teal",
    "navy",
    "maroon",
    "olive",
    "coral",
    "salmon",
    "crimson",
    "indigo",
    "violet",
    "turquoise",
    "gold",
    "silver",
    "beige",
    "ivory",
    "khaki",
    "lavender",
    "plum",
    "orchid",
    "sienna",
    "tan",
    "aqua",
    "chartreuse",
    "fuchsia",
    "tomato",
    "slate gray",
    "steel blue",
    "forest green",
    "royal blue",
    "dark red",
    "sea green",
    "midnight blue",
    "sandy brown",
    "pale green",
    "deep pink",
    "medium purple",
];

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let r = ctx.rng.range(0, 255) as u8;
    let g = ctx.rng.range(0, 255) as u8;
    let b = ctx.rng.range(0, 255) as u8;
    match ctx.modifier {
        "hex" => {
            buf.reserve(7);
            let _ = write!(buf, "#{r:02x}{g:02x}{b:02x}");
        }
        "rgb" => {
            buf.reserve(12);
            let _ = write!(buf, "{r}, {g}, {b}");
        }
        "rgba" => {
            let a = ctx.rng.range(10, 99);
            buf.reserve(18);
            let _ = write!(buf, "{r}, {g}, {b}, 0.{a}");
        }
        _ => buf.push_str(NAMES[ctx.rng.urange(0, NAMES.len() - 1)]),
    }
}
