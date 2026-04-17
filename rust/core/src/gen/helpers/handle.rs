//! Shared handle generation for login-like fields.
//!
//! Three user archetypes model real-world behavior:
//! - `NameOnly` (45%): name-based handles everywhere
//! - `NickSocial` (40%): name for email, nickname for social
//! - `FullNick` (15%): nickname everywhere including email
//!
//! Per-platform separator rules (`SepRules`) control which separators
//! are allowed and how often they appear. Each platform constant
//! reflects real-world data (e.g. GitHub only allows `-`, email
//! prefers `.`, Twitter/Telegram allow `_` only).
//!
//! Weight tables in `gen_name_handle` are calibrated from analysis
//! of 150K GitHub user profiles. Key findings:
//! - 87% of logins have NO trailing digits
//! - 91% have NO separator (GitHub-specific; 9% use hyphen)
//! - `firstlast` (38.9%) dominates name-based patterns
//! - 33% of logins are completely unrelated to the real name

use crate::rng::Rng;

use super::nickname::{self, Nickname};

// ─── separator rules ──────────────────────────────────────────────────

/// Per-platform separator configuration.
#[derive(Clone, Copy)]
pub struct SepRules {
    /// Allowed separator bytes in priority order.
    pub allowed: [u8; 3],
    /// Number of valid entries in `allowed` (0–3).
    pub count: u8,
    /// Probability (0–100) of inserting a separator when optional.
    pub use_pct: u8,
}

// Platform-specific separator constants (from 150K GitHub analysis).
pub const GITHUB_SEPS: SepRules = SepRules { allowed: [b'-', 0, 0], count: 1, use_pct: 9 };
pub const EMAIL_SEPS: SepRules = SepRules { allowed: [b'.', b'-', b'_'], count: 3, use_pct: 60 };
pub const TWITTER_SEPS: SepRules = SepRules { allowed: [b'_', 0, 0], count: 1, use_pct: 5 };
pub const INSTAGRAM_SEPS: SepRules = SepRules { allowed: [b'_', b'.', 0], count: 2, use_pct: 15 };
pub const FACEBOOK_SEPS: SepRules = SepRules { allowed: [b'.', 0, 0], count: 1, use_pct: 20 };
pub const TELEGRAM_SEPS: SepRules = SepRules { allowed: [b'_', 0, 0], count: 1, use_pct: 10 };
pub const YOUTUBE_SEPS: SepRules = SepRules { allowed: [b'-', b'_', b'.'], count: 3, use_pct: 15 };
pub const SOCIAL_SEPS: SepRules = SepRules { allowed: [b'_', b'.', 0], count: 2, use_pct: 15 };
pub const USERNAME_SEPS: SepRules = SepRules { allowed: [b'_', b'-', b'.'], count: 3, use_pct: 15 };
/// For creative nickname construction (`dark_wolf` style).
pub const NICKNAME_SEPS: SepRules = SepRules { allowed: [b'_', 0, 0], count: 1, use_pct: 45 };

fn sep_str(b: u8) -> &'static str {
    match b {
        b'-' => "-",
        b'_' => "_",
        b'.' => ".",
        _ => "",
    }
}

/// The platform's primary separator (deterministic, no rng).
/// Used when adapting a stored nickname to a specific platform.
pub fn platform_sep(rules: SepRules) -> &'static str {
    if rules.count == 0 {
        ""
    } else {
        sep_str(rules.allowed[0])
    }
}

/// Always return a separator from the allowed list (for patterns that
/// explicitly require one, like `first-last`). Returns `""` only if
/// the platform allows no separators at all.
pub fn pick_sep(rng: &mut Rng, rules: SepRules) -> &'static str {
    if rules.count == 0 {
        return "";
    }
    // 85% primary, 15% secondary (if available)
    let idx = usize::from(rules.count > 1 && rng.urange(0, 99) < 15);
    sep_str(rules.allowed[idx])
}

/// Probabilistically return a separator (for patterns where sep is
/// optional, e.g. nickname words). Returns `""` with probability
/// `(100 - use_pct)%`.
pub fn maybe_sep(rng: &mut Rng, rules: SepRules) -> &'static str {
    if rules.count == 0 || rng.urange(0, 99) >= rules.use_pct as usize {
        return "";
    }
    pick_sep(rng, rules)
}

