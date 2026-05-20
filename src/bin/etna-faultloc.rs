// Crabcheck fault-localization runner for ryu.
//
// Mirrors the aho-corasick / crc32fast etna-faultloc binaries: drives the
// roundtrip property via `crabcheck::profiling::quickcheck`, which
// instruments every iteration with LLVM coverage and snapshots .profraw
// files around the failing seed for SBFL analysis.
//
// Self-contained — the existing `etna` runner in src/bin/etna.rs is
// untouched.

use std::fmt;

use crabcheck::profiling::quickcheck;
use crabcheck::quickcheck::{Arbitrary, Mutate};
use rand::Rng;
use ryu::etna::{property_format32_roundtrip, property_format64_roundtrip, PropertyResult};

// Wrap raw bit patterns so Arbitrary draws over the full IEEE 754 space
// (matches what src/bin/etna.rs's crabcheck adapter does — it generates
// random u32/u64 bits and reinterprets, so denormals / large magnitudes /
// the small-negative range that triggers the bug are all reachable).

#[derive(Clone, Copy)]
struct F32(f32);
#[derive(Clone, Copy)]
struct F64(f64);

impl fmt::Debug for F32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (bits={:#010x})", self.0, self.0.to_bits())
    }
}
impl fmt::Debug for F64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (bits={:#018x})", self.0, self.0.to_bits())
    }
}

impl<R: Rng> Arbitrary<R> for F32 {
    fn generate(rng: &mut R, _n: usize) -> Self {
        F32(f32::from_bits(rng.random::<u32>()))
    }
}
impl<R: Rng> Arbitrary<R> for F64 {
    fn generate(rng: &mut R, _n: usize) -> Self {
        F64(f64::from_bits(rng.random::<u64>()))
    }
}

// BST-style single-point perturbation: flip exactly one bit of the IEEE
// representation. Keeps the mutant close to the failing seed in bit-space.
// Mutants that become NaN/Inf get Discarded by the property's `is_finite`
// guard.
impl<R: Rng> Mutate<R> for F32 {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let bit = rng.random_range(0u32..32);
        F32(f32::from_bits(self.0.to_bits() ^ (1u32 << bit)))
    }
}
impl<R: Rng> Mutate<R> for F64 {
    fn mutate(&self, rng: &mut R, _n: usize) -> Self {
        let bit = rng.random_range(0u32..64);
        F64(f64::from_bits(self.0.to_bits() ^ (1u64 << bit)))
    }
}

fn to_opt(r: PropertyResult) -> Option<bool> {
    match r {
        PropertyResult::Pass => Some(true),
        PropertyResult::Fail(_) => Some(false),
        PropertyResult::Discard => None,
    }
}

// The format32 bug can corrupt the buffer such that ryu's
// `format_finite` panics during str slicing. Catch and treat as Fail so
// the mutation loop can continue.

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <tool> <property> [tests]", args[0]);
        eprintln!("  tool:     crabcheck");
        eprintln!("  property: Format32Roundtrip | Format64Roundtrip");
        return;
    }
    let tool = args[1].as_str();
    let property = args[2].as_str();

    let result = match (tool, property) {
        ("crabcheck", "Format32Roundtrip") => {
            quickcheck(|F32(f)| to_opt(property_format32_roundtrip(f)))
        },
        ("crabcheck", "Format64Roundtrip") => {
            quickcheck(|F64(f)| to_opt(property_format64_roundtrip(f)))
        },
        _ => panic!("Unknown tool or property: {tool} {property}"),
    };

    println!("Result: {:?}", result);
}
