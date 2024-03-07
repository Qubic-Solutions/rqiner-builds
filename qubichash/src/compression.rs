use std::simd::{u64x8, u64x4, u8x64, u8x32};
use crate::consts::DATA_LENGTH;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompressedState {
    pub pos: [u8; 25],
    pub neg: [u8; 25]
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlinedCompressedState<'a> {
    pub pos: &'a mut [u8; 25],
    pub neg: &'a mut [u8; 25]
}


#[inline(always)]
pub fn split_compress_x8(state: &[u64x8; 25], compressed_state: &mut [CompressedState; 8], state_il: &mut [[u8x64; 8]; 4]) {
    unsafe {
        let state = &*(state as *const u64x8 as *const [[[u8; 8]; 8]; 25]); // [u8; 8] = u64, [[u8; 8]; 8] = u64x8

        // select i'th u64x8
        for i in 0..8 {
            // select il group state_il[n] ([u8x64; 8])
            let select_il_state_group = i/2;

            // select u64 sub index
            let select_u64_sub = 25 * (i%2);

            // iterate over k lanes
            for k in 0..25 {

                // iterate over u64
                for j in 0..8 {
                    state_il[select_il_state_group][j][k + select_u64_sub] = state[k][i][j];
                }
            }
        }
        
        for n in 0..4 {
            let mut pos = u8x64::default();
            let mut neg = u8x64::default();
            for i in 0..8 {
                state_il[n][i] = (state_il[n][i] % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
                let k = state_il[n][i] >> u8x64::from_array([1; 64]);
                pos |= k << u8x64::from_array([i as u8; 64]);
                neg |= ((state_il[n][i] & u8x64::from_array([1; 64])) & !k) << u8x64::from_array([i as u8; 64]);
            }

            compressed_state[2*n].pos.copy_from_slice(&pos.as_array()[..25]);
            compressed_state[2*n].neg.copy_from_slice(&neg.as_array()[..25]);
            compressed_state[2*n + 1].pos.copy_from_slice(&pos.as_array()[25..50]);
            compressed_state[2*n + 1].neg.copy_from_slice(&neg.as_array()[25..50]);
        }
    }
}

const SHIFTER_X8: u8x64 = u8x64::from_array([0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]);

#[inline(always)]
pub fn fast_split_compress_x8(state: &[u64x8; 25], compressed_state: &mut [CompressedState; 8]) {
    unsafe {
        let n_state = &*(state.as_ptr() as *const [[u8x64; 8]; 3]);

        // n_state gets compressed byte ordered
        // byte ordering works by putting the state in the SIMD vector as stored in memory
        // which causes different ordering in the compressed state, where index 0 8 16 24 32 40 48 56 are stored alongside in the first byte
        // the next byte has each index incremented, for every 8th compressed byte the indicies are incremented by 64 from the last 8th byte
        // e.g. byte 8 would contain the compressed byte indicies 64 72 80 88 96 104 112 120
        for n in 0..3 {
            let mut pos = u8x64::default();
            let mut neg = u8x64::default();

            for i in 0..8 {
                let tmp_state = (n_state[n][i] % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
                let k = tmp_state >> u8x64::from_array([1; 64]);
                pos |= k << u8x64::from_array([i as u8; 64]);
                neg |= ((tmp_state & u8x64::from_array([1; 64])) & !k) << u8x64::from_array([i as u8; 64]);
            }

            for i in 0..8 {
                compressed_state[i].pos[n*8..n*8 + 8].copy_from_slice(&pos.as_array()[i*8..i*8 + 8]);
                compressed_state[i].neg[n*8..n*8 + 8].copy_from_slice(&neg.as_array()[i*8..i*8 + 8]);
            }
        }

        let last_state = *(state.as_ptr().offset(24) as *const u8x64);

        let mut pos = u8x64::default();
        let mut neg = u8x64::default();

        let tmp_state = (last_state % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
        let k = tmp_state >> u8x64::from_array([1; 64]);
        pos |= k << SHIFTER_X8;
        neg |= ((tmp_state & u8x64::from_array([1; 64])) & !k) << SHIFTER_X8;

        //regular bit ordering for last byte: 0, 1, 2, 3, 4, 5, 6, 7 aka 192 193 194 195 196 197 198 199
        for n in 0..8 {
            for i in 0..8 {
                compressed_state[n].pos[24] |= pos[n*8 + i];
                compressed_state[n].neg[24] |= neg[n*8 + i];
            }
        }
    }
}

const SHIFTER_X4: u8x32 = u8x32::from_array([0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]);

#[inline(always)]
pub fn fast_split_compress_x4(state: &[u64x4; 25], compressed_state: &mut [CompressedState; 4]) {
    unsafe {
        let n_state = &*(state.as_ptr() as *const [[u8x32; 8]; 3]);

        // n_state gets compressed byte ordered
        // byte ordering works by putting the state in the SIMD vector as stored in memory
        // which causes different ordering in the compressed state, where index 0 8 16 24 32 40 48 56 are stored alongside in the first byte
        // the next byte has each index incremented, for every 8th compressed byte the indicies are incremented by 64 from the last 8th byte
        // e.g. byte 8 would contain the compressed byte indicies 64 72 80 88 96 104 112 120
        for n in 0..3 {
            let mut pos = u8x32::default();
            let mut neg = u8x32::default();

            for i in 0..8 {
                let tmp_state = (n_state[n][i] % u8x32::from_array([3; 32])) ^ u8x32::from_array([1; 32]);
                let k = tmp_state >> u8x32::from_array([1; 32]);
                pos |= k << u8x32::from_array([i as u8; 32]);
                neg |= ((tmp_state & u8x32::from_array([1; 32])) & !k) << u8x32::from_array([i as u8; 32]);
            }

            for i in 0..4 {
                compressed_state[i].pos[n*8..n*8 + 8].copy_from_slice(&pos.as_array()[i*8..i*8 + 8]);
                compressed_state[i].neg[n*8..n*8 + 8].copy_from_slice(&neg.as_array()[i*8..i*8 + 8]);
            }
        }

        let last_state = *(state.as_ptr().offset(24) as *const u8x32);

        let mut pos = u8x32::default();
        let mut neg = u8x32::default();

        let tmp_state = (last_state % u8x32::from_array([3; 32])) ^ u8x32::from_array([1; 32]);
        let k = tmp_state >> u8x32::from_array([1; 32]);
        pos |= k << SHIFTER_X4;
        neg |= ((tmp_state & u8x32::from_array([1; 32])) & !k) << SHIFTER_X4;

        //regular bit ordering for last byte: 0, 1, 2, 3, 4, 5, 6, 7 aka 192 193 194 195 196 197 198 199
        for n in 0..4 {
            for i in 0..8 {
                compressed_state[n].pos[24] |= pos[n*8 + i];
                compressed_state[n].neg[24] |= neg[n*8 + i];
            }
        }
    }
}

#[inline(always)]
pub fn fast_split_compress_x8_inlined(state: &[u64x8; 25], compressed_state: &mut [InlinedCompressedState; 8]) {
    unsafe {
        let n_state = &*(state.as_ptr() as *const [[u8x64; 8]; 3]);

        // n_state gets compressed byte ordered
        // byte ordering works by putting the state in the SIMD vector as stored in memory
        // which causes different ordering in the compressed state, where index 0 8 16 24 32 40 48 56 are stored alongside in the first byte
        // the next byte has each index incremented, for every 8th compressed byte the indicies are incremented by 64 from the last 8th byte
        // e.g. byte 8 would contain the compressed byte indicies 64 72 80 88 96 104 112 120
        for n in 0..3 {
            let mut pos = u8x64::default();
            let mut neg = u8x64::default();

            for i in 0..8 {
                let tmp_state = (n_state[n][i] % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
                let k = tmp_state >> u8x64::from_array([1; 64]);
                pos |= k << u8x64::from_array([i as u8; 64]);
                neg |= ((tmp_state & u8x64::from_array([1; 64])) & !k) << u8x64::from_array([i as u8; 64]);
            }

            for i in 0..8 {
                compressed_state[i].pos[n*8..n*8 + 8].copy_from_slice(&pos.as_array()[i*8..i*8 + 8]);
                compressed_state[i].neg[n*8..n*8 + 8].copy_from_slice(&neg.as_array()[i*8..i*8 + 8]);
            }
        }

        let last_state = *(state.as_ptr().offset(24) as *const u8x64);

        let mut pos = u8x64::default();
        let mut neg = u8x64::default();

        let tmp_state = (last_state % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
        let k = tmp_state >> u8x64::from_array([1; 64]);
        pos |= k << SHIFTER_X8;
        neg |= ((tmp_state & u8x64::from_array([1; 64])) & !k) << SHIFTER_X8;

        //regular bit ordering for last byte: 0, 1, 2, 3, 4, 5, 6, 7 aka 192 193 194 195 196 197 198 199
        for n in 0..8 {
            let mut pos_in = 0;
            let mut neg_in = 0;
            for i in 0..8 {
                pos_in |= pos[n*8 + i];
                neg_in |= neg[n*8 + i];
            }

            compressed_state[n].pos[24] = pos_in;
            compressed_state[n].neg[24] = neg_in;
        }
    }
}


#[inline(always)]
pub fn split_compress_x8_exp(state: &[u64x8; 25], compressed_state: &mut [CompressedState; 8], state_il: &mut [[u8x64; 8]; 4]) {
    unsafe {

        // select i'th u64x8
        for n in 0..4 {
            for n_sub in 0..2 {
                // select il group state_il[n] ([u8x64; 8])
                //let select_il_state_group = i/2;

                // select u64 sub index
                let select_u64_sub = 25 * n_sub;

                // iterate over k lanes
                for k in 0..25 {

                    // iterate over u64
                    for j in 0..8 {
                        state_il[n][j][k + select_u64_sub] = *(state as *const u64x8 as *const u8).offset((k*64 + 8*(2*n + n_sub) + j) as isize);
                    }
                }
            }
        }
        
        for n in 0..4 {
            let mut pos = u8x64::default();
            let mut neg = u8x64::default();
            for i in 0..8 {
                state_il[n][i] = (state_il[n][i] % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
                let k = state_il[n][i] >> u8x64::from_array([1; 64]);
                pos |= k << u8x64::from_array([i as u8; 64]);
                neg |= ((state_il[n][i] & u8x64::from_array([1; 64])) & !k) << u8x64::from_array([i as u8; 64]);
            }

            compressed_state[2*n].pos.copy_from_slice(&pos.as_array()[..25]);
            compressed_state[2*n].neg.copy_from_slice(&neg.as_array()[..25]);
            compressed_state[2*n + 1].pos.copy_from_slice(&pos.as_array()[25..50]);
            compressed_state[2*n + 1].neg.copy_from_slice(&neg.as_array()[25..50]);
        }
    }
}

#[inline(always)]
pub fn split_compress_x4(state: &[u64x4; 25], compressed_state: &mut [CompressedState; 4]) {
    unsafe {
        let mut state_il = [[u8x64::default(); 8]; 4];
        let state = &*(state as *const u64x4 as *const [[[u8; 8]; 4]; 25]); // [u8; 8] = u64, [[u8; 8]; 8] = u64x8

        // select i'th u64x8
        for i in 0..4 {
            // select il group state_il[n] ([u8x64; 8])
            let select_il_state_group = i/2;

            // select u64 sub index
            let select_u64_sub = 25 * (i%2);

            // iterate over k lanes
            for k in 0..25 {

                // iterate over u64
                for j in 0..8 {
                    state_il[select_il_state_group][j][k + select_u64_sub] = state[k][i][j];
                }
            }
        }
        
        for n in 0..2 {
            let mut pos = u8x64::default();
            let mut neg = u8x64::default();
            for i in 0..8 {
                state_il[n][i] = (state_il[n][i] % u8x64::from_array([3; 64])) ^ u8x64::from_array([1; 64]);
                let k = state_il[n][i] >> u8x64::from_array([1; 64]);
                pos |= k << u8x64::from_array([i as u8; 64]);
                neg |= ((state_il[n][i] & u8x64::from_array([1; 64])) & !k) << u8x64::from_array([i as u8; 64]);
            }

            compressed_state[2*n].pos.copy_from_slice(&pos.as_array()[..25]);
            compressed_state[2*n].neg.copy_from_slice(&neg.as_array()[..25]);
            compressed_state[2*n + 1].pos.copy_from_slice(&pos.as_array()[25..50]);
            compressed_state[2*n + 1].neg.copy_from_slice(&neg.as_array()[25..50]);
        }
    }
}

//#[inline(always)]
pub(crate) fn select_nth(c: &[u8], idx: usize) -> i8 {
    let sub_idx = idx%200;
    let exp_idx = idx/200;
    let select_byte_group = sub_idx/64;

    // byte group == 3 means index >192 which accesses bit ordered pos and neg fields
    if select_byte_group != 3 {
        let select_byte_subgroup = sub_idx%8;
        let select_bit = (sub_idx/8)%8;

        let val = (c[exp_idx*25 + select_byte_group*8 + select_byte_subgroup] >> select_bit) & 1;

        if val == 0 { 1 } else { -1 }
    } else {
        if (c[idx/8] >> idx%8) & 1 == 0 { 1 } else { -1 }
    }
}

pub fn compress_mining_data(values: [i32; DATA_LENGTH]) -> [u8; DATA_LENGTH/8] {
    let mut data = [0; DATA_LENGTH/8];

    for i in 0..DATA_LENGTH {
        let sub_idx = i%200;
        let exp_idx = i/200;
        let byte_group = sub_idx/64;
        let sub_byte_group = sub_idx%8;
        let select_bit = (sub_idx/8)%8;


        if byte_group != 3 {
            data[exp_idx*25 + byte_group*8 + sub_byte_group] |= (if values[i] < 0 { 1 } else { 0 }) << select_bit;
        } else {
            data[i/8] |= (if values[i] < 0 { 1 } else { 0 }) << i%8;
        }
    }

    data
}

pub fn compress_mining_data_bit_ordered(values: [i32; DATA_LENGTH]) -> [u8; DATA_LENGTH/8] {
    let mut data = [0; DATA_LENGTH/8];

    for i in 0..DATA_LENGTH {
        data[i/8] |= (if values[i] < 0 { 1 } else { 0 }) << i%8;
    }

    data
}

#[test]
fn test_mining_data_compression() {
    let mut random_seed = [0u8; 32];

    random_seed[0] = 66;
    random_seed[1] = 99;
    random_seed[2] = 25;
    random_seed[3] = 11;
    random_seed[4] = 169;
    random_seed[5] = 122;
    random_seed[6] = 77;
    random_seed[7] = 102;

    let random_seed: [u64; 4] = random_seed.chunks_exact(8).into_iter().map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
    let mut mining_data = [0i32; DATA_LENGTH];
    
    crate::random::random(&random_seed, &random_seed, mining_data.as_mut_ptr() as *mut u64, std::mem::size_of::<[i32; DATA_LENGTH]>());

    let pc_data = crate::compression::compress_mining_data(mining_data);

    for i in 0..1200 {
        //dbg!(i, pc_data[i]);
        assert_eq!(if mining_data[i] >= 0 { 1 } else { -1 }, select_nth(&pc_data, i));
    }
}

#[test]
fn test() {
    use rand::Rng;

    let mut rng = rand::thread_rng();

    let mut state = [u64x8::default(); 25];

    for i in 0..25 {
        for j in 0..8 {
            state[i][j] = rng.gen();
        }
    }
    let mut state_il = Default::default();
    let mut compressed_state1 = [CompressedState::default(); 8];

    split_compress_x8(&state, &mut compressed_state1, &mut state_il);

    
    let mut compressed_state2 = [CompressedState::default(); 8];
    split_compress_x8_exp(&state, &mut compressed_state2, &mut state_il);

    assert_eq!(compressed_state1, compressed_state2);
}