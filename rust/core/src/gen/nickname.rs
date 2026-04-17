use crate::ctx::GenContext;

use super::helpers::handle::{
    unique_tag, HandleArchetype, HandleInput, SOCIAL_POLICY, USERNAME_SEPS,
};
use super::helpers::nickname as nick;

/// Generate a pure creative nickname (e.g. `darkwolf42`, `swiftcoder`, `nebula_quest`).
/// Unlike `gen_username`/`gen_login` which mix name-based and nick-based patterns,
/// this always produces a creative handle unrelated to the person's real name.
pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let tag = unique_tag(ctx.rng.record(), 0xD1CE);
    let h = HandleInput {
        first: "",
        last: "",
        tag,
        seps: USERNAME_SEPS,
        archetype: HandleArchetype::FullNick,
        policy: SOCIAL_POLICY,
        nick: ctx.identity.and_then(|id| id.nickname.as_ref()),
        birth_year: ctx.identity.map_or(0, |id| id.birth_year),
        birth_month: ctx.identity.map_or(0, |id| id.birth_month),
        birth_day: ctx.identity.map_or(0, |id| id.birth_day),
    };
    if let Some(n) = h.nick {
        // Reuse or mutate stored nickname
        let roll = ctx.rng.urange(0, 99);
        if roll < 55 {
            buf.push_str(&n.full);
        } else {
            nick::mutate_nickname(buf, n, &h, &mut ctx.rng);
        }
    } else {
        nick::gen_nickname(buf, &h, &mut ctx.rng);
    }

    if ctx.modifier == "xuniq" {
        buf.push('_');
        ctx.rng.push_lower_digit(buf, 5);
    }
}
