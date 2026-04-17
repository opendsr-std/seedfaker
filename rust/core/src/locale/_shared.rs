//! Shared data accessible from all locales.
//!
//! Global email providers and international names that cross locale boundaries.
//! A Russian user may have gmail.com; a German startup may hire a Leon or Sofia.

use crate::rng::Rng;

// ─── global email providers ──────────────────────────────────────────
//
// Ordered by global market share. First COMMON items are selected 70% of the time.

/// Global email providers used worldwide regardless of locale.
pub const GLOBAL_PROVIDERS: &[&str] = &[
    // ── common (top providers, ~70% of global email traffic) ──
    "gmail.com",
    "yahoo.com",
    "outlook.com",
    "hotmail.com",
    "icloud.com",
    // ── less common but still global ──
    "protonmail.com",
    "live.com",
    "aol.com",
    "zoho.com",
    "mail.com",
    "gmx.com",
    "fastmail.com",
    "tutanota.com",
    "hey.com",
    "yandex.com",
    "msn.com",
    "me.com",
    "mac.com",
    "inbox.com",
    "pm.me",
];

/// First 5 are top-tier (gmail/yahoo/outlook/hotmail/icloud).
pub const GLOBAL_PROVIDERS_COMMON: usize = 5;

// ─── international first names ───────────────────────────────────────
//
// Names that appear across many cultures: Alex in Russia, Sofia in Japan,
// Leon in Germany, Maria in every country. Used for ~5% cross-locale mixing.

pub const INTL_FIRST_NAMES: &[&str] = &[
    "Alex", "Max", "Leon", "Leo", "Lucas", "Liam", "Noah", "Oliver", "Adam", "Oscar", "Daniel",
    "David", "Thomas", "Victor", "Roman", "Felix", "Simon", "Martin", "Mark", "Sofia", "Maria",
    "Anna", "Emma", "Mia", "Eva", "Sara", "Maya", "Nina", "Diana", "Elena", "Julia", "Sophia",
    "Lara", "Alisa", "Laura", "Clara", "Luna", "Amelia",
];

// ─── personal / custom domain TLDs ───────────────────────────────────
//
// ~3% of emails use personal domains: firstname.com, surname.dev, etc.
// These TLDs are used to construct `name@name.TLD` style addresses.

pub const PERSONAL_TLDS: &[&str] = &[
    "com", "io", "dev", "me", "co", "net", "org", "xyz", "tech", "pro", "cc", "name", "one", "us",
    "uk", "de", "fr", "nl", "se",
];

// ─── weighted selection ──────────────────────────────────────────────

/// Pick from array with frequency bias toward first `common` items.
/// 70% chance from common tier, 30% from full array.
/// If `common == 0` — uniform selection.
#[inline]
pub fn weighted_choice<'a>(rng: &mut Rng, items: &'a [&'a str], common: usize) -> &'a str {
    if common > 0 && common < items.len() && rng.urange(0, 99) < 70 {
        items[rng.urange(0, common - 1)]
    } else {
        items[rng.urange(0, items.len() - 1)]
    }
}
