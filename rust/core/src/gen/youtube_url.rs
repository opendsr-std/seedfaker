use crate::ctx::GenContext;

use super::ascii_lower;
use super::helpers::handle::{
    gen_handle, unique_tag, HandleArchetype, HandleInput, HIGH_SOCIAL_POLICY, YOUTUBE_SEPS,
};

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let owned_first;
    let owned_last;
    let (first, last, arch, nick) = if let Some(id) = ctx.identity {
        (&*id.first_ascii, &*id.last_ascii, id.archetype, id.nickname.as_ref())
    } else {
        let loc = ctx.locale();
        let fi = ctx.rng.urange(0, loc.first_names.len() - 1);
        let li = ctx.rng.urange(0, loc.last_names.len() - 1);
        owned_first = ascii_lower(&mut ctx.rng, loc.first_names[fi]);
        owned_last = ascii_lower(&mut ctx.rng, loc.last_names[li]);
        (owned_first.as_str(), owned_last.as_str(), HandleArchetype::NameOnly, None)
    };
    let tag = unique_tag(ctx.rng.record(), 0xBE01);
    buf.push_str("https://youtube.com/@");
    let h = HandleInput {
        first,
        last,
        tag,
        seps: YOUTUBE_SEPS,
        archetype: arch,
        policy: HIGH_SOCIAL_POLICY,
        nick,
        birth_year: ctx.identity.map_or(0, |id| id.birth_year),
        birth_month: ctx.identity.map_or(0, |id| id.birth_month),
        birth_day: ctx.identity.map_or(0, |id| id.birth_day),
    };
    gen_handle(buf, &h, &mut ctx.rng);
}
