# ryu — Injected Bugs

Fast floating point to string conversion (Ryū algorithm) — ETNA workload.

Total mutations: 1

## Bug Index

| # | Variant | Name | Location | Injection | Fix Commit |
|---|---------|------|----------|-----------|------------|
| 1 | `format32_sign_overwrite_daf6d4d_1` | `format32_sign_overwrite` | `src/pretty/mod.rs` | `patch` | `daf6d4d13969685390878263cb158c46557915ec` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `format32_sign_overwrite_daf6d4d_1` | `Format32Roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero`, `witness_format32_roundtrip_case_positive_small_sanity`, `witness_format64_roundtrip_case_negative_small_sanity` |

## Framework Coverage

| Property | proptest | quickcheck | crabcheck | hegel |
|----------|---------:|-----------:|----------:|------:|
| `Format32Roundtrip` | ✓ | ✓ | ✓ | ✓ |

## Bug Details

### 1. format32_sign_overwrite

- **Variant**: `format32_sign_overwrite_daf6d4d_1`
- **Location**: `src/pretty/mod.rs`
- **Property**: `Format32Roundtrip`
- **Witness(es)**:
  - `witness_format32_roundtrip_case_negative_small_e_minus_3`
  - `witness_format32_roundtrip_case_negative_small_e_minus_1`
  - `witness_format32_roundtrip_case_negative_small_near_zero`
  - `witness_format32_roundtrip_case_positive_small_sanity`
  - `witness_format64_roundtrip_case_negative_small_sanity`
- **Source**: Fix pretty negative small f32
  > The `-6 < kk && kk <= 0` branch of `f2s_buffered_n` wrote `'0'` to offset 0 instead of `result.offset(index)` — overwriting the leading sign byte for negative values like `-0.001234` and producing invalid decimal strings that failed to parse back.
- **Fix commit**: `daf6d4d13969685390878263cb158c46557915ec` — Fix pretty negative small f32
- **Invariant violated**: Formatting a finite `f32` and parsing the result must yield the same bit pattern (sign included).
- **How the mutation triggers**: In `format32`'s leading-zero branch (`-6 < kk && kk <= 0`, i.e. values in `(-1.0, -1e-6) ∪ (1e-6, 1.0)`), the mutation replaces `*result.offset(index) = b'0'` with `*result = b'0'`. For a negative input the sign byte lives at position 0, so the `'0'` write clobbers `'-'` and the written buffer becomes e.g. `"0\0.001234"` (with a NUL at what was the old `'0'` slot) — a string that fails `f32::parse`, flipping `PropertyResult::Pass` to `Fail` for all negative f32 values whose decimal form has 0–5 leading zeros after the point.
