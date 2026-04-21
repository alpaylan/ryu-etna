//! ETNA framework-neutral property functions for ryu.
//!
//! Each `property_<name>` is a pure function over concrete, owned inputs that
//! returns `PropertyResult`. The adapters in `src/bin/etna.rs` and the witness
//! tests in `tests/etna_witnesses.rs` both call these functions — the
//! invariant is never re-implemented inside an adapter.

#![allow(missing_docs)]

use crate::Buffer;
use std::borrow::ToOwned;
use std::format;
use std::string::String;

pub enum PropertyResult {
    Pass,
    Fail(String),
    Discard,
}

/// Round-trip: formatting a finite `f32` and parsing the result must yield the
/// same bit pattern, including sign. Detects bugs where the formatted string
/// loses the leading minus (the `daf6d4d` pretty-format regression: for small
/// negative f32 values like `-1.234e-3`, the buggy code overwrites the sign
/// byte with `'0'`, producing `"00.001234"` instead of `"-0.001234"`).
pub fn property_format32_roundtrip(f: f32) -> PropertyResult {
    if !f.is_finite() {
        return PropertyResult::Discard;
    }
    let mut buf = Buffer::new();
    let s = buf.format_finite(f).to_owned();
    match s.parse::<f32>() {
        Ok(parsed) => {
            if parsed.to_bits() == f.to_bits() {
                PropertyResult::Pass
            } else {
                PropertyResult::Fail(format!(
                    "f32 roundtrip mismatch: {} (bits={:#x}) -> {:?} -> {} (bits={:#x})",
                    f,
                    f.to_bits(),
                    s,
                    parsed,
                    parsed.to_bits()
                ))
            }
        }
        Err(e) => PropertyResult::Fail(format!(
            "f32 format {:?} failed to parse back: {}",
            s, e
        )),
    }
}

/// Same round-trip invariant for `f64`. Detects any f64-side regression of the
/// same class (sign loss, mantissa corruption, wrong scientific/decimal
/// boundary) that would make the formatted string parse to a different float.
pub fn property_format64_roundtrip(f: f64) -> PropertyResult {
    if !f.is_finite() {
        return PropertyResult::Discard;
    }
    let mut buf = Buffer::new();
    let s = buf.format_finite(f).to_owned();
    match s.parse::<f64>() {
        Ok(parsed) => {
            if parsed.to_bits() == f.to_bits() {
                PropertyResult::Pass
            } else {
                PropertyResult::Fail(format!(
                    "f64 roundtrip mismatch: {} (bits={:#x}) -> {:?} -> {} (bits={:#x})",
                    f,
                    f.to_bits(),
                    s,
                    parsed,
                    parsed.to_bits()
                ))
            }
        }
        Err(e) => PropertyResult::Fail(format!(
            "f64 format {:?} failed to parse back: {}",
            s, e
        )),
    }
}
