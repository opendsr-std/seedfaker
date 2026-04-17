use crate::ctx::GenContext;

use super::date::push_pad2;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let (y, m, d) = if let Some(id) = ctx.identity {
        (id.birth_year, i64::from(id.birth_month), i64::from(id.birth_day))
    } else {
        let year = if let Some((from, to)) = ctx.range {
            let yf = crate::temporal::epoch_to_year(from);
            let yt = crate::temporal::epoch_to_year(to.saturating_sub(1)).max(yf);
            ctx.rng.range(yf, yt)
        } else {
            let yf = crate::temporal::epoch_to_year(ctx.since);
            let yt = crate::temporal::epoch_to_year(ctx.until.saturating_sub(1)).max(yf);
            crate::ctx::weighted_birth_year(&mut ctx.rng, yf, yt)
        };
        (year, ctx.rng.range(1, 12), ctx.rng.range(1, 28))
    };
    ctx.numeric = Some(crate::temporal::date_to_epoch(y, m, d, 0, 0, 0) as f64);
    let mut ib = itoa::Buffer::new();
    match ctx.modifier {
        "us" => {
            push_pad2(buf, m);
            buf.push('/');
            push_pad2(buf, d);
            buf.push('/');
            buf.push_str(ib.format(y));
        }
        "eu" => {
            push_pad2(buf, d);
            buf.push('.');
            push_pad2(buf, m);
            buf.push('.');
            buf.push_str(ib.format(y));
        }
        _ => {
            buf.push_str(ib.format(y));
            buf.push('-');
            push_pad2(buf, m);
            buf.push('-');
            push_pad2(buf, d);
        }
    }
}
