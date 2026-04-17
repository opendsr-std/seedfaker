// SplitMix64 — fast, deterministic, sufficient for synthetic data.
// Passes BigCrush. Seeding is ~2ns vs ~120ns for ChaCha12.
// Algorithm: https://prng.di.unimi.it/splitmix64.c

/// FNV-1a hash.
fn mix(mut h: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

/// Pre-compute domain hash: `mix(master, domain_bytes)`. Call once per field at startup.
pub fn domain_hash(master: u64, domain: &str) -> u64 {
    mix(master, domain.as_bytes())
}

/// Derive a sub-seed from pre-computed domain hash and record number.
/// Only mixes 8 bytes per call instead of N domain bytes.
pub fn sub_seed(dh: u64, record: u64) -> u64 {
    mix(dh, &record.to_le_bytes())
}

/// Cryptographically random seed from OS entropy.
pub fn random_seed() -> u64 {
    let mut buf = [0u8; 8];
    if getrandom::fill(&mut buf).is_err() {
        // Early boot: getrandom may fail before urandom is ready
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(42u64, |d| d.as_nanos() as u64);
        buf = t.to_le_bytes();
    }
    u64::from_le_bytes(buf)
}

pub struct Rng {
    state: u64,
    record: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed, record: 0 }
    }

    pub fn derive(master: u64, record: u64, domain: &str) -> Self {
        Self { state: sub_seed(domain_hash(master, domain), record), record }
    }

    /// Fast derive from pre-computed domain hash. Use in hot loops.
    pub fn derive_fast(dh: u64, record: u64) -> Self {
        Self { state: sub_seed(dh, record), record }
    }

    /// Current record number (0-based).
    pub fn record(&self) -> u64 {
        self.record
    }

    /// Set the record number (used by library bindings).
    pub fn set_record(&mut self, r: u64) {
        self.record = r;
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform random in `[0, n)`. Lemire's nearly divisionless method.
    #[inline]
    fn bounded(&mut self, n: u64) -> u64 {
        let mut x = self.next_u64();
        let mut m = u128::from(x).wrapping_mul(u128::from(n));
        let mut l = m as u64;
        if l < n {
            let t = n.wrapping_neg() % n;
            while l < t {
                x = self.next_u64();
                m = u128::from(x).wrapping_mul(u128::from(n));
                l = m as u64;
            }
        }
        (m >> 64) as u64
    }

    pub fn choice<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.bounded(items.len() as u64) as usize]
    }

    pub fn maybe(&mut self, p: f64) -> bool {
        ((self.next_u64() >> 11) as f64 / (1u64 << 53) as f64) < p
    }

    pub fn range(&mut self, lo: i64, hi: i64) -> i64 {
        let span = (hi - lo + 1) as u64;
        // bounded(span) < span ≤ i64::MAX, cast always safe
        let offset = self.bounded(span) as i64;
        lo + offset
    }

    /// Zipf-distributed value in `[lo, hi]`. Rank 1 (most frequent) maps to `lo`.
    pub fn zipf_range(&mut self, lo: i64, hi: i64, s: f64) -> i64 {
        let n = (hi - lo + 1) as u64;
        let rank = self.zipf(n, s);
        // rank ∈ [1, n] where n = hi - lo + 1 ≤ i64::MAX
        let offset = (rank - 1) as i64;
        lo + offset
    }

    pub fn urange(&mut self, lo: usize, hi: usize) -> usize {
        let span = (hi - lo + 1) as u64;
        lo + self.bounded(span) as usize
    }

    pub fn digits(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_digits(&mut s, n);
        s
    }

    pub fn alnum(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_alnum(&mut s, n);
        s
    }

    pub fn hex_str(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_hex(&mut s, n);
        s
    }

    pub fn lower(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_lower(&mut s, n);
        s
    }

    pub fn lower_digit(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_lower_digit(&mut s, n);
        s
    }

    pub fn upper(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_upper(&mut s, n);
        s
    }

    pub fn upper_digit(&mut self, n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_upper_digit(&mut s, n);
        s
    }

    pub fn charset_string(&mut self, charset: &[u8], n: usize) -> String {
        let mut s = String::with_capacity(n);
        self.push_charset(&mut s, charset, n);
        s
    }

    pub fn push_digits(&mut self, buf: &mut String, n: usize) {
        buf.reserve(n);
        for _ in 0..n {
            buf.push((b'0' + self.bounded(10) as u8) as char);
        }
    }

    pub fn push_hex(&mut self, buf: &mut String, n: usize) {
        const HEX: &[u8] = b"0123456789abcdef";
        buf.reserve(n);
        for _ in 0..n {
            buf.push(HEX[self.bounded(16) as usize] as char);
        }
    }

    pub fn push_lower(&mut self, buf: &mut String, n: usize) {
        buf.reserve(n);
        for _ in 0..n {
            buf.push((b'a' + self.bounded(26) as u8) as char);
        }
    }

    pub fn push_alnum(&mut self, buf: &mut String, n: usize) {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        buf.reserve(n);
        for _ in 0..n {
            buf.push(CHARS[self.bounded(CHARS.len() as u64) as usize] as char);
        }
    }

    pub fn push_upper(&mut self, buf: &mut String, n: usize) {
        buf.reserve(n);
        for _ in 0..n {
            buf.push((b'A' + self.bounded(26) as u8) as char);
        }
    }

    pub fn push_upper_digit(&mut self, buf: &mut String, n: usize) {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        buf.reserve(n);
        for _ in 0..n {
            buf.push(CHARS[self.bounded(CHARS.len() as u64) as usize] as char);
        }
    }

    pub fn push_lower_digit(&mut self, buf: &mut String, n: usize) {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        buf.reserve(n);
        for _ in 0..n {
            buf.push(CHARS[self.bounded(CHARS.len() as u64) as usize] as char);
        }
    }

    pub fn push_charset(&mut self, buf: &mut String, charset: &[u8], n: usize) {
        buf.reserve(n);
        for _ in 0..n {
            buf.push(charset[self.bounded(charset.len() as u64) as usize] as char);
        }
    }

    /// Zipf-distributed random integer in `[1, n]` with exponent `s`.
    /// Inverse CDF via binary search over precomputed prefix sums.
    /// `s` > 0; typical values 0.5–2.0; default 1.0.
    pub fn zipf(&mut self, n: u64, s: f64) -> u64 {
        debug_assert!(n >= 1);
        debug_assert!(s > 0.0);
        if n == 1 {
            return 1;
        }
        // For small n, precompute CDF and binary search.
        // For large n, use rejection-inversion.
        if n <= 65536 {
            self.zipf_cdf(n, s)
        } else {
            self.zipf_reject(n, s)
        }
    }

    /// Same draw + binary search as the tail of `zipf_cdf`, but on a CDF the
    /// caller built once. Lets per-FK-column callers amortize the O(n) setup
    /// across millions of samples — that setup (N `powf` + alloc per call) is the
    /// dominant cost of `zipf` when called per row.
    pub fn zipf_from_cdf(&mut self, cum: &[f64]) -> u64 {
        // Empty CDF is a programming error in the caller; the sensible value
        // for a 1-element domain is 1, matching `zipf(n=1, _)`.
        let Some(&total) = cum.last() else { return 1 };
        let target = self.next_f64() * total;
        match cum.partition_point(|&c| c < target) {
            i if i < cum.len() => (i + 1) as u64,
            _ => cum.len() as u64,
        }
    }

    /// Inverse CDF via cumulative weights. O(n) setup, O(log n) sample.
    fn zipf_cdf(&mut self, n: u64, s: f64) -> u64 {
        // Build cumulative weight array
        let n_usize = n as usize;
        let mut cum = Vec::with_capacity(n_usize);
        let mut total = 0.0_f64;
        for k in 1..=n_usize {
            total += 1.0 / (k as f64).powf(s);
            cum.push(total);
        }
        let target = self.next_f64() * total;
        // Binary search for the rank
        match cum.partition_point(|&c| c < target) {
            i if i < n_usize => (i + 1) as u64,
            _ => n,
        }
    }

    /// Rejection-inversion for large n. Avoids O(n) memory.
    fn zipf_reject(&mut self, n: u64, s: f64) -> u64 {
        let n_f = n as f64;
        let one_minus_s = 1.0 - s;
        if one_minus_s.abs() < 1e-9 {
            // s ≈ 1: H(x) = ln(x)
            let hx0 = (n_f + 0.5).ln();
            loop {
                let u = self.next_f64();
                let x = (u * hx0).exp();
                let k = (x + 0.5).floor().max(1.0).min(n_f);
                let accept = 1.0 / k;
                let proposal = 1.0 / x;
                if self.next_f64() * proposal <= accept {
                    return k as u64;
                }
            }
        } else {
            let h = |x: f64| (x.powf(one_minus_s) - 1.0) / one_minus_s;
            let h_inv = |y: f64| (y * one_minus_s + 1.0).powf(1.0 / one_minus_s);
            let hx0 = h(n_f + 0.5);
            let h05 = h(0.5);
            loop {
                let u = self.next_f64();
                let x = h_inv(u * (hx0 - h05) + h05);
                let k = (x + 0.5).floor().max(1.0).min(n_f);
                let accept = k.powf(-s);
                let proposal = if (x - k).abs() < 1e-10 {
                    accept
                } else {
                    (h(k + 0.5) - h(k - 0.5)).max(1e-30)
                };
                if self.next_f64() * proposal <= accept {
                    return k as u64;
                }
            }
        }
    }

    /// Uniform f64 in [0, 1).
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    pub fn sample<T: Clone>(&mut self, items: &[T], k: usize) -> Vec<T> {
        let mut indices: Vec<usize> = (0..items.len()).collect();
        let k = k.min(items.len());
        for i in 0..k {
            let j = i + self.bounded((items.len() - i) as u64) as usize;
            indices.swap(i, j);
        }
        indices[..k].iter().map(|&i| items[i].clone()).collect()
    }
}
