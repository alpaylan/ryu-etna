# ryu — ETNA Tasks

Total tasks: 4

ETNA tasks are **mutation/property/witness triplets**. Each row below is one runnable task: the command executes the framework-specific adapter against the buggy variant branch and should report a counterexample.

Run against a variant by first checking out its branch (`git checkout etna/<variant>`) or applying its patch on a clean tree (`git apply patches/<variant>.patch`).

## Task Index

| Task | Variant | Framework | Property | Witness(es) | Command |
|------|---------|-----------|----------|-------------|---------|
| 001 | `format32_sign_overwrite_daf6d4d_1` | proptest | `property_format32_roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero` | `cargo run --release --bin etna -- proptest Format32Roundtrip` |
| 002 | `format32_sign_overwrite_daf6d4d_1` | quickcheck | `property_format32_roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero` | `cargo run --release --bin etna -- quickcheck Format32Roundtrip` |
| 003 | `format32_sign_overwrite_daf6d4d_1` | crabcheck | `property_format32_roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero` | `cargo run --release --bin etna -- crabcheck Format32Roundtrip` |
| 004 | `format32_sign_overwrite_daf6d4d_1` | hegel | `property_format32_roundtrip` | `witness_format32_roundtrip_case_negative_small_e_minus_3`, `witness_format32_roundtrip_case_negative_small_e_minus_1`, `witness_format32_roundtrip_case_negative_small_near_zero` | `cargo run --release --bin etna -- hegel Format32Roundtrip` |

## Witness catalog

Each witness is a deterministic concrete test in `tests/etna_witnesses.rs`. Base build: passes. Variant-active build: fails (for the three negative-f32 witnesses) or passes (the positive control and f64 sanity witness, which stay green under the `daf6d4d` mutation).

- `witness_format32_roundtrip_case_negative_small_e_minus_3` — `property_format32_roundtrip(-1.234e-3_f32)` → `Pass`. Under `format32_sign_overwrite_daf6d4d_1` the formatted string is `"0\0.001234"` (sign byte clobbered), which fails to re-parse, so the property returns `Fail`.
- `witness_format32_roundtrip_case_negative_small_e_minus_1` — `property_format32_roundtrip(-0.5_f32)` → `Pass`. Same bug manifested via a number whose formatted form has exactly one leading zero after the point.
- `witness_format32_roundtrip_case_negative_small_near_zero` — `property_format32_roundtrip(-0.0012345_f32)` → `Pass`. Same bug, stretched to three leading zeros after the point.
- `witness_format32_roundtrip_case_positive_small_sanity` — `property_format32_roundtrip(1.234e-3_f32)` → `Pass`. Positive control: still passes on both base and mutation, because the mutation only corrupts the sign byte of negatives.
- `witness_format64_roundtrip_case_negative_small_sanity` — `property_format64_roundtrip(-1.234e-3_f64)` → `Pass`. f64 sanity: the `daf6d4d` patch does not touch `format64`, so this stays green; exists to ensure the `property_format64_roundtrip` function is exercised by at least one witness.