// ─── uniqueness tags ──────────────────────────────────────────────────

/// Bijective scramble of record number. Deterministic, collision-free.
pub fn unique_tag(record: u64, key: u64) -> u64 {
    let x = record.wrapping_add(1) ^ key;
    x.wrapping_mul(0x9E37_79B9_7F4A_7C15)
}

// ─── unified suffix system ────────────────────────────────────────────
//
// All suffix functions work identically with and without ctx.
// With ctx: real birth year / month / day.
// Without ctx: plausible numbers derived from tag.

/// Short "year-like" 2-digit suffix: `94`, `01`, `87`.
/// With identity (ctx strict): 60% chance of using real birth year % 100,
/// 40% tag-derived. Without identity: 100% tag-derived.
/// This creates realistic `john94` patterns that correlate with birthday.
pub(super) fn push_year2(buf: &mut String, h: &HandleInput<'_>) {
    let y = if h.birth_year > 0 && (h.tag >> 40) % 100 < 60 {
        (h.birth_year % 100) as u64
    } else {
        h.tag % 100
    };
    if y < 10 {
        buf.push('0');
    }
    buf.push_str(itoa::Buffer::new().format(y));
}

/// Full birth year: `1994`, `2001`.
/// With identity: 60% real birth year, 40% tag-derived (1975-2024).
/// Low cardinality (50 values) — use at ≤4% weight to limit collision impact.
pub(super) fn push_year4(buf: &mut String, h: &HandleInput<'_>) {
    if h.birth_year > 0 && (h.tag >> 40) % 100 < 60 {
        buf.push_str(itoa::Buffer::new().format(h.birth_year));
    } else {
        buf.push_str(itoa::Buffer::new().format(h.tag % 50 + 1975));
    }
}

/// Birth-month + day: `0512`, `1203`. Used as `john0512` style suffix.
/// With identity: 60% real birth month+day, 40% tag-derived.
pub(super) fn push_bday_mmdd(buf: &mut String, h: &HandleInput<'_>) {
    let (m, d) = if h.birth_month > 0 && (h.tag >> 40) % 100 < 60 {
        (u64::from(h.birth_month), u64::from(h.birth_day))
    } else {
        ((h.tag >> 5) % 12 + 1, h.tag % 28 + 1)
    };
    let mut ib = itoa::Buffer::new();
    if m < 10 {
        buf.push('0');
    }
    buf.push_str(ib.format(m));
    if d < 10 {
        buf.push('0');
    }
    buf.push_str(ib.format(d));
}

/// "Birthday" DDMM: `0512`, `2303`.
/// With identity: 60% real day+month, 40% tag-derived.
pub(super) fn push_bday(buf: &mut String, h: &HandleInput<'_>) {
    let (d, m) = if h.birth_month > 0 && (h.tag >> 40) % 100 < 60 {
        (u64::from(h.birth_day), u64::from(h.birth_month))
    } else {
        (h.tag % 28 + 1, (h.tag >> 5) % 12 + 1)
    };
    let mut ib = itoa::Buffer::new();
    if d < 10 {
        buf.push('0');
    }
    buf.push_str(ib.format(d));
    if m < 10 {
        buf.push('0');
    }
    buf.push_str(ib.format(m));
}

/// Two non-year digits 10-99.
fn push_2digit(buf: &mut String, tag: u64) {
    buf.push_str(itoa::Buffer::new().format(tag % 90 + 10));
}

/// Three digits 100-999.
fn push_3digit(buf: &mut String, tag: u64) {
    buf.push_str(itoa::Buffer::new().format(tag % 900 + 100));
}

/// Zero-padded 3-digit: `001`–`999`. Distinct from bare 3-digit (100-999).
pub(super) fn push_zpad3(buf: &mut String, tag: u64) {
    let v = tag % 999 + 1;
    if v < 10 {
        buf.push_str("00");
    } else if v < 100 {
        buf.push('0');
    }
    buf.push_str(itoa::Buffer::new().format(v));
}

/// Zero-padded 4-digit: `0001`–`9999`. High cardinality (9999 values).
pub(super) fn push_zpad4(buf: &mut String, tag: u64) {
    let v = tag % 9999 + 1;
    if v < 10 {
        buf.push_str("000");
    } else if v < 100 {
        buf.push_str("00");
    } else if v < 1000 {
        buf.push('0');
    }
    buf.push_str(itoa::Buffer::new().format(v));
}

