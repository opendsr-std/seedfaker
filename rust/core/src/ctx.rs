use crate::gen::helpers::handle::{pick_archetype, unique_tag, HandleArchetype};
use crate::gen::helpers::nickname::{build_nickname, Nickname};
use crate::gen::{ascii_lower, pick_locale};
use crate::locale::shared;
use crate::locale::{Locale, NameOrder};
use crate::rng::Rng;

/// Shared identity for context-aware generation (--ctx strict/loose).
pub struct Identity {
    pub locale_code: &'static str,
    pub name_order: NameOrder,
    pub first_name: String,
    pub last_name: String,
    pub last_name2: String,
    pub first_ascii: String,
    pub last_ascii: String,
    pub archetype: HandleArchetype,
    /// Birth year for tag correlation (nicknames, birthdate field).
    pub birth_year: i64,
    /// Birth month (1-12) — used for realistic numeric suffixes (DDMM/MMDD).
    pub birth_month: u8,
    /// Birth day (1-28) — used for realistic numeric suffixes (DDMM/MMDD).
    pub birth_day: u8,
    /// Pre-computed nickname. None for `NameOnly` archetype.
    pub nickname: Option<Nickname>,
    pub city: String,
    pub region: String,
    pub postal: String,
    pub lat: f64,
    pub lon: f64,
    pub tz: &'static str,
}

/// Weighted birth year distribution — realistic age pyramid.
/// `ref_year` = `until` (the "present" year for generated data).
pub fn weighted_birth_year(rng: &mut Rng, since: i64, until: i64) -> i64 {
    let ref_year = until;
    // (age_lo, age_hi, weight out of 1000)
    let buckets: [(i64, i64, i64); 8] = [
        (18, 25, 250),
        (26, 35, 250),
        (36, 45, 200),
        (46, 55, 130),
        (56, 65, 80),
        (66, 75, 50),
        (76, 85, 25),
        (86, 100, 15),
    ];
    let roll = rng.range(0, 999);
    let mut acc = 0;
    for (age_lo, age_hi, weight) in buckets {
        acc += weight;
        if roll < acc {
            let birth_hi = (ref_year - age_lo).min(until);
            let birth_lo = (ref_year - age_hi).max(since);
            if birth_lo < birth_hi {
                return rng.range(birth_lo, birth_hi);
            }
        }
    }
    let lo = (ref_year - 40).max(since);
    let hi = (ref_year - 18).min(until);
    if lo < hi {
        rng.range(lo, hi)
    } else {
        since
    }
}

impl Identity {
    pub fn new(
        rng: &mut Rng,
        locales: &[&Locale],
        birth_year_range: Option<(i64, i64)>,
        since: i64,
        until: i64,
    ) -> Self {
        let loc = pick_locale(rng, locales);
        // 5% chance of international first name (Leon in Russia, Sofia in Japan).
        let first_name = if rng.urange(0, 99) < 5 {
            shared::INTL_FIRST_NAMES[rng.urange(0, shared::INTL_FIRST_NAMES.len() - 1)].to_string()
        } else {
            shared::weighted_choice(rng, loc.first_names, loc.first_names_common).to_string()
        };
        let last_name =
            shared::weighted_choice(rng, loc.last_names, loc.last_names_common).to_string();
        let last_name2 = match loc.name_order {
            NameOrder::DoubleSurname => (*rng.choice(loc.last_names)).to_string(),
            NameOrder::Patronymic { .. } | NameOrder::PatronymicMiddle => {
                (*rng.choice(loc.first_names)).to_string()
            }
            _ => String::new(),
        };
        let first_ascii = ascii_lower(rng, &first_name);
        let last_ascii = ascii_lower(rng, &last_name);
        let archetype = pick_archetype(rng);
        let birth_year = if let Some((from, to)) = birth_year_range {
            let yf = crate::temporal::epoch_to_year(from);
            let yt = crate::temporal::epoch_to_year(to.saturating_sub(1)).max(yf);
            rng.range(yf, yt)
        } else {
            let yf = crate::temporal::epoch_to_year(since);
            let yt = crate::temporal::epoch_to_year(until.saturating_sub(1)).max(yf);
            weighted_birth_year(rng, yf, yt)
        };
        let birth_month = rng.range(1, 12) as u8;
        let birth_day = rng.range(1, 28) as u8;
        let nickname = if archetype == HandleArchetype::NameOnly {
            None
        } else {
            let nick_tag = unique_tag(rng.record(), 0xDEAD);
            Some(build_nickname(nick_tag, rng))
        };
        let city = rng.choice(loc.cities);
        Self {
            locale_code: loc.code,
            name_order: loc.name_order,
            first_name,
            last_name,
            last_name2,
            first_ascii,
            last_ascii,
            archetype,
            birth_year,
            birth_month,
            birth_day,
            nickname,
            city: city.name.to_string(),
            region: city.region.to_string(),
            postal: city.postal.to_string(),
            lat: city.lat,
            lon: city.lon,
            tz: city.tz,
        }
    }
}

/// Generation context — replaces globals and rigid `GenFn` params.
pub struct GenContext<'a> {
    pub rng: Rng,
    pub locales: &'a [&'a Locale],
    pub modifier: &'a str,
    pub identity: Option<&'a Identity>,
    pub tz_offset_minutes: i32,
    pub since: i64,
    pub until: i64,
    /// Per-field range override, resolved (both bounds filled).
    pub range: Option<(i64, i64)>,
    /// Monotonic ordering: Asc/Desc use record number for position.
    pub ordering: crate::field::Ordering,
    /// Zipf distribution over the range. None = uniform.
    pub zipf: Option<crate::field::ZipfSpec>,
    /// Raw numeric value — set by numeric generators, read by aggregators.
    pub numeric: Option<f64>,
}

impl<'a> GenContext<'a> {
    /// Locale locked to Identity when ctx active, otherwise random.
    pub fn locale(&mut self) -> &'a Locale {
        assert!(!self.locales.is_empty(), "GenContext requires non-empty locales");
        if let Some(id) = self.identity {
            crate::locale::get(id.locale_code).unwrap_or(self.locales[0])
        } else {
            self.locales[self.rng.urange(0, self.locales.len() - 1)]
        }
    }

    /// Always random locale pick (ignores identity).
    pub fn pick_locale(&mut self) -> &'a Locale {
        assert!(!self.locales.is_empty(), "GenContext requires non-empty locales");
        self.locales[self.rng.urange(0, self.locales.len() - 1)]
    }

    /// TZ offset for Apache log: "+HHMM" / "-HHMM"
    pub fn tz_log(&self, buf: &mut String) {
        let m = self.tz_offset_minutes;
        buf.push(if m < 0 { '-' } else { '+' });
        let abs = m.unsigned_abs();
        super::gen::date::push_pad2(buf, i64::from(abs / 60));
        super::gen::date::push_pad2(buf, i64::from(abs % 60));
    }

    /// TZ offset for ISO 8601: "Z" or "+HH:MM" / "-HH:MM"
    pub fn tz_iso(&self, buf: &mut String) {
        let m = self.tz_offset_minutes;
        if m == 0 {
            buf.push('Z');
            return;
        }
        buf.push(if m < 0 { '-' } else { '+' });
        let abs = m.unsigned_abs();
        super::gen::date::push_pad2(buf, i64::from(abs / 60));
        buf.push(':');
        super::gen::date::push_pad2(buf, i64::from(abs % 60));
    }
}
