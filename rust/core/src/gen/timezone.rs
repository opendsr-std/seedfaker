use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if let Some(id) = ctx.identity {
        buf.push_str(id.tz);
        return;
    }
    let tzs = [
        "America/New_York",
        "America/Chicago",
        "America/Los_Angeles",
        "America/Sao_Paulo",
        "Europe/London",
        "Europe/Berlin",
        "Europe/Paris",
        "Europe/Moscow",
        "Asia/Tokyo",
        "Asia/Shanghai",
        "Asia/Kolkata",
        "Asia/Dubai",
        "Australia/Sydney",
        "Pacific/Auckland",
        "Africa/Johannesburg",
    ];
    buf.push_str(tzs[ctx.rng.urange(0, tzs.len() - 1)]);
}