/// Letter + single digit: `x7`, `k3`, `m0`. 26×10 = 260 combos.
pub(super) fn push_letter_digit(buf: &mut String, tag: u64) {
    buf.push((b'a' + (tag % 26) as u8) as char);
    buf.push((b'0' + ((tag >> 5) % 10) as u8) as char);
}

/// Short hex suffix without separator: `a3f`, `7b2`. 4096 values.
pub(super) fn push_hex3(buf: &mut String, tag: u64) {
    let h3 = tag & 0xFFF;
    const HEX: &[u8; 16] = b"0123456789abcdef";
    buf.push(HEX[((h3 >> 8) & 0xF) as usize] as char);
    buf.push(HEX[((h3 >> 4) & 0xF) as usize] as char);
    buf.push(HEX[(h3 & 0xF) as usize] as char);
}

// ─── letter-based mutations ───────────────────────────────────────────
//
// Real users make handles unique with letter tricks when digits feel wrong:
// - alexx, alexxx (double/triple last char)
// - danielx, danielz (cool letter suffix)
// - alex_j, alex_x (underscore + letter)
// - alexo, alexe (vowel ending)
// - alexkk, alexrr (double consonant)

/// Append a letter-based variation for organic-looking uniqueness.
/// Uses tag bits for both mutation type AND letter choice — different records
/// with the same base get different mutations.
fn push_letter_variation(buf: &mut String, tag: u64) {
    // Use different tag bits for type vs letter to maximize entropy
    let variant = (tag >> 20) % 10;
    let letter_bits = tag >> 28; // separate bits for letter selection
    match variant {
        // tag-derived letter + double: alexkk, danielrr (26×1 = 26 combos)
        0 => {
            let c = (b'a' + (letter_bits % 26) as u8) as char;
            buf.push(c);
            buf.push(c);
        }
        // x/z + tag digit: alexz3, danielx7 (2×10 = 20 combos)
        1 => {
            let c = if letter_bits & 1 == 0 { 'x' } else { 'z' };
            buf.push(c);
            buf.push((b'0' + (letter_bits % 10) as u8) as char);
        }
        // underscore + tag-derived letter: alex_j, daniel_m (26 combos)
        2 => {
            buf.push('_');
            buf.push((b'a' + (letter_bits % 26) as u8) as char);
        }
        // two tag-derived letters: alexjm, danielrk (26×26 = 676 combos)
        3..=4 => {
            buf.push((b'a' + (letter_bits % 26) as u8) as char);
            buf.push((b'a' + ((letter_bits >> 5) % 26) as u8) as char);
        }
        // vowel + consonant: alexok, danieler (5×21 = 105 combos)
        5 => {
            const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
            const CONSONANTS: &[u8] = b"bcdfghjklmnpqrstvwxyz";
            buf.push(VOWELS[(letter_bits % 5) as usize] as char);
            buf.push(CONSONANTS[((letter_bits >> 3) % CONSONANTS.len() as u64) as usize] as char);
        }
        // single letter suffix: alexk, danielr (26 combos)
        6..=7 => {
            buf.push((b'a' + (letter_bits % 26) as u8) as char);
        }
        // suffix word: alexdev, danielpro (34 combos)
        8 => {
            let sfx = nickname::SUFFIXES[(letter_bits % nickname::SUFFIXES.len() as u64) as usize];
            buf.push_str(sfx);
        }
        // underscore + two letters: alex_jm (26×26 = 676 combos)
        _ => {
            buf.push('_');
            buf.push((b'a' + (letter_bits % 26) as u8) as char);
            buf.push((b'a' + ((letter_bits >> 5) % 26) as u8) as char);
        }
    }
}

// ─── adaptation system ────────────────────────────────────────────────

/// Short hex suffix from tag. Looks like app-generated ID fragment.
/// Examples: `_a3f`, `_7b2`, `_e09`. Always unique — bijective from record.
fn push_hex_tag(buf: &mut String, tag: u64, seps: SepRules) {
    buf.push_str(platform_sep(seps));
    let h3 = tag & 0xFFF; // 12 bits = 4096 values, 3 hex chars
    const HEX: &[u8; 16] = b"0123456789abcdef";
    buf.push(HEX[((h3 >> 8) & 0xF) as usize] as char);
    buf.push(HEX[((h3 >> 4) & 0xF) as usize] as char);
    buf.push(HEX[(h3 & 0xF) as usize] as char);
}

