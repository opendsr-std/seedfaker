use crate::field::Ordering;

/// Map record number monotonically into `[min, max]` with deterministic jitter.
///
/// `step = max(1, range / 1_000_000)` — derived from range, independent of `-n`.
/// 1M records = full coverage of range. Less = partial. More = clamp at boundary.
///
/// Jitter is positive-only `[0, step/4)` — guarantees strict monotonicity.
/// Clamp at min/max — no wrap-around.
///
/// Integer example: `asc:1..100` → step=1 → 1, 2, 3, ..., 100, 100, 100
/// Timestamp example: `asc` with 1-year range → step=31s → 31s between records
pub fn monotonic_value(record: u64, tag: u64, min: i64, max: i64, ordering: Ordering) -> i64 {
    if ordering == Ordering::None || min >= max {
        return min;
    }
    let range = (max - min) as u128;
    let step = (range / 1_000_000).max(1);
    let jitter_max = (step / 4).max(1) as u64;
    let jitter = u128::from(tag % jitter_max); // positive only → strict monotonicity
    let pos = u128::from(record) * step + jitter;

    let v = match ordering {
        Ordering::Asc => min as u128 + pos.min(range),
        Ordering::Desc => max as u128 - pos.min(range),
        Ordering::None => min as u128,
    };
    (v as i64).clamp(min, max)
}
