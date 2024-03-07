use std::ptr::copy_nonoverlapping;

use std::simd::u64x8;
use keccak_p_x::keccak_p1600_12_x8;

use crate::{compression::{fast_split_compress_x8, CompressedState}, consts::{NUMBER_OF_OUTPUT_NEURONS, DATA_LENGTH, OUTER_COMPRESSION_RATIO, VECTOR_COMPUTATION_OFFSET, INFO_LENGTH, NUMBER_OF_INPUT_NEURONS, MAX_INPUT_DURATION, MAX_OUTPUT_DURATION}, compressed_data::SynapsesCompressed};

pub fn random_compressed_x8(public_key: &[u64x8; 4], nonce: &[u64x8; 4], output: &mut [SynapsesCompressed; 8]) {
    let mut state: [u64x8; 25] = [u64x8::default(); 25];
    unsafe {
        for i in 0..4 {
            state[i] = public_key[i];
            state[4 + i] = nonce[i]
        }

        let mut output_ptr = (0..8).map(|n| [output[n].input.pos.as_mut_ptr(), output[n].input.neg.as_mut_ptr()]).collect::<Vec<_>>();
        
        let mut compressed_state = [CompressedState::default(); 8];
        for _ in 0..(NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) {

            for _ in 0..(DATA_LENGTH/OUTER_COMPRESSION_RATIO)/25 {
                keccak_p1600_12_x8(&mut state);

                fast_split_compress_x8(&state, &mut compressed_state);

                for i in 0..8 {
                    copy_nonoverlapping(compressed_state[i].pos.as_ptr(), output_ptr[i][0], 25);
                    copy_nonoverlapping(compressed_state[i].neg.as_ptr(), output_ptr[i][1], 25);

                    output_ptr[i][0] = output_ptr[i][0].add(25);
                    output_ptr[i][1] = output_ptr[i][1].add(25);
                }

                compressed_state = [CompressedState::default(); 8];
            }

            // mid offset
            for i in 0..8 {
                output_ptr[i][0] = output_ptr[i][0].add(VECTOR_COMPUTATION_OFFSET[0]);
                output_ptr[i][1] = output_ptr[i][1].add(VECTOR_COMPUTATION_OFFSET[0]);
            }
            
            for _ in 0..((NUMBER_OF_INPUT_NEURONS + INFO_LENGTH)/OUTER_COMPRESSION_RATIO)/25 {
                keccak_p1600_12_x8(&mut state);

                fast_split_compress_x8(&state, &mut compressed_state);

                for i in 0..8 {
                    copy_nonoverlapping(compressed_state[i].pos.as_ptr(), output_ptr[i][0], 25);
                    copy_nonoverlapping(compressed_state[i].neg.as_ptr(), output_ptr[i][1], 25);

                    output_ptr[i][0] = output_ptr[i][0].add(25);
                    output_ptr[i][1] = output_ptr[i][1].add(25);
                }

                compressed_state = [CompressedState::default(); 8];
            }

            // tail offset
            /*for i in 0..8 {
                output_ptr[i][0] = output_ptr[i][0].add(VECTOR_COMPUTATION_OFFSET[1] - VECTOR_COMPUTATION_OFFSET[0]);
                output_ptr[i][1] = output_ptr[i][1].add(VECTOR_COMPUTATION_OFFSET[1] - VECTOR_COMPUTATION_OFFSET[0]);
            }*/
        }

        let mut output_ptr = (0..8).map(|n| [output[n].output.pos.as_mut_ptr(), output[n].output.neg.as_mut_ptr()]).collect::<Vec<_>>();

        let mut compressed_state = [CompressedState::default(); 8];
        for _ in 0..(NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) {
            for _ in 0..(INFO_LENGTH/OUTER_COMPRESSION_RATIO)/25 {
                keccak_p1600_12_x8(&mut state);

                fast_split_compress_x8(&state, &mut compressed_state);

                for i in 0..8 {
                    copy_nonoverlapping(compressed_state[i].pos.as_ptr(), output_ptr[i][0], 25);
                    copy_nonoverlapping(compressed_state[i].neg.as_ptr(), output_ptr[i][1], 25);

                    output_ptr[i][0] = output_ptr[i][0].add(25);
                    output_ptr[i][1] = output_ptr[i][1].add(25);
                }

                compressed_state = [CompressedState::default(); 8];
            }

            // mid offset
            for i in 0..8 {
                output_ptr[i][0] = output_ptr[i][0].add(VECTOR_COMPUTATION_OFFSET[0]);
                output_ptr[i][1] = output_ptr[i][1].add(VECTOR_COMPUTATION_OFFSET[0]);
            }
            
            for _ in 0..((NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH)/OUTER_COMPRESSION_RATIO)/25 {
                keccak_p1600_12_x8(&mut state);

                fast_split_compress_x8(&state, &mut compressed_state);

                for i in 0..8 {
                    copy_nonoverlapping(compressed_state[i].pos.as_ptr(), output_ptr[i][0], 25);
                    copy_nonoverlapping(compressed_state[i].neg.as_ptr(), output_ptr[i][1], 25);

                    output_ptr[i][0] = output_ptr[i][0].add(25);
                    output_ptr[i][1] = output_ptr[i][1].add(25);
                }

                compressed_state = [CompressedState::default(); 8];
            }

            // tail offset
            /*for i in 0..8 {
                output_ptr[i][0] = output_ptr[i][0].add(VECTOR_COMPUTATION_OFFSET[1] - VECTOR_COMPUTATION_OFFSET[0]);
                output_ptr[i][1] = output_ptr[i][1].add(VECTOR_COMPUTATION_OFFSET[1] - VECTOR_COMPUTATION_OFFSET[0]);
            }*/
        }

        let mut output = (0..8).map(|n| output[n].lengths.as_mut_ptr() as *mut u64).collect::<Vec<_>>();
        let mut deinterlaced = [0; 25];

        for _ in 0..std::mem::size_of::<[u16; MAX_INPUT_DURATION * (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + MAX_OUTPUT_DURATION * (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH)]>()/std::mem::size_of::<[u64; 25]>() {
            keccak_p1600_12_x8(&mut state);


            for i in 0..8 {
                for j in 0..25 {
                    deinterlaced[j] = state[j][i];
                }

                copy_nonoverlapping(deinterlaced.as_ptr(), output[i], 25);
                output[i] = output[i].add(25);
            }
        }
    }
}