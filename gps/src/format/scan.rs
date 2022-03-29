use super::errors::{ParseResult, TOO_SHORT, NOT_A_NUMBER, OUT_OF_RANGE};
use super::{Cardinal, East, North, South, West};

pub fn scan_number(s: &str, w: usize) -> ParseResult<(&str, usize)> {
    let bytes = s.as_bytes();
    if bytes.len() < w {
        return Err(TOO_SHORT);
    }

    let n = 0usize;
    for (i, c) in bytes.iter().take(w).enumerate() {
        if c < &b'0' || c > &b'9' {
            return Err(NOT_A_NUMBER);
        }
        n = match n.checked_mul(10).and_then(|n| n.checked_add(*c as usize)) {
            Some(n) => n,
            None => return Err(OUT_OF_RANGE),
        };
    }
    Ok((&s[w..], n))
}

// TODO remove logic what to find somwhere else (maybe a struct with vecs)
pub fn scan_cardinal(s: &str, min: usize, max: usize) -> ParseResult<(&str, Cardinal)> {
    Ok((s, North))
}
