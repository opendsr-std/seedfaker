use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let loc = ctx.locale();
    let (first, last) = if let Some(id) = ctx.identity {
        (id.first_name.as_str(), id.last_name.as_str())
    } else {
        (
            loc.first_names[ctx.rng.urange(0, loc.first_names.len() - 1)],
            loc.last_names[ctx.rng.urange(0, loc.last_names.len() - 1)],
        )
    };
    let arr = ["Engineering", "Marketing", "Finance", "HR", "Operations", "Legal"];
    let ou = arr[ctx.rng.urange(0, arr.len() - 1)];
    let arr = ["corp", "internal", "company"];
    let dc = arr[ctx.rng.urange(0, arr.len() - 1)];
    buf.reserve(3 + first.len() + 1 + last.len() + 4 + ou.len() + 4 + dc.len() + 7);
    buf.push_str("CN=");
    buf.push_str(first);
    buf.push(' ');
    buf.push_str(last);
    buf.push_str(",OU=");
    buf.push_str(ou);
    buf.push_str(",DC=");
    buf.push_str(dc);
    buf.push_str(",DC=com");
}
