pub const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALNUM_SPECIAL: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~+/";
pub const B64_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Parse modifier as exact length (digits:4 → 4) or default to random 8-16.
pub fn primitive_len(modifier: &str, rng: &mut crate::rng::Rng) -> usize {
    if modifier.is_empty() {
        return rng.urange(8, 16);
    }
    modifier.parse().unwrap_or_else(|_| rng.urange(8, 16))
}
