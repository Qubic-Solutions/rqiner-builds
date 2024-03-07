use std::{ptr::read_unaligned, simd::u8x64};

use std::simd::{u64x4, u64x8, u8x32};

#[inline(always)]
pub fn ddp_opt512(sign: *const u64, neg: *const u64, pos: *const u64, neg_count: &mut u64x8, pos_count: &mut u64x8) {
    unsafe {
        let n = u64x8::from_array(read_unaligned(sign as *const [u64; 8]));
        let neg = u64x8::from_array(read_unaligned(neg as *const [u64; 8]));
        let pos = u64x8::from_array(read_unaligned(pos as *const [u64; 8]));

        let neg = neg ^ n;
        let pos = pos ^ n;

        for i in 0..8 {
            neg_count[i] += neg[i].count_ones() as u64;
            pos_count[i] += pos[i].count_ones() as u64;
        }
    }
}

#[inline(always)]
#[allow(dead_code)]
pub fn ddp_opt512_safe(sign: &[u8], neg: &[u8], pos: &[u8], neg_count: &mut u8x64, pos_count: &mut u8x64) {
    let n = u8x64::from_slice(sign);
    let neg = u8x64::from_slice(neg);
    let pos = u8x64::from_slice(pos);

    let neg = neg ^ n;
    let pos = pos ^ n;

    for i in 0..64 {
        neg_count[i] += neg[i].count_ones() as u8;
        pos_count[i] += pos[i].count_ones() as u8;
    }
}

#[inline(always)]
pub fn ddp_opt256(sign: *const u64, neg: *const u64, pos: *const u64, neg_count: &mut u64x8, pos_count: &mut u64x8) {
    unsafe {
        let n = u64x4::from_array(read_unaligned(sign as *const [u64; 4]));
        let neg = u64x4::from_array(read_unaligned(neg as *const [u64; 4]));
        let pos = u64x4::from_array(read_unaligned(pos as *const [u64; 4]));

        let neg = neg ^ n;
        let pos = pos ^ n;

        for i in 0..4 {
            neg_count[i] += neg[i].count_ones() as u64;
            pos_count[i] += pos[i].count_ones() as u64;
        }
    }
}

#[inline(always)]
pub fn ddp_opt16(sign: *const u16, neg: *const u16, pos: *const u16, neg_count: &mut u64x8, pos_count: &mut u64x8) {
    unsafe {
        let n = read_unaligned(sign);
        let neg = read_unaligned(neg);
        let pos = read_unaligned(pos);

        let neg = n ^ neg;
        let pos = n ^ pos;

        neg_count[0] += neg.count_ones() as u64;
        pos_count[0] += pos.count_ones() as u64;
    }
}

#[inline(always)]
#[allow(dead_code)]
pub fn ddp_opt256_safe(sign: &[u8], neg: &[u8], pos: &[u8], neg_count: &mut u8x64, pos_count: &mut u8x64) {
    let n = u8x32::from_slice(sign);
    let neg = u8x32::from_slice(neg);
    let pos = u8x32::from_slice(pos);

    let neg = neg ^ n;
    let pos = pos ^ n;

    for i in 0..32 {
        neg_count[i] += neg[i].count_ones() as u8;
        pos_count[i] += pos[i].count_ones() as u8;
    }
}