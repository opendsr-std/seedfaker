use std::fmt::Write;

use crate::ctx::GenContext;
use crate::rng::Rng;

fn one_env_var(rng: &mut Rng) -> String {
    match rng.urange(0, 9) {
        0 => {
            // Must preserve RNG order: lower_digit(6), alnum(14), range(1,99)
            let mut s = String::with_capacity(64);
            s.push_str("DATABASE_URL=postgres://");
            rng.push_lower_digit(&mut s, 6);
            s.push(':');
            rng.push_alnum(&mut s, 14);
            let n = rng.range(1, 99);
            let _ = write!(s, "@db-{n}.internal:5432/production");
            s
        }
        1 => {
            let mut s = String::with_capacity(38);
            s.push_str("AWS_ACCESS_KEY_ID=AKIA");
            rng.push_upper_digit(&mut s, 16);
            s
        }
        2 => {
            let mut s = String::with_capacity(62);
            s.push_str("AWS_SECRET_ACCESS_KEY=");
            rng.push_charset(&mut s, super::helpers::charsets::B64_CHARSET, 40);
            s
        }
        3 => {
            let mut s = String::with_capacity(56);
            s.push_str("API_KEY=sk-");
            rng.push_alnum(&mut s, 48);
            s
        }
        4 => {
            // Must preserve RNG order: alnum(16), range(1,20)
            let mut s = String::with_capacity(48);
            s.push_str("REDIS_URL=redis://:");
            rng.push_alnum(&mut s, 16);
            let n = rng.range(1, 20);
            let _ = write!(s, "@cache-{n}.internal:6379");
            s
        }
        5 => {
            let mut s = String::with_capacity(51);
            s.push_str("STRIPE_SECRET_KEY=sk_live_");
            rng.push_alnum(&mut s, 32);
            s
        }
        6 => {
            let mut s = String::with_capacity(50);
            s.push_str("GITHUB_TOKEN=ghp_");
            rng.push_alnum(&mut s, 36);
            s
        }
        7 => {
            let mut s = String::with_capacity(72);
            s.push_str("SENTRY_DSN=https://");
            rng.push_hex(&mut s, 32);
            s.push_str("@o");
            rng.push_digits(&mut s, 6);
            s.push_str(".ingest.sentry.io/");
            rng.push_digits(&mut s, 7);
            s
        }
        8 => {
            let mut s = String::with_capacity(43);
            s.push_str("JWT_SECRET=");
            rng.push_alnum(&mut s, 32);
            s
        }
        _ => {
            let mut s = String::with_capacity(30);
            s.push_str("SMTP_PASSWORD=");
            rng.push_alnum(&mut s, 16);
            s
        }
    }
}

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    match ctx.modifier {
        "multi" => {
            let n = ctx.rng.urange(3, 6);
            let mut lines = Vec::with_capacity(n);
            for _ in 0..n {
                lines.push(one_env_var(&mut ctx.rng));
            }
            buf.push_str(&lines.join("\n"));
        }
        _ => buf.push_str(&one_env_var(&mut ctx.rng)),
    }
}
