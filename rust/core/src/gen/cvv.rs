use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let arr = [3usize, 3, 3, 4];
    let len = arr[ctx.rng.urange(0, arr.len() - 1)];
    ctx.rng.push_digits(buf, len);
}
