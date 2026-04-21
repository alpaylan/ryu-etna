# ryu — Injected Bugs

Total mutations: 1

All variants are patch-based; apply the listed patch to a clean HEAD to reproduce the buggy build. Each `etna/<variant>` branch is a pre-applied snapshot.

## Bug Index

| # | Name | Variant | File | Injection | Fix Commit |
|---|------|---------|------|-----------|------------|
| 1 | format32 sign byte overwrite for small negatives | `format32_sign_overwrite_daf6d4d_1` | `patches/format32_sign_overwrite_daf6d4d_1.patch` | patch | `daf6d4d13969685390878263cb158c46557915ec` |

## Property Mapping

| Variant | Property | Witness(es) |
|---------|----------|-------------|
| `format32_sign_overwrite_daf6d4d_1` | `property_format32_roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero`, `witness_format32_roundtrip_case_positive_small_sanity`, `witness_format64_roundtrip_case_negative_small_sanity` |

## Framework Coverage

| Property | etna | proptest | quickcheck | crabcheck | hegel |
|----------|:----:|:--------:|:----------:|:---------:|:-----:|
| `property_format32_roundtrip` | ✓ | ✓ | ✓ | ✓ | ✓ |
| `property_format64_roundtrip` | ✓ | ✓ | ✓ | ✓ | ✓ |

(`property_format64_roundtrip` is a sanity property exercised by the adapters and one witness; the `daf6d4d` patch does not touch the f64 path, so no variant currently targets it.)

## Bug Details

### 1. format32 sign byte overwrite for small negatives

- **Variant**: `format32_sign_overwrite_daf6d4d_1`
- **Location**: `patches/format32_sign_overwrite_daf6d4d_1.patch` (applies to `src/pretty/mod.rs`)
- **Property**: `property_format32_roundtrip`
- **Witness(es)**: `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero` (positive control: `witness_format32_roundtrip_case_positive_small_sanity`; f64 sanity: `witness_format64_roundtrip_case_negative_small_sanity`)
- **Fix commit**: `daf6d4d13969685390878263cb158c46557915ec` — `Fix pretty negative small f32`
- **Invariant violated**: Formatting a finite `f32` and parsing the result must yield the same bit pattern (sign included).
- **How the mutation triggers**: In `format32`'s leading-zero branch (`-6 < kk && kk <= 0`, i.e. values in `(-1.0, -1e-6) ∪ (1e-6, 1.0)`), the mutation replaces `*result.offset(index) = b'0'` with `*result = b'0'`. For a negative input the sign byte lives at position 0, so the `'0'` write clobbers `'-'` and the written buffer becomes e.g. `"0\0.001234"` (with a NUL at what was the old `'0'` slot) — a string that fails `f32::parse`, flipping `PropertyResult::Pass` to `Fail` for all negative f32 values whose decimal form has 0–5 leading zeros after the point.
