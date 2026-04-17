use crate::ctx::GenContext;

// Format: Sentry DSN — https://docs.sentry.io/concepts/key-terms/dsn-explainer/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    // "https://" (8) + hex(32) + "@o" (2) + digits(6) + ".ingest.sentry.io/" (19) + digits(7) = 74
    buf.reserve(74);
    buf.push_str("https://");
    ctx.rng.push_hex(buf, 32);
    buf.push_str("@o");
    ctx.rng.push_digits(buf, 6);
    buf.push_str(".ingest.sentry.io/");
    ctx.rng.push_digits(buf, 7);
}
