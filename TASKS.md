# ryu — ETNA Tasks

Total tasks: 4

## Task Index

| Task | Variant | Framework | Property | Witness |
|------|---------|-----------|----------|---------|
| 001 | `format32_sign_overwrite_daf6d4d_1` | proptest | `Format32Roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3` |
| 002 | `format32_sign_overwrite_daf6d4d_1` | quickcheck | `Format32Roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3` |
| 003 | `format32_sign_overwrite_daf6d4d_1` | crabcheck | `Format32Roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3` |
| 004 | `format32_sign_overwrite_daf6d4d_1` | hegel | `Format32Roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3` |

## Witness Catalog

- `witness_format32_roundtrip_case_negative_small_e_minus_3` — base passes, variant fails
- `witness_format32_roundtrip_case_negative_small_e_minus_1` — base passes, variant fails
- `witness_format32_roundtrip_case_negative_small_near_zero` — base passes, variant fails
- `witness_format32_roundtrip_case_positive_small_sanity` — base passes, variant fails
- `witness_format64_roundtrip_case_negative_small_sanity` — base passes, variant fails