/// Adaptation-driven suffix for name-based handles (`johndoe42`, `maria_garcia94`).
/// Deterministic: same tag → same suffix type + value.
///
/// Name-based handles need digit-centric suffixes — underscore-prefixed variants
/// like `_007` are unnatural here (those belong in nickname handles).
/// Entropy comes from heavy 2-4digit (9000) and 3digit (900) weights.
fn push_adapted_suffix(buf: &mut String, h: &HandleInput<'_>) {
    let v = (h.tag >> 16) % 100;
    match v {
        0..=7 => push_year2(buf, h), //  8% year2 (100)
        8..=34 => buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1)), // 27% 2-4digit (9000)
        35..=49 => push_3digit(buf, h.tag), // 15% 3digit (900)
        50..=56 => push_bday(buf, h), //  7% bday (336)
        57..=59 => push_year4(buf, h), //  3% year4 (50) — max_1995
        60..=64 => push_zpad3(buf, h.tag), //  5% 007-style (999)
        65..=74 => push_letter_variation(buf, h.tag), // 10% letter (26-676)
        75..=82 => {
            push_letter_variation(buf, h.tag); //  8% letter + 2digit
            push_2digit(buf, h.tag >> 8);
        }
        83..=87 => push_2digit(buf, h.tag), //  5% 2digit (90)
        88..=93 => push_letter_digit(buf, h.tag), //  6% x7/k3 (260)
        _ => push_hex_tag(buf, h.tag, h.seps), //  6% _a3f (4096)
    }
}

/// High-entropy suffix for low-cardinality base patterns (`first_only`, `initials`).
/// Heavily biased toward 2-4digit (9000) and 3digit (900) for maximum uniqueness
/// while keeping realistic digit-centric appearance.
fn push_required_suffix(buf: &mut String, h: &HandleInput<'_>) {
    let v = (h.tag >> 16) % 100;
    match v {
        0..=4 => push_year2(buf, h), //  5% year2 (100)
        5..=39 => buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1)), // 35% 2-4digit (9000)
        40..=59 => push_3digit(buf, h.tag), // 20% 3digit (900)
        60..=69 => push_bday(buf, h), // 10% bday (336)
        70..=77 => push_zpad3(buf, h.tag), //  8% 007-style (999)
        78..=84 => {
            push_letter_variation(buf, h.tag); //  7% letter + 2digit
            push_2digit(buf, h.tag >> 8);
        }
        85..=89 => push_letter_digit(buf, h.tag), //  5% x7/k3 (260)
        90..=94 => push_2digit(buf, h.tag),       //  5% 2digit (90)
        _ => push_hex_tag(buf, h.tag, h.seps),    //  5% _a3f (4096)
    }
}

/// General suffix: weighted random pick (for nicknames, mixed contexts).
pub(super) fn push_suffix(buf: &mut String, h: &HandleInput<'_>, rng: &mut Rng) {
    let w = rng.urange(0, 99);
    match w {
        0..=24 => push_year2(buf, h),       // 25% (100 values)
        25..=49 => push_3digit(buf, h.tag), // 25% (900 values)
        50..=64 => push_bday(buf, h),       // 15% (336 values)
        65..=79 => push_2digit(buf, h.tag), // 15% (90 values)
        _ => {
            // 20% tag 2-4 digit (9000 values)
            buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1));
        }
    }
}

// ─── archetype ────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HandleArchetype {
    NameOnly = 0,
    NickSocial = 1,
    FullNick = 2,
}

pub fn pick_archetype(rng: &mut Rng) -> HandleArchetype {
    let w = rng.urange(0, 99);
    match w {
        // Most real users use name-based handles even on social platforms.
        // Creative nicknames are the minority — gamers, streamers, teens.
        0..=54 => HandleArchetype::NameOnly,    // 55%
        55..=84 => HandleArchetype::NickSocial, // 30%
        _ => HandleArchetype::FullNick,         // 15%
    }
}

// ─── handle policy ────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct HandlePolicy {
    pub ns_reuse: u8,
    pub ns_mutate: u8,
    pub ns_fresh: u8,
    pub fn_reuse: u8,
    pub fn_mutate: u8,
    pub fn_fresh: u8,
}

