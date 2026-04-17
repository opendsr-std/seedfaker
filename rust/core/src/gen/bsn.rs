use crate::ctx::GenContext;

// Format: Netherlands BSN — https://www.government.nl/topics/personal-data/citizen-service-number-bsn
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_digits(buf, 9);
}
