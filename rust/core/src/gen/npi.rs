use crate::ctx::GenContext;

// Format: US NPI — https://npiregistry.cms.hhs.gov/
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_digits(buf, 10);
}