pub const EMAIL_POLICY: HandlePolicy = HandlePolicy {
    ns_reuse: 5,
    ns_mutate: 0,
    ns_fresh: 0,
    // Even gamers (FullNick) mostly use name-based email addresses.
    fn_reuse: 15,
    fn_mutate: 5,
    fn_fresh: 5,
};

pub const USERNAME_POLICY: HandlePolicy = HandlePolicy {
    // NickSocial: 45% nickname, 55% name-based — even gamers often have
    // normal-looking usernames on non-gaming platforms.
    ns_reuse: 25,
    ns_mutate: 15,
    ns_fresh: 5,
    fn_reuse: 50,
    fn_mutate: 20,
    fn_fresh: 10,
};

pub const SOCIAL_POLICY: HandlePolicy = HandlePolicy {
    ns_reuse: 45,
    ns_mutate: 25,
    ns_fresh: 10,
    fn_reuse: 55,
    fn_mutate: 25,
    fn_fresh: 10,
};

pub const FACEBOOK_POLICY: HandlePolicy = HandlePolicy {
    ns_reuse: 15,
    ns_mutate: 10,
    ns_fresh: 5,
    fn_reuse: 35,
    fn_mutate: 15,
    fn_fresh: 10,
};

pub const GITHUB_POLICY: HandlePolicy = HandlePolicy {
    ns_reuse: 30,
    ns_mutate: 20,
    ns_fresh: 15,
    fn_reuse: 45,
    fn_mutate: 25,
    fn_fresh: 15,
};

pub const HIGH_SOCIAL_POLICY: HandlePolicy = HandlePolicy {
    ns_reuse: 45,
    ns_mutate: 25,
    ns_fresh: 10,
    fn_reuse: 55,
    fn_mutate: 25,
    fn_fresh: 10,
};

// ─── public API ───────────────────────────────────────────────────────

pub struct HandleInput<'a> {
    pub first: &'a str,
    pub last: &'a str,
    pub tag: u64,
    pub seps: SepRules,
    pub archetype: HandleArchetype,
    pub policy: HandlePolicy,
    pub nick: Option<&'a Nickname>,
    pub birth_year: i64,
    pub birth_month: u8,
    pub birth_day: u8,
}

pub fn gen_handle(buf: &mut String, h: &HandleInput<'_>, rng: &mut Rng) {
    if h.archetype == HandleArchetype::NameOnly {
        gen_name_handle(buf, h, rng);
        return;
    }
    let (p_reuse, p_mutate, p_fresh) = if h.archetype == HandleArchetype::NickSocial {
        (h.policy.ns_reuse, h.policy.ns_mutate, h.policy.ns_fresh)
    } else {
        (h.policy.fn_reuse, h.policy.fn_mutate, h.policy.fn_fresh)
    };
    let roll = rng.urange(0, 99) as u8;
    if let Some(n) = h.nick {
        if roll < p_reuse {
            // Adapt stored nickname to platform — always mutate slightly
            // so username != nickname (real users vary handles per platform).
            nickname::mutate_nickname(buf, n, h, rng);
            return;
        }
        if roll < p_reuse + p_mutate {
            nickname::mutate_nickname(buf, n, h, rng);
            return;
        }
    }
    if roll < p_reuse + p_mutate + p_fresh {
        nickname::gen_nickname(buf, h, rng);
    } else {
        // Name-based, but with hybrid chance when nick is available
        if h.nick.is_some() && rng.urange(0, 99) < 25 {
            gen_hybrid_handle(buf, h, rng);
        } else {
            gen_name_handle(buf, h, rng);
        }
    }
}

// ─── name-based handle generation ─────────────────────────────────────
//
// Two-stage generation: first pick a length tier (from tag), then a pattern
// that naturally produces handles in that length range.
//
// Target length distribution (general-purpose, not GitHub-specific):
//   4-6: 15%   7-8: 22%   9-10: 23%   11-13: 25%   14+: 15%
//
// Bare (no digits): ~42% — realistic for mixed-audience platforms.
// Birth-year correlation: push_year2/push_year4 use real birth year when
// identity is available (ctx strict), ~60% of the time.

