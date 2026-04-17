pub fn base64url_encode(data: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    let mut i = 0;
    while i < data.len() {
        let b0 = u32::from(data[i]);
        let b1 = if i + 1 < data.len() { u32::from(data[i + 1]) } else { 0 };
        let b2 = if i + 2 < data.len() { u32::from(data[i + 2]) } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        let remaining = data.len() - i;
        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if remaining > 1 {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        }
        if remaining > 2 {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        }
        i += 3;
    }
    out
}
