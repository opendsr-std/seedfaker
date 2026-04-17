use crate::ctx::GenContext;

// Format: AMA CPT (proprietary) — https://www.ama-assn.org/practice-management/cpt
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    ctx.rng.push_digits(buf, 5);
}
