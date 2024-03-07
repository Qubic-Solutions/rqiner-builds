#![feature(portable_simd)]

mod macros;

use std::simd::{u64x2, u64x4, u64x8};

trait RotateLeft {
    fn rotate_left(&self, s: u64) -> Self;
}

macro_rules! generate_function {
    ($name: ident, $type: ty, $lanes: expr) => {
        impl RotateLeft for $type {
            #[inline(always)]
            fn rotate_left(&self, s: u64) -> Self {
                *self << <$type>::splat(s) | *self >> <$type>::splat(64-s)
            }
        }

        #[inline(always)]
        pub fn $name(lanes: &mut [$type; 25]) {
            iter_rounds!(
                lanes,
                (<$type>::from_array([0x000000008000808b; $lanes]), <$type>::from_array([0x800000000000008b; $lanes])),
                (<$type>::from_array([0x8000000000008089; $lanes]), <$type>::from_array([0x8000000000008003; $lanes])),
                (<$type>::from_array([0x8000000000008002; $lanes]), <$type>::from_array([0x8000000000000080; $lanes])),
                (<$type>::from_array([0x000000000000800a; $lanes]), <$type>::from_array([0x800000008000000a; $lanes])),
                (<$type>::from_array([0x8000000080008081; $lanes]), <$type>::from_array([0x8000000000008080; $lanes])),
                (<$type>::from_array([0x0000000080000001; $lanes]), <$type>::from_array([0x8000000080008008; $lanes]))
            );
        }
    };
}

generate_function!(keccak_p1600_12_x2, u64x2, 2);
generate_function!(keccak_p1600_12_x4, u64x4, 4);
generate_function!(keccak_p1600_12_x8, u64x8, 8);

#[test]
fn test() {
    let mut state_simd = [u64x2::default(); 25];
    let mut state = [0u64; 25];
    keccak_p1600_12_x2(&mut state_simd);
    keccak_p::keccak_p1600_12(&mut state);

    dbg!(state_simd, state);
}