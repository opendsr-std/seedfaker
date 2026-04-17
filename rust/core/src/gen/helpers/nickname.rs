//! Nickname chunk data, generation, and mutation.
//!
//! Three word categories combine into realistic creative handles:
//! - NOUNS: what the user "is" (animals, archetypes, elements, objects)
//! - TOPICS: what the user does (tech, gaming, creative, community)
//! - ADJECTIVES: self-description (intensity, mood, style)
//!
//! Plus PREFIXES and SUFFIXES for mutations and rare patterns.
//!
//! Word selection criteria: all combinations must be brand-safe.
//! Removed words that create offensive/inappropriate pairs.

use crate::rng::Rng;

use super::handle::{
    maybe_sep, push_bday, push_hex3, push_letter_digit, push_suffix, push_year2, push_year4,
    push_zpad3, push_zpad4, HandleInput, NICKNAME_SEPS,
};

// ─── chunk data ─────────────────────────────────────────────────────
//
// Every word was reviewed for pairwise safety: no combination of
// adj+noun, noun+topic, or prefix+noun should produce offensive,
// sexual, violent, or politically charged readings.

pub const NOUNS: &[&str] = &[
    // animals — common gamer/social handles
    "wolf",
    "fox",
    "hawk",
    "raven",
    "cobra",
    "viper",
    "tiger",
    "lynx",
    "bear",
    "owl",
    "crow",
    "shark",
    "panda",
    "falcon",
    "orca",
    "mantis",
    "cat",
    "lion",
    "dragon",
    "panther",
    "jaguar",
    "eagle",
    "serpent",
    "scorpion",
    "spider",
    "bat",
    "rhino",
    "badger",
    "otter",
    "ferret",
    "hyena",
    "jackal",
    "moose",
    "bison",
    "coyote",
    "crane",
    "heron",
    "parrot",
    "beetle",
    "hornet",
    "wasp",
    "gecko",
    "iguana",
    "chameleon",
    "pelican",
    "dove",
    "wren",
    "finch",
    "stag",
    "elk",
    "ram",
    "goat",
    "seal",
    "walrus",
    "squid",
    "octopus",
    "starling",
    "swift",
    // archetypes — gamer / RPG / movie tropes
    "ghost",
    "shadow",
    "phantom",
    "ninja",
    "pirate",
    "knight",
    "scout",
    "ranger",
    "nomad",
    "rebel",
    "rogue",
    "hunter",
    "pilot",
    "hero",
    "wizard",
    "sage",
    "samurai",
    "viking",
    "gladiator",
    "titan",
    "sentinel",
    "warden",
    "oracle",
    "sorcerer",
    "paladin",
    "monk",
    "druid",
    "shaman",
    "bandit",
    "mercenary",
    "sniper",
    "trooper",
    "captain",
    "marshal",
    "reaper",
    "specter",
    "wraith",
    "golem",
    "sphinx",
    "griffin",
    "minotaur",
    "chimera",
    "phoenix",
    "hydra",
    "kraken",
    "leviathan",
    "behemoth",
    "valkyrie",
    "amazon",
    "centurion",
    "corsair",
    "buccaneer",
    "ronin",
    "shogun",
    "duelist",
    "berserker",
    "templar",
    "crusader",
    // elements / nature
    "storm",
    "frost",
    "blaze",
    "flame",
    "ember",
    "spark",
    "thunder",
    "nova",
    "comet",
    "void",
    "flux",
    "pulse",
    "dusk",
    "dawn",
    "rain",
    "ice",
    "mist",
    "ash",
    "dust",
    "gale",
    "breeze",
    "torrent",
    "inferno",
    "glacier",
    "aurora",
    "nebula",
    "quasar",
    "eclipse",
    "solstice",
    "zenith",
    "nadir",
    "vortex",
    "cascade",
    "rapids",
    "tundra",
    "mesa",
    "canyon",
    "reef",
    "coral",
    "delta",
    "summit",
    "ridge",
    "crest",
    "peak",
    "vale",
    "grove",
    "meadow",
    // objects / concepts
    "blade",
    "shield",
    "arrow",
    "bolt",
    "cipher",
    "nexus",
    "vector",
    "apex",
    "onyx",
    "helix",
    "prism",
    "glitch",
    "drift",
    "surge",
    "orbit",
    "marble",
    "crystal",
    "shard",
    "relic",
    "totem",
    "sigil",
    "glyph",
    "rune",
    "scroll",
    "compass",
    "anvil",
    "forge",
    "hammer",
    "dagger",
    "lance",
    "spear",
    "helm",
    "crown",
    "throne",
    "beacon",
    "lantern",
    "torch",
    "mirror",
    "lens",
    "prong",
    "fang",
    "claw",
    "talon",
    "horn",
    "tusk",
    "spine",
    "shell",
    // sci-fi / cyber
    "pixel",
    "neon",
    "cyber",
    "chrome",
    "synth",
    "matrix",
    "grid",
    "node",
    "circuit",
    "warp",
    "quantum",
    "photon",
    "plasma",
    "laser",
    "mech",
    "droid",
    "probe",
    "signal",
    "relay",
    // pop culture safe archetypes
    "ace",
    "maverick",
    "legend",
    "prodigy",
    "enigma",
    "mystic",
    "prophet",
    "vagabond",
    "outlaw",
    "misfit",
    "drifter",
    "loner",
    "stranger",
    "wanderer",
    // gamer / esports classics
    "boss",
    "noob",
    "pwnr",
    "snax",
    "clutch",
    "carry",
    "aggro",
    "tank",
    "healer",
    "caster",
    "spawn",
    "camper",
    "rusher",
    "flanker",
    "tryhard",
    // pop culture safe handles (no trademarked names, just archetypes)
    "admin",
    "root",
    "daemon",
    "kernel",
    "sudo",
    "sysop",
    "hacker",
    "glider",
    "runner",
    "rider",
    "racer",
    "jumper",
    "seeker",
    "finder",
    "maker",
    "builder",
    "keeper",
    "breaker",
    "bender",
    "shifter",
    "spinner",
    // common standalone nick words
    "pain",
    "fury",
    "rage",
    "havoc",
    "chaos",
    "doom",
    "fate",
    "bane",
    "jinx",
    "hex",
    "luck",
    "karma",
    "zen",
    "guru",
    "sensei",
    "master",
    "chief",
    "boss",
    "king",
    "duke",
    "baron",
    "count",
];

pub const TOPICS: &[&str] = &[
    // tech / dev
    "dev", "code", "coder", "byte", "node", "stack", "sys", "ops", "hack", "data", "core", "bit",
    "net", "web", "cloud", "log", "api", "git", "sql", "cli", "ssh", "vim", "nix", "rust", "lua",
    "cpp", "bash", // creative / media
    "art", "ink", "pixel", "craft", "beats", "music", "draws", "film", "photo", "lens", "sketch",
    "paint", "studio", "edit", "mix", "vibe", "sound", "audio", "visual", // gaming
    "game", "plays", "gamer", "quest", "arena", "spawn", "loot", "raid", "tank", "healer", "mage",
    "rogue", "dps", "pvp", "pve", "mmr", "rank", "frag", "combo", "rush", "grind", "farm", "craft",
    "forge", "guild", "party", "boss", "level", "score", "streak",
    // professional / brand
    "pro", "work", "lab", "hub", "hq", "eng", "tech", "build", "maker", "smith", "works", "studio",
    "forge", "shop", "den", "nest", "lair", "vault", "bunker", // community / social
    "bot", "box", "zone", "crew", "clan", "squad", "base", "camp", "pack", "tribe", "gang", "mob",
    "fleet", "army", "legion", "order", "cult", "ring", "circle",
    // streaming / content
    "live", "stream", "cast", "show", "feed", "clip", "reel", "vlog", "pod",
    // keyboard patterns / meme fragments — real people use these
    "qwerty", "qwer", "zaq", "wasd", "asd", "asdf", "zxc", "qaz", "wsx",
    // filler / padding — used as handle suffixes IRL
    "xxx", "xx", "xxxx", "xxy", "zzz", "zz", "zzzz", "aaa", "sss", "ddd", "qqq", "www", "eee",
];

pub const ADJECTIVES: &[&str] = &[
    // intensity
    "mega", "ultra", "super", "hyper", "turbo", "epic", "max", "prime", "omega", "alpha", "sigma",
    "delta", "zero", "infinite", "absolute", "supreme", "grand", "mighty",
    // mood / style
    "dark", "wild", "bold", "chill", "cool", "swift", "slick", "keen", "fierce", "savage", "noble",
    "grand", "grim", "stern", "stoic", "witty", "sly", "sharp", "bright", "vivid", "neon",
    "golden", "silver", "iron", "steel", "copper", "jade", "ruby", "amber", "ivory",
    // size / state
    "tiny", "true", "real", "lone", "lazy", "slim", "fast", "loud", "calm", "deep", "tall", "vast",
    "wide", "long", "flat", "raw", "pure", "clean", "fresh", "crisp", "hot", "cold", "warm", "dry",
    "wet", // internet / gamer flavor
    "xo", "lil", "big", "old", "new", "top", "low", "mid", "high", "mini", "nano", "proto",
    "retro", "neo", "meta", "anti", "semi", "dual", "tri", // vibe / aesthetic
    "chill", "cozy", "hazy", "misty", "dusty", "rusty", "foggy", "smoky", "stormy", "cloudy",
    "sunny", "lunar", "solar", "astral", "cosmic", "arctic", "tropic",
    // gamer intensifiers
    "toxic", "chaotic", "random", "sneaky", "silent", "stealth", "rapid", "instant", "lethal",
    "brutal", "insane", "crazy", "mental", "wicked", "vicious",
    // extra gamer / internet culture
    "noob", "pro", "based", "cringe", "elite", "godlike", "op", "broken", "cursed", "blessed",
    "epic", "rare", "common", "exotic", "mythic",
];

pub const PREFIXES: &[&str] = &[
    "the", "real", "its", "hey", "hi", "im", "just", "not", "only", "my", "mr", "ms", "dr", "sir",
    "captain", "official", "og", "el", "la", "don", "lord", "lady", "king", "queen", "prince",
    "duke", "chief", "general", "agent", "saint",
];

pub const SUFFIXES: &[&str] = &[
    // tech
    "dev", "pro", "hq", "io", "app", "tech", "code", "ops", "eng", "lab", "hub", "box", "net",
    "one", "go", "run", "now", "ai", "js", "py", "web", "rs", "ml", "xyz",
    // platform / streaming
    "gg", "tv", "fm", "live", "vip", // internet slang
    "lol", "wtf", "btw", "ftw", "irl", "afk", "brb", "omg", "tbh", "xd",
    // keyboard / padding
    "xx", "xxx", "xxy", "zz", "zzz", "qq", "aaa", "sss",
];

/// Keyboard/meme suffixes — bare (for `build_nickname` stored value).
const KEYBOARD_BARE: &[&str] = &[
    "xx", "xxx", "xd", "zz", "zzz", "qq", "qqq", "aaa", "sss", "qwerty", "wasd", "asdf", "zxc",
    "qaz", "owo", "uwu", "xoxo", "yolo", "kek", "pog", "gg", "ez",
];

/// Keyboard/meme suffixes — underscore-prefixed (for runtime nick suffix).
const KEYBOARD_SEP: &[&str] = &[
    "_xx", "_xxx", "_xd", "_zz", "_zzz", "_qq", "_qqq", "_aaa", "_sss", "_qwerty", "_wasd",
    "_asdf", "_zxc", "_qaz", "_owo", "_uwu", "_xoxo", "_yolo", "_kek", "_pog", "_gg", "_ez",
];

/// Iconic gamer/meme numbers used in suffixes.
const ICONIC: &[&str] = &[
    "001", "007", "069", "100", "101", "111", "123", "228", "256", "303", "313", "322", "333",
    "360", "404", "420", "444", "500", "512", "555", "616", "666", "699", "707", "777", "800",
    "808", "888", "900", "911", "999", "1337",
];

// ─── Nickname struct ────────────────────────────────────────────────

/// Pre-computed nickname for an identity. Decomposed for efficient mutation.
pub struct Nickname {
    /// Full assembled nickname with tag, e.g. `dark_wolf42`.
    pub full: String,
    /// Length of base (without trailing digits), e.g. 9 for `dark_wolf`.
    pub(super) base_len: usize,
    /// First word (static ref into chunk arrays).
    pub word1: &'static str,
    /// Second word (static ref into chunk arrays).
    pub word2: &'static str,
    /// Separator byte between word1 and word2 (0 = none).
    pub sep: u8,
}

impl Nickname {
    /// Base portion without trailing digits.
    pub fn base(&self) -> &str {
        &self.full[..self.base_len]
    }
}

// ─── nickname length tier ────────────────────────────────────────────
//
// Bell curve centered on 7-10 chars. Minimum 3 chars (rare).
//
//  3-5:   5%  ▓▓           — single word + short suffix: fox42, ace99
//  6-8:  25%  ▓▓▓▓▓▓▓▓▓   — single word + suffix: wolf_007, hawk42
//  9-12: 40%  ▓▓▓▓▓▓▓▓▓▓▓ — 2-word + suffix: darkwolf42 (current default)
// 13-16: 25%  ▓▓▓▓▓▓▓▓▓   — 2-word + prefix/long suffix
//  17+:   5%  ▓▓           — 3-word + suffix: dark_wolf_dev_42

/// Nickname length tier from tag bits. Deterministic per record.
#[inline]
fn nick_length_tier(tag: u64) -> u8 {
    let v = (tag >> 52) % 100;
    match v {
        0..=4 => 0,   //  5% ultra-short (3-5 chars)
        5..=29 => 1,  // 25% short (6-8 chars)
        30..=69 => 2, // 40% medium (9-12 chars)
        70..=94 => 3, // 25% long (13-16 chars)
        _ => 4,       //  5% extra-long (17+ chars)
    }
}

/// Pick a word from any pool (nouns, adjectives, or topics).
/// Used for single-word nicknames — maximizes base space (596 words).
fn pick_any_word(rng: &mut Rng) -> &'static str {
    let total = NOUNS.len() + ADJECTIVES.len() + TOPICS.len();
    let i = rng.urange(0, total - 1);
    if i < NOUNS.len() {
        NOUNS[i]
    } else if i < NOUNS.len() + ADJECTIVES.len() {
        ADJECTIVES[i - NOUNS.len()]
    } else {
        TOPICS[i - NOUNS.len() - ADJECTIVES.len()]
    }
}

// ─── nickname construction ──────────────────────────────────────────

/// Pick a separator for nickname chunks using `NICKNAME_SEPS`.
fn nick_sep(rng: &mut Rng) -> (u8, &'static str) {
    let s = maybe_sep(rng, NICKNAME_SEPS);
    let b = if s.is_empty() { 0 } else { s.as_bytes()[0] };
    (b, s)
}

/// Build a Nickname struct for storage in Identity. Called once per record.
/// Tag-derived suffixes ensure per-record uniqueness.
///
/// Length tier determines word count:
/// - tier 0-1 (30%): single word — `wolf`, `storm`, `dev` (word2 = word1)
/// - tier 2-3 (65%): two words — `dark_wolf`, `storm_code`
/// - tier 4 (5%): three words — `dark_wolf_dev`
///
/// Every nickname always gets a suffix — no bare outputs.
pub fn build_nickname(tag: u64, rng: &mut Rng) -> Nickname {
    let tier = nick_length_tier(tag);
    let noun = NOUNS[rng.urange(0, NOUNS.len() - 1)];

    let (word1, word2, sep_byte, sep_s, word3) = if tier <= 1 {
        // Single-word nickname: word2 = word1 for mutation compat.
        // Tier 0 picks from all pools (596 words), tier 1 from nouns (336).
        let w = if tier == 0 { pick_any_word(rng) } else { noun };
        (w, w, 0u8, "", None)
    } else if tier <= 3 {
        // Two-word patterns (current default behavior)
        let w = rng.urange(0, 49);
        match w {
            // adj + noun (most common)
            0..=19 => {
                let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
                let (sb, ss) = nick_sep(rng);
                (adj, noun, sb, ss, None)
            }
            // noun + topic
            20..=32 => {
                let topic = TOPICS[rng.urange(0, TOPICS.len() - 1)];
                let (sb, ss) = nick_sep(rng);
                (noun, topic, sb, ss, None)
            }
            // noun + noun
            33..=40 => {
                let noun2 = NOUNS[rng.urange(0, NOUNS.len() - 1)];
                let (sb, ss) = nick_sep(rng);
                (noun, noun2, sb, ss, None)
            }
            // adj + topic
            _ => {
                let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
                let topic = TOPICS[rng.urange(0, TOPICS.len() - 1)];
                let (sb, ss) = nick_sep(rng);
                (adj, topic, sb, ss, None)
            }
        }
    } else {
        // 3-word: adj + noun + topic
        let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
        let topic = TOPICS[rng.urange(0, TOPICS.len() - 1)];
        let (sb, ss) = nick_sep(rng);
        (adj, noun, sb, ss, Some(topic))
    };

    let mut full = String::with_capacity(word1.len() + 1 + word2.len() + 8 + 6);
    full.push_str(word1);
    // Single-word nicks (tier 0-1): word1 == word2, don't duplicate
    if !std::ptr::eq(word1, word2) {
        full.push_str(sep_s);
        full.push_str(word2);
    }
    if let Some(w3) = word3 {
        full.push_str(sep_s);
        full.push_str(w3);
    }
    let base_len = full.len();

    // Suffix distribution — every path produces a non-empty suffix.
    // Removed: bare (caused collisions). Year4 restored at low weight (4%).
    let tw = rng.urange(0, 99);
    match tw {
        0..=17 => {
            // 18% year-like 2-digit (tag-derived, 00-99)
            let y = tag % 100;
            if y < 10 {
                full.push('0');
            }
            full.push_str(itoa::Buffer::new().format(y));
        }
        18..=21 => {
            // 4% year4 (1975-2024) — darkwolf1994
            full.push_str(itoa::Buffer::new().format(tag % 50 + 1975));
        }
        22..=44 => {
            // 23% tag-derived 2-4 digits — high cardinality
            full.push_str(itoa::Buffer::new().format(tag % 9000 + 1));
        }
        45..=54 => {
            // 10% birthday-like DDMM (tag-derived)
            let d = tag % 28 + 1;
            let m = (tag >> 5) % 12 + 1;
            let mut ib = itoa::Buffer::new();
            if d < 10 {
                full.push('0');
            }
            full.push_str(ib.format(d));
            if m < 10 {
                full.push('0');
            }
            full.push_str(ib.format(m));
        }
        55..=69 => {
            // 15% 2-digit (tag-derived, 10-99)
            full.push_str(itoa::Buffer::new().format(tag % 90 + 10));
        }
        70..=79 => {
            // 10% 3-digit tag (100-999)
            full.push_str(itoa::Buffer::new().format(tag % 900 + 100));
        }
        80..=84 => {
            // 5% iconic gamer numbers
            full.push_str(ICONIC[(tag % ICONIC.len() as u64) as usize]);
        }
        85..=89 => {
            // 5% zero-padded (007, 042, 001)
            let v = tag % 999 + 1;
            if v < 10 {
                full.push_str("00");
            } else if v < 100 {
                full.push('0');
            }
            full.push_str(itoa::Buffer::new().format(v));
        }
        90..=94 => {
            // 5% keyboard/meme suffix
            full.push_str(KEYBOARD_BARE[(tag % KEYBOARD_BARE.len() as u64) as usize]);
        }
        _ => {
            // 5% hex tag (4096 values)
            let h3 = tag & 0xFFF;
            const HEX: &[u8; 16] = b"0123456789abcdef";
            full.push(HEX[((h3 >> 8) & 0xF) as usize] as char);
            full.push(HEX[((h3 >> 4) & 0xF) as usize] as char);
            full.push(HEX[(h3 & 0xF) as usize] as char);
        }
    }

    Nickname { full, base_len, word1, word2, sep: sep_byte }
}

// ─── nickname generation (fresh, not from identity) ─────────────────

/// Generate a fresh nickname into `buf`. Length-tier controlled for realistic
/// distribution: peak at 7-10 chars, minimum 3 chars, bell curve tail to 17+.
pub fn gen_nickname(buf: &mut String, h: &HandleInput<'_>, rng: &mut Rng) {
    match nick_length_tier(h.tag) {
        // ── TIER 0: ultra-short 3-5 chars (5%) ──────────────
        // Single word from any pool + short suffix: `fox42`, `dev99`, `ace07`
        0 => {
            let word = pick_any_word(rng);
            buf.push_str(word);
            // High-entropy suffix to compensate small base space (596 words)
            buf.push_str(itoa::Buffer::new().format(h.tag % 90 + 10));
        }

        // ── TIER 1: short 6-8 chars (25%) ───────────────────
        // Single noun + diverse suffix: `wolf_007`, `hawk42`, `doom_99`
        1 => {
            let noun = NOUNS[rng.urange(0, NOUNS.len() - 1)];
            buf.push_str(noun);
            push_nick_suffix(buf, h, rng);
        }

        // ── TIER 2: medium 9-12 chars (40%) ─────────────────
        // Mix of single-word-with-long-suffix and 2-word-with-short-suffix.
        // Single-word: `wolf_007_42`, `storm1337` = 8-12 chars.
        // Two-word: `darkfox42`, `stormdev99` = 9-13 chars.
        2 => {
            let noun = NOUNS[rng.urange(0, NOUNS.len() - 1)];
            let w = rng.urange(0, 99);
            match w {
                // noun + suffix_word + short suffix (20%) — stormdev42, hawkcode99
                0..=19 => {
                    let sfx = SUFFIXES[(h.tag % SUFFIXES.len() as u64) as usize];
                    buf.push_str(noun);
                    buf.push_str(sfx);
                }
                // adj + noun (25%)
                20..=44 => {
                    let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
                    buf.push_str(adj);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(noun);
                }
                // noun + topic (20%)
                45..=64 => {
                    let topic = TOPICS[rng.urange(0, TOPICS.len() - 1)];
                    buf.push_str(noun);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(topic);
                }
                // noun + noun (15%)
                65..=79 => {
                    let noun2 = NOUNS[rng.urange(0, NOUNS.len() - 1)];
                    buf.push_str(noun);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(noun2);
                }
                // single noun (20%) — longer suffix compensates
                _ => {
                    buf.push_str(noun);
                }
            }
            push_nick_suffix(buf, h, rng);
        }

        // ── TIER 3: long 13-16 chars (25%) ──────────────────
        // 2-word + prefix or longer suffix patterns
        3 => {
            let noun = NOUNS[rng.urange(0, NOUNS.len() - 1)];
            let w = rng.urange(0, 99);
            match w {
                // prefix + adj + noun (30%)
                0..=29 => {
                    let prefix = PREFIXES[(h.tag % PREFIXES.len() as u64) as usize];
                    buf.push_str(prefix);
                    buf.push_str(maybe_sep(rng, h.seps));
                    let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
                    buf.push_str(adj);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(noun);
                }
                // adj + noun (40%)
                30..=69 => {
                    let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
                    buf.push_str(adj);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(noun);
                }
                // noun + topic (30%)
                _ => {
                    let topic = TOPICS[rng.urange(0, TOPICS.len() - 1)];
                    buf.push_str(noun);
                    buf.push_str(maybe_sep(rng, h.seps));
                    buf.push_str(topic);
                }
            }
            push_nick_suffix(buf, h, rng);
        }

        // ── TIER 4: extra-long 17+ chars (5%) ───────────────
        // 3-word: adj + noun + topic + suffix
        _ => {
            let noun = NOUNS[rng.urange(0, NOUNS.len() - 1)];
            let adj = ADJECTIVES[rng.urange(0, ADJECTIVES.len() - 1)];
            let topic = TOPICS[rng.urange(0, TOPICS.len() - 1)];
            let sep = maybe_sep(rng, h.seps);
            buf.push_str(adj);
            buf.push_str(sep);
            buf.push_str(noun);
            buf.push_str(sep);
            buf.push_str(topic);
            push_nick_suffix(buf, h, rng);
        }
    }
}

/// Push a suffix appropriate for nicknames.
/// Always produces a non-empty suffix — bare handles removed for collision safety.
///
/// 20 distinct suffix types across three families:
/// - bare digits (40%): `42`, `94`, `742`, `3847`, `0512`, `007`
/// - underscore-prefixed (35%): `_42`, `_742`, `_007`, `_0042`, `_3847`
/// - mixed (25%): `1337`, `gg`, `x7`, `a3f`
///
/// Underscore-prefixed and bare are distinct output strings (`wolf42` ≠ `wolf_42`),
/// effectively doubling the slot space for shared cardinality ranges.
fn push_nick_suffix(buf: &mut String, h: &HandleInput<'_>, rng: &mut Rng) {
    let w = rng.urange(0, 99);
    match w {
        // ── bare digits (40%) ─────────────────────────────────
        0..=7 => push_year2(buf, h), //  8% year2 (100)
        8..=16 => buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1)), //  9% 2-4digit (9000)
        17..=19 => push_year4(buf, h), //  3% year4 (50) — darkwolf1994
        20..=24 => push_bday(buf, h), //  5% bday (336)
        25..=29 => buf.push_str(itoa::Buffer::new().format(h.tag % 900 + 100)), //  5% 3digit (900)
        30..=34 => buf.push_str(itoa::Buffer::new().format(h.tag % 90 + 10)), //  5% 2digit (90)
        35..=39 => push_zpad3(buf, h.tag), //  5% 007-style (999)

        // ── underscore-prefixed (35%) ─────────────────────────
        40..=49 => {
            buf.push('_');
            buf.push_str(itoa::Buffer::new().format(h.tag % 9000 + 1));
        } // 10% _3847 (9000)
        50..=54 => {
            buf.push('_');
            push_zpad3(buf, h.tag);
        } //  5% _007 (999)
        55..=59 => {
            buf.push('_');
            buf.push_str(itoa::Buffer::new().format(h.tag % 900 + 100));
        } //  5% _742 (900)
        60..=64 => {
            buf.push('_');
            buf.push_str(itoa::Buffer::new().format(h.tag % 90 + 10));
        } //  5% _42 (90)
        65..=69 => {
            buf.push('_');
            push_zpad4(buf, h.tag);
        } //  5% _0042 (9999)
        70..=74 => {
            buf.push('_');
            push_year2(buf, h);
        } //  5% _94 (100)

        // ── mixed (25%) ───────────────────────────────────────
        75..=79 => push_nick_iconic(buf, h.tag), //  5% 1337/420/666 (32)
        80..=84 => push_nick_keyboard(buf, h.tag), //  5% gg/xd/uwu (30)
        85..=89 => push_letter_digit(buf, h.tag), //  5% x7/k3 (260)
        90..=94 => push_hex3(buf, h.tag),        //  5% a3f (4096)
        _ => push_suffix(buf, h, rng),           //  5% general mix
    }
}

/// Iconic gamer numbers: `1337`, `420`, `666`, `007`, etc.
fn push_nick_iconic(buf: &mut String, tag: u64) {
    buf.push_str(ICONIC[(tag % ICONIC.len() as u64) as usize]);
}

/// Keyboard/meme suffixes: `_gg`, `_xd`, `_uwu`, `_qwerty`, etc.
/// Underscore-prefixed to avoid unreadable concatenation (`pwnrzxc` → `pwnr_zxc`).
fn push_nick_keyboard(buf: &mut String, tag: u64) {
    buf.push_str(KEYBOARD_SEP[(tag % KEYBOARD_SEP.len() as u64) as usize]);
}

// ─── mutations ──────────────────────────────────────────────────────

/// Mutate an existing nickname into `buf`. The result looks like a variation
/// of the original — as if the preferred nick was taken and the user adapted.
/// Every path produces a suffix — no bare outputs.
pub fn mutate_nickname(buf: &mut String, nick: &Nickname, h: &HandleInput<'_>, rng: &mut Rng) {
    let w = rng.urange(0, 99);
    match w {
        // Tag change (35%): same base, different number
        0..=34 => {
            buf.push_str(nick.base());
            push_nick_suffix(buf, h, rng);
        }
        // Prefix add (20%)
        35..=54 => {
            let prefix = PREFIXES[rng.urange(0, PREFIXES.len() - 1)];
            buf.push_str(prefix);
            buf.push_str(maybe_sep(rng, h.seps));
            buf.push_str(nick.base());
            push_nick_suffix(buf, h, rng);
        }
        // Sep change (20%) — reconstruct with platform separator
        55..=74 => {
            buf.push_str(nick.word1);
            buf.push_str(maybe_sep(rng, h.seps));
            buf.push_str(nick.word2);
            push_nick_suffix(buf, h, rng);
        }
        // Suffix swap (15%)
        75..=89 => {
            buf.push_str(nick.base());
            buf.push_str(maybe_sep(rng, h.seps));
            let suffix = SUFFIXES[(h.tag % SUFFIXES.len() as u64) as usize];
            buf.push_str(suffix);
            push_nick_suffix(buf, h, rng);
        }
        // Truncate (10%)
        _ => {
            let c1 = nick.word1.as_bytes()[0] as char;
            buf.push(c1);
            buf.push_str(nick.word2);
            push_nick_suffix(buf, h, rng);
        }
    }
}
