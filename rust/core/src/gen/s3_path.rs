use std::fmt::Write;

use crate::ctx::GenContext;

pub fn gen(ctx: &mut GenContext<'_>, buf: &mut String) {
    let buckets = ["data-lake", "backups", "exports", "analytics", "ml-models", "logs", "assets"];
    let bucket = buckets[ctx.rng.urange(0, buckets.len() - 1)];
    let prefixes = ["raw", "processed", "staging", "prod", "archive"];
    let prefix = prefixes[ctx.rng.urange(0, prefixes.len() - 1)];
    let names = ["data", "export", "snapshot", "dump", "model"];
    let name = names[ctx.rng.urange(0, names.len() - 1)];
    let exts = ["csv", "parquet", "json", "tar.gz"];
    let ext = exts[ctx.rng.urange(0, exts.len() - 1)];
    let y = ctx.rng.range(2023, 2026);
    let m = ctx.rng.range(1, 12);
    let d = ctx.rng.range(1, 28);
    buf.reserve(5 + bucket.len() + 1 + prefix.len() + 1 + 10 + 1 + name.len() + 1 + ext.len());
    buf.push_str("s3://");
    buf.push_str(bucket);
    buf.push('/');
    buf.push_str(prefix);
    let _ = write!(buf, "/{y}-{m:02}-{d:02}/");
    buf.push_str(name);
    buf.push('.');
    buf.push_str(ext);
}
