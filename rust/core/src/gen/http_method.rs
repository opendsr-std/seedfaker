use crate::ctx::GenContext;

/// Weighted HTTP method distribution based on real-world traffic analysis.
/// GET dominates (~63%), POST second (~22%), rest are rare.
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let w = ctx.rng.urange(0, 99);
    buf.push_str(match w {
        0..=62 => "GET",     // 63%
        63..=84 => "POST",   // 22%
        85..=89 => "PUT",    //  5%
        90..=93 => "DELETE", //  4%
        94..=96 => "PATCH",  //  3%
        97..=98 => "HEAD",   //  2%
        _ => "OPTIONS",      //  1%
    });
}
