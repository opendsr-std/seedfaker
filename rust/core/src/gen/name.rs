use std::fmt::Write;

use crate::ctx::{GenContext, Identity};
use crate::locale::NameOrder;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    if let Some(id) = ctx.identity {
        format_identity_name(id, buf);
        return;
    }
    let loc = ctx.locale();
    let first = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
    let last = loc.last_names[ctx.rng.urange(0, loc.last_names.len() - 1)];

    match loc.name_order {
        NameOrder::Western => {
            let _ = write!(buf, "{first} {last}");
        }
        NameOrder::Eastern => {
            let _ = write!(buf, "{last} {first}");
        }
        NameOrder::DoubleSurname => {
            let last2 = loc.last_names[ctx.rng.urange(0, loc.last_names.len() - 1)];
            let _ = write!(buf, "{first} {last} {last2}");
        }
        NameOrder::Patronymic { particle } => {
            let father = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
            let _ = write!(buf, "{first} {particle} {father} {last}");
        }
        NameOrder::PatronymicMiddle => {
            let father = loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)];
            let _ = write!(buf, "{first} {father} {last}");
        }
    }
}

fn format_identity_name(id: &Identity, buf: &mut String) {
    match id.name_order {
        NameOrder::Western => {
            let _ = write!(buf, "{} {}", id.first_name, id.last_name);
        }
        NameOrder::Eastern => {
            let _ = write!(buf, "{} {}", id.last_name, id.first_name);
        }
        NameOrder::DoubleSurname => {
            if id.last_name2.is_empty() {
                let _ = write!(buf, "{} {}", id.first_name, id.last_name);
            } else {
                let _ = write!(buf, "{} {} {}", id.first_name, id.last_name, id.last_name2);
            }
        }
        NameOrder::Patronymic { particle } => {
            if id.last_name2.is_empty() {
                let _ = write!(buf, "{} {}", id.first_name, id.last_name);
            } else {
                let _ = write!(
                    buf,
                    "{} {} {} {}",
                    id.first_name, particle, id.last_name2, id.last_name
                );
            }
        }
        NameOrder::PatronymicMiddle => {
            if id.last_name2.is_empty() {
                let _ = write!(buf, "{} {}", id.first_name, id.last_name);
            } else {
                let _ = write!(buf, "{} {} {}", id.first_name, id.last_name2, id.last_name);
            }
        }
    }
}
