//! ETNA witness tests for ryu.
//!
//! Each `witness_<name>_case_<tag>` is a deterministic `#[test]` that calls
//! the framework-neutral `property_<name>` from `ryu::etna` with frozen
//! inputs. Base HEAD: passes. Under an `etna/<variant>` branch (mutation
//! active): at least one of these must fail.
//!
//! These witnesses are the ground truth for "does the injected bug actually
//! flip the property function". Framework adapters in `src/bin/etna.rs` must
//! also find the bug via random generators, but witnesses are what validate
//! the injection itself.

use ryu::etna::{property_format32_roundtrip, property_format64_roundtrip, PropertyResult};

fn assert_pass(result: PropertyResult, label: &str) {
    match result {
        PropertyResult::Pass | PropertyResult::Discard => {}
        PropertyResult::Fail(m) => panic!("{label}: {m}"),
    }
}

// ---- property_format32_roundtrip ----
//
// The `daf6d4d` bug: in `src/pretty/mod.rs::format32`, the leading-zero branch
// (`-6 < kk && kk <= 0`) writes `*result = b'0'` instead of
// `*result.offset(index) = b'0'`. For a negative input, the sign byte at
// position 0 gets overwritten with `'0'`, producing e.g. "00.001234" where
// "-0.001234" was expected — which parses as `+0.001234`, flipping the sign.
//
// The witness inputs all have `-6 < kk <= 0` (kk in {-5, -4, -3, -2, -1, 0})
// AND are negative, so they hit the mutated path. A few representative cases
// ensure the mutation isn't masked by any single input.

#[test]
fn witness_format32_roundtrip_case_negative_small_e_minus_3() {
    assert_pass(
        property_format32_roundtrip(-1.234e-3_f32),
        "witness_format32_roundtrip_case_negative_small_e_minus_3",
    );
}

#[test]
fn witness_format32_roundtrip_case_negative_small_e_minus_1() {
    assert_pass(
        property_format32_roundtrip(-0.5_f32),
        "witness_format32_roundtrip_case_negative_small_e_minus_1",
    );
}

#[test]
fn witness_format32_roundtrip_case_negative_small_near_zero() {
    assert_pass(
        property_format32_roundtrip(-0.0012345_f32),
        "witness_format32_roundtrip_case_negative_small_near_zero",
    );
}

// Positive control — must keep passing on base AND on the mutation branch.
// If this fails under the mutation, the mutation is broader than advertised
// and the witness needs revisiting.
#[test]
fn witness_format32_roundtrip_case_positive_small_sanity() {
    assert_pass(
        property_format32_roundtrip(1.234e-3_f32),
        "witness_format32_roundtrip_case_positive_small_sanity",
    );
}

// ---- property_format64_roundtrip ----
//
// f64 sanity witness. The `daf6d4d` patch does not touch the f64 path, so this
// should pass on both base and mutation. It exists so the property function is
// exercised by at least one witness (keeps the `document` stage's witness/
// property mapping populated).
#[test]
fn witness_format64_roundtrip_case_negative_small_sanity() {
    assert_pass(
        property_format64_roundtrip(-1.234e-3_f64),
        "witness_format64_roundtrip_case_negative_small_sanity",
    );
}
