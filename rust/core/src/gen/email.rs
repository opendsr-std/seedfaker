use crate::ctx::GenContext;
use crate::locale::shared;

use super::ascii_lower;
use super::helpers::handle::{
    gen_handle, unique_tag, HandleArchetype, HandleInput, EMAIL_POLICY, EMAIL_SEPS,
};

/// Sanitize email local part in-place: collapse `..` → `.`, strip leading/trailing dots.
/// RFC 5321: dots cannot be consecutive or at boundaries in unquoted local parts.
fn sanitize_email_local(buf: &mut String, start: usize) {
    let local = buf[start..].to_string();
    buf.truncate(start);
    let mut prev_dot = true; // treat start as "previous was dot" to strip leading dot
    for c in local.bytes() {
        if c == b'.' {
            if prev_dot {
                continue; // skip consecutive dots
            }
            prev_dot = true;
        } else {
            prev_dot = false;
        }
        buf.push(c as char);
    }
    // strip trailing dot
    if buf.len() > start && buf.as_bytes()[buf.len() - 1] == b'.' {
        buf.pop();
    }
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let tag = unique_tag(ctx.rng.record(), loc.domains.len() as u64 ^ 0xA5A5);

    // Domain selection:
    //  3% personal domain (name.dev, surname.io — like real devs/freelancers)
    // 33% global provider (gmail, yahoo, outlook)
    // 64% locale-specific (weighted by common tier)
    let domain_roll = (tag >> 32) % 100;

    let owned_first;
    let owned_last;
    let (first, last, arch, nick) = if let Some(id) = ctx.identity {
        (&*id.first_ascii, &*id.last_ascii, id.archetype, id.nickname.as_ref())
    } else {
        let fi = ctx.rng.urange(0, loc.first_names.len() - 1);
        let li = ctx.rng.urange(0, loc.last_names.len() - 1);
        owned_first = ascii_lower(&mut ctx.rng, loc.first_names[fi]);
        owned_last = ascii_lower(&mut ctx.rng, loc.last_names[li]);
        (owned_first.as_str(), owned_last.as_str(), HandleArchetype::NameOnly, None)
    };

    let h = HandleInput {
        first,
        last,
        tag,
        seps: EMAIL_SEPS,
        archetype: arch,
        policy: EMAIL_POLICY,
        nick,
        birth_year: ctx.identity.map_or(0, |id| id.birth_year),
        birth_month: ctx.identity.map_or(0, |id| id.birth_month),
        birth_day: ctx.identity.map_or(0, |id| id.birth_day),
    };
    let local_start = buf.len();
    gen_handle(buf, &h, &mut ctx.rng);

    // Sanitize local part: collapse `..` → `.`, strip leading/trailing `.`
    sanitize_email_local(buf, local_start);

    // :xuniq — extended uniqueness tag before @
    if ctx.modifier == "xuniq" {
        buf.push('.');
        ctx.rng.push_lower_digit(buf, 5);
    }

    buf.push('@');

    if domain_roll < 3 {
        // Personal domain: name@lastname.dev, name@firstname.io
        let tld = shared::PERSONAL_TLDS[(tag % shared::PERSONAL_TLDS.len() as u64) as usize];
        buf.push_str(last);
        buf.push('.');
        buf.push_str(tld);
    } else if domain_roll < 36 {
        let d = shared::weighted_choice(
            &mut ctx.rng,
            shared::GLOBAL_PROVIDERS,
            shared::GLOBAL_PROVIDERS_COMMON,
        );
        buf.push_str(d);
    } else {
        let d = shared::weighted_choice(&mut ctx.rng, loc.domains, loc.domains_common);
        buf.push_str(d);
    }
}
