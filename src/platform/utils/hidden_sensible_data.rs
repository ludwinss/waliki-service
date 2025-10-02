pub fn last4(s: &str) -> &str {
    let n = s.len();
    if n <= 4 { s } else { &s[n - 4..] }
}