/// Append a single tag-derived letter for collision safety on bare handles.
/// 50% chance: `johndoe` stays bare, 50% becomes `johndoek`.
/// Both look realistic — many real handles end with an extra letter
/// (benbjohnson, samypesse, sophiebits).
#[inline]
fn maybe_disambiguate(buf: &mut String, tag: u64) {
    if (tag >> 44) & 1 == 1 {
        buf.push((b'a' + ((tag >> 36) % 26) as u8) as char);
    }
}

/// Length tier: controls which patterns are selected to hit target length range.
fn length_tier(tag: u64) -> u8 {
    let v = (tag >> 48) % 100;
    match v {
        0..=14 => 0,  // 15% short (4-8 chars)
        15..=36 => 1, // 22% medium-short (7-10 chars)
        37..=59 => 2, // 23% medium (9-12 chars)
        60..=84 => 3, // 25% medium-long (11-14 chars)
        _ => 4,       // 15% long (13+ chars)
    }
}

fn gen_name_handle(buf: &mut String, h: &HandleInput<'_>, rng: &mut Rng) {
    let first = h.first;
    let last = h.last;
    let f = first.as_bytes()[0] as char;
    let l = last.as_bytes()[0] as char;

    match length_tier(h.tag) {
        // ── TIER 0: short (4-8 chars, 15%) ───────────────────
        // Patterns that naturally produce short handles. Mostly bare.
        0 => {
            let w = rng.urange(0, 99);
            match w {
                // flast bare (25%) — fchollet, jdoe, jdoek
                0..=24 => {
                    buf.push(f);
                    buf.push_str(last);
                    maybe_disambiguate(buf, h.tag);
                }
                // firstl bare (20%) — wesm, evanw, mitchellh
                25..=44 => {
                    buf.push_str(first);
                    buf.push(l);
                    maybe_disambiguate(buf, h.tag);
                }
                // syllable bare (15%) — fabpot, dmitshur
                45..=59 if first.len() >= 3 && last.len() >= 3 => {
                    let take_f = rng.urange(2, first.len().min(4));
                    let take_l = rng.urange(2, last.len().min(4));
                    buf.push_str(&first[..take_f]);
                    buf.push_str(&last[..take_l]);
                    maybe_disambiguate(buf, h.tag);
                }
                // first_only + letter (15%) — ferossk, yangshunz
                60..=74 => {
                    buf.push_str(first);
                    push_letter_digit(buf, h.tag);
                }
                // last_only + letter (10%) — torvaldsk
                75..=84 => {
                    buf.push_str(last);
                    push_letter_digit(buf, h.tag);
                }
                // initials + required (10%) — jd42, ab007
                85..=94 => {
                    buf.push(f);
                    buf.push(l);
                    push_required_suffix(buf, h);
                }
                // first + year2 (5%) — john94, maria01
                _ => {
                    buf.push_str(first);
                    push_year2(buf, h);
                }
            }
        }

        // ── TIER 1: medium-short (7-10 chars, 22%) ───────────
        // Firstlast bare dominates. Some with short suffixes.
        1 => {
            let w = rng.urange(0, 99);
            match w {
                // firstlast bare (40%) — johndoe, johndoek
                0..=39 => {
                    buf.push_str(first);
                    buf.push_str(last);
                    maybe_disambiguate(buf, h.tag);
                }
                // first{sep}last bare (10%)
                40..=49 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    maybe_disambiguate(buf, h.tag);
                }
                // last + suffix_word bare (8%) — mouredev, akitadev
                50..=57 => {
                    buf.push_str(last);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                }
                // first + suffix_word bare (7%) — jessfraz, dalindev
                58..=64 => {
                    buf.push_str(first);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                }
                // lastfirst bare (5%) — goldbergyoni, chenshuo
                65..=69 => {
                    buf.push_str(last);
                    buf.push_str(first);
                    maybe_disambiguate(buf, h.tag);
                }
                // flast + year2 (5%) — jdoe94
                70..=74 => {
                    buf.push(f);
                    buf.push_str(last);
                    push_year2(buf, h);
                }
                // first + year4 (5%) — maria1994, john2001
                75..=79 => {
                    buf.push_str(first);
                    push_year4(buf, h);
                }
                // first + year2 (5%) — john94, maria01
                80..=84 => {
                    buf.push_str(first);
                    push_year2(buf, h);
                }
                // prefix + first bare (5%) — thejohn, realmaria, imjohn
                85..=89 => {
                    let prefix =
                        nickname::PREFIXES[(h.tag % nickname::PREFIXES.len() as u64) as usize];
                    buf.push_str(prefix);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(first);
                }
                // first + bday_mmdd (5%) — john0512, maria1203
                90..=94 => {
                    buf.push_str(first);
                    push_bday_mmdd(buf, h);
                }
                // last + year2 (5%) — doe94, smith01
                _ => {
                    buf.push_str(last);
                    push_year2(buf, h);
                }
            }
        }

        // ── TIER 2: medium (9-12 chars, 23%) ─────────────────
        // Mix of bare firstlast and suffixed patterns.
        2 => {
            let w = rng.urange(0, 99);
            match w {
                // firstlast bare (25%) — johndoe, johndoem
                0..=24 => {
                    buf.push_str(first);
                    buf.push_str(last);
                    maybe_disambiguate(buf, h.tag);
                }
                // first{sep}last bare (10%)
                25..=34 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    maybe_disambiguate(buf, h.tag);
                }
                // firstlast + year2 (10%) — johndoe94
                35..=44 => {
                    buf.push_str(first);
                    buf.push_str(last);
                    push_year2(buf, h);
                }
                // first{sep}last + year2 (10%) — john_doe94
                45..=54 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    push_year2(buf, h);
                }
                // first + suffix_word + year2 (8%) — johndev94
                55..=62 => {
                    buf.push_str(first);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                    push_year2(buf, h);
                }
                // first{sep}last{sep}suffix_word bare (7%) — john_doe_dev
                63..=69 => {
                    buf.push_str(first);
                    let sep = pick_sep(rng, h.seps);
                    buf.push_str(sep);
                    buf.push_str(last);
                    buf.push_str(sep);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                }
                // firstlast + year4 (5%) — johndoe1994
                70..=74 => {
                    buf.push_str(first);
                    buf.push_str(last);
                    push_year4(buf, h);
                }
                // last{sep}first + year2 (5%)
                75..=79 => {
                    buf.push_str(last);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(first);
                    push_year2(buf, h);
                }
                // first + truncated_last + adapted (5%) — jessfr42
                80..=84 => {
                    buf.push_str(first);
                    if last.len() >= 5 {
                        let take = rng.urange(2, last.len().min(4));
                        buf.push_str(&last[..take]);
                    } else {
                        buf.push_str(last);
                    }
                    push_adapted_suffix(buf, h);
                }
                // first + bday (5%) — john0512
                85..=89 => {
                    buf.push_str(first);
                    push_bday(buf, h);
                }
                // first{sep}last + bday_mmdd (5%) — john_doe0512
                90..=94 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    push_bday_mmdd(buf, h);
                }
                // prefix + first + year2 (5%) — thejohn94
                _ => {
                    let prefix =
                        nickname::PREFIXES[(h.tag % nickname::PREFIXES.len() as u64) as usize];
                    buf.push_str(prefix);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(first);
                    push_year2(buf, h);
                }
            }
        }

        // ── TIER 3: medium-long (11-14 chars, 25%) ───────────
        // Suffixed patterns dominate.
        3 => {
            let w = rng.urange(0, 99);
            match w {
                // firstlast + adapted (20%)
                0..=19 => {
                    buf.push_str(first);
                    buf.push_str(last);
                    push_adapted_suffix(buf, h);
                }
                // first{sep}last + adapted (15%)
                20..=34 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    push_adapted_suffix(buf, h);
                }
                // first + suffix_word + required (12%)
                35..=46 => {
                    buf.push_str(first);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                    push_required_suffix(buf, h);
                }
                // flast + required (10%)
                47..=56 => {
                    buf.push(f);
                    buf.push_str(last);
                    push_required_suffix(buf, h);
                }
                // last + suffix_word + adapted (8%)
                57..=64 => {
                    buf.push_str(last);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                    push_adapted_suffix(buf, h);
                }
                // first + required (8%)
                65..=72 => {
                    buf.push_str(first);
                    push_required_suffix(buf, h);
                }
                // first{sep}l + required (5%)
                73..=77 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push(l);
                    push_required_suffix(buf, h);
                }
                // last + 2-4digit (5%)
                78..=82 => {
                    buf.push_str(last);
                    buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1));
                }
                // first + 2-4digit (5%)
                83..=87 => {
                    buf.push_str(first);
                    buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1));
                }
                // first_only + adapted (5%)
                88..=92 => {
                    buf.push_str(first);
                    push_adapted_suffix(buf, h);
                }
                // last_only + adapted (5%)
                93..=97 => {
                    buf.push_str(last);
                    push_adapted_suffix(buf, h);
                }
                // f{sep}last + adapted (2%)
                _ => {
                    buf.push(f);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    push_adapted_suffix(buf, h);
                }
            }
        }

        // ── TIER 4: long (13+ chars, 15%) ────────────────────
        // Heavy suffix patterns, compound constructions.
        _ => {
            let w = rng.urange(0, 99);
            match w {
                // first{sep}last + required (20%)
                0..=19 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    push_required_suffix(buf, h);
                }
                // firstlast + required (15%)
                20..=34 => {
                    buf.push_str(first);
                    buf.push_str(last);
                    push_required_suffix(buf, h);
                }
                // prefix + first + adapted (12%)
                35..=46 => {
                    let prefix =
                        nickname::PREFIXES[(h.tag % nickname::PREFIXES.len() as u64) as usize];
                    buf.push_str(prefix);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(first);
                    push_adapted_suffix(buf, h);
                }
                // first{sep}last{sep}suffix_word (10%)
                47..=56 => {
                    buf.push_str(first);
                    let sep = pick_sep(rng, h.seps);
                    buf.push_str(sep);
                    buf.push_str(last);
                    buf.push_str(sep);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                }
                // f{sep}last{sep}required (10%)
                57..=66 => {
                    buf.push(f);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    buf.push_str(pick_sep(rng, h.seps));
                    push_required_suffix(buf, h);
                }
                // first{sep}digits (10%)
                67..=76 => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    push_required_suffix(buf, h);
                }
                // last{sep}first + adapted (8%)
                77..=84 => {
                    buf.push_str(last);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(first);
                    push_adapted_suffix(buf, h);
                }
                // first + suffix_word + required (8%)
                85..=92 => {
                    buf.push_str(first);
                    let sfx =
                        nickname::SUFFIXES[(h.tag % nickname::SUFFIXES.len() as u64) as usize];
                    buf.push_str(sfx);
                    push_required_suffix(buf, h);
                }
                // initials + required (4%)
                93..=96 => {
                    buf.push(f);
                    buf.push(l);
                    push_required_suffix(buf, h);
                }
                // first{sep}last + year4 (3%)
                _ => {
                    buf.push_str(first);
                    buf.push_str(pick_sep(rng, h.seps));
                    buf.push_str(last);
                    push_year4(buf, h);
                }
            }
        }
    }
}

