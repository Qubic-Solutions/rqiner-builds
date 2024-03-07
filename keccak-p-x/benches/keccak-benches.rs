#![feature(portable_simd)]
use std::simd::{u64x2, u64x4, u64x8};

use criterion::{Criterion, criterion_group, criterion_main, black_box};

use keccak_p_x::*;

pub fn benchmarks(c: &mut Criterion) {
    let mut state: [u64; 25] = [1, 2, 3, 1, 4, 6, 2, 4, 2, 4, 23, 18388, 1778388, 178838498, 17728, 187838, 72183, 9273, 7173277, 15636778, 163782, 17388912, 1727, 16273, 17273];

    c.bench_function("keccak-p", |b| b.iter(|| keccak_p::keccak_p1600_12(black_box(&mut state))));

    let mut state_x2 = [u64x2::default(); 25];

    for i in 0..25 {
        state_x2[i] = u64x2::splat(state[i]);
    }
    c.bench_function("keccak-p-x2", |b| b.iter(|| keccak_p1600_12_x2(&mut state_x2)));

    let mut state_x4 = [u64x4::default(); 25];

    for i in 0..25 {
        state_x4[i] = u64x4::splat(state[i]);
    }
    c.bench_function("keccak-p-x4", |b| b.iter(|| keccak_p1600_12_x4(&mut state_x4)));

    let mut state_x8 = [u64x8::default(); 25];

    for i in 0..25 {
        state_x8[i] = u64x8::splat(state[i]);
    }
    c.bench_function("keccak-p-x8", |b| b.iter(|| keccak_p1600_12_x8(&mut state_x8)));
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);