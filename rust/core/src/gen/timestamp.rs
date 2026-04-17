use crate::ctx::GenContext;
use crate::field::Ordering;
use crate::gen::helpers::handle::unique_tag;

use super::date::push_pad2;
use super::helpers::monotonic::monotonic_value;

const MONTHS: &[&str] =
    &["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/// Convert epoch seconds to (year, month 1-12, day 1-31, hour, min, sec).
/// Uses Howard Hinnant's `civil_from_days` algorithm (exact, no approximation).
pub fn epoch_to_parts(epoch: i64) -> (i64, i64, i64, i64, i64, i64) {
    let day_secs = epoch.rem_euclid(86400);
    let sec = day_secs % 60;
    let min = (day_secs / 60) % 60;
    let hour = day_secs / 3600;
    let z = epoch.div_euclid(86400) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d, hour, min, sec)
}

pub fn raw(ctx: &mut GenContext<'_>) -> f64 {
    // since/until are already epoch seconds
    let (epoch_from, epoch_to) = ctx.range.unwrap_or((ctx.since.max(0), ctx.until));

    // Generate epoch seconds — monotonic or random
    let epoch = if matches!(ctx.ordering, Ordering::Asc | Ordering::Desc) {
        let tag = unique_tag(ctx.rng.record(), 0x71AE);
        monotonic_value(ctx.rng.record(), tag, epoch_from, epoch_to, ctx.ordering)
    } else {
        ctx.rng.range(epoch_from, epoch_to)
    };

    epoch as f64
}

pub fn fmt(v: f64, ctx: &mut GenContext<'_>, buf: &mut String) {
    let epoch = v as i64;

    match ctx.modifier {
        "unix" => {
            buf.push_str(itoa::Buffer::new().format(epoch));
        }
        "ms" => {
            // 13-digit millisecond timestamp (e.g. 1711468800123)
            let ms = epoch * 1000 + ctx.rng.range(0, 999);
            buf.push_str(itoa::Buffer::new().format(ms));
        }
        "log" => {
            let (year, month, day, hour, min, sec) = epoch_to_parts(epoch);
            let month_idx = (month - 1) as usize;
            push_pad2(buf, day);
            buf.push('/');
            buf.push_str(MONTHS[month_idx.min(11)]);
            buf.push('/');
            buf.push_str(itoa::Buffer::new().format(year));
            buf.push(':');
            push_pad2(buf, hour);
            buf.push(':');
            push_pad2(buf, min);
            buf.push(':');
            push_pad2(buf, sec);
            buf.push(' ');
            ctx.tz_log(buf);
        }
        _ => {
            let (year, month, day, hour, min, sec) = epoch_to_parts(epoch);
            buf.push_str(itoa::Buffer::new().format(year));
            buf.push('-');
            push_pad2(buf, month);
            buf.push('-');
            push_pad2(buf, day);
            buf.push('T');
            push_pad2(buf, hour);
            buf.push(':');
            push_pad2(buf, min);
            buf.push(':');
            push_pad2(buf, sec);
            ctx.tz_iso(buf);
        }
    }
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let v = raw(ctx);
    ctx.numeric = Some(v);
    fmt(v, ctx, buf);
}