// ─── hybrid handle (name + nickname word) ─────────────────────────────

fn gen_hybrid_handle(buf: &mut String, h: &HandleInput<'_>, rng: &mut Rng) {
    let first = h.first;
    let last = h.last;
    let Some(nick) = h.nick else {
        gen_name_handle(buf, h, rng);
        return;
    };

    let w = rng.urange(0, 99);
    match w {
        // first + noun + suffix
        0..=19 => {
            buf.push_str(first);
            buf.push_str(maybe_sep(rng, h.seps));
            buf.push_str(nick.word2);
            push_adapted_suffix(buf, h);
        }
        // noun + first + suffix
        20..=34 => {
            buf.push_str(nick.word1);
            buf.push_str(maybe_sep(rng, h.seps));
            buf.push_str(first);
            push_adapted_suffix(buf, h);
        }
        // adj/word1 + first + suffix
        35..=49 => {
            buf.push_str(nick.word1);
            buf.push_str(first);
            push_adapted_suffix(buf, h);
        }
        // first + word1 + suffix
        50..=64 => {
            buf.push_str(first);
            buf.push_str(nick.word1);
            push_adapted_suffix(buf, h);
        }
        // word2 + last + suffix
        65..=79 => {
            buf.push_str(nick.word2);
            buf.push_str(maybe_sep(rng, h.seps));
            buf.push_str(last);
            push_adapted_suffix(buf, h);
        }
        // first + word2 + year2
        80..=89 => {
            buf.push_str(first);
            buf.push_str(nick.word2);
            push_year2(buf, h);
        }
        // word1 + last + suffix
        _ => {
            buf.push_str(nick.word1);
            buf.push_str(maybe_sep(rng, h.seps));
            buf.push_str(last);
            push_adapted_suffix(buf, h);
        }
    }
}
