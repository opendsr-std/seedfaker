use crate::ctx::GenContext;

use super::ascii_lower;
use super::helpers::handle::{
    gen_handle, unique_tag, HandleArchetype, HandleInput, SepRules, USERNAME_POLICY,
};

/// Clean username: `[a-z0-9_]` only, as accepted by most platforms.
const CLEAN_USERNAME_SEPS: SepRules = SepRules { allowed: [b'_', 0, 0], count: 1, use_pct: 12 };

/// Handles shorter than this get extra tag-derived digits for dedup.
/// Real usernames peak at 7-10 chars — guard only ultra-short ones
/// (initials, 2-3 letter abbreviations) to preserve natural patterns.
const GUARD_THRESHOLD: usize = 6;

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
    let tag = unique_tag(ctx.rng.record(), 0xBEEF);
    let start = buf.len();
    let h = HandleInput {
        first,
        last,
        tag,
        seps: CLEAN_USERNAME_SEPS,
        archetype: arch,
        policy: USERNAME_POLICY,
        nick,
        birth_year: ctx.identity.map_or(0, |id| id.birth_year),
        birth_month: ctx.identity.map_or(0, |id| id.birth_month),
        birth_day: ctx.identity.map_or(0, |id| id.birth_day),
    };
    gen_handle(buf, &h, &mut ctx.rng);

    // Enforce [a-z0-9_]: lowercase and strip disallowed chars.
    let generated = buf[start..].to_ascii_lowercase();
    buf.truncate(start);
    for c in generated.bytes() {
        match c {
            b'a'..=b'z' | b'0'..=b'9' | b'_' => buf.push(c as char),
            b'-' | b'.' => buf.push('_'),
            _ => {}
        }
    }

    // Dedup guard: short handles have high collision risk at scale.
    // Append 2 tag-derived digits when handle is under threshold.
    let handle_len = buf.len() - start;
    if handle_len < GUARD_THRESHOLD {
        let guard = tag % 90 + 10;
        buf.push_str(itoa::Buffer::new().format(guard));
    }

    // :xuniq — extended uniqueness suffix
    if ctx.modifier == "xuniq" {
        buf.push('_');
        ctx.rng.push_lower_digit(buf, 5);
    }
}
