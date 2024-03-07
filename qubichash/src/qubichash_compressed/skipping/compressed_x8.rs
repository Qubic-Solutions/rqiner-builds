use std::simd::u64x8;
use std::simd::prelude::{SimdUint, SimdInt};
use crate::{consts::*, compression::select_nth, ddp::{ddp_opt512, ddp_opt256, ddp_opt16}};
use crate::compressed_data::{NeuronsCompressed, SynapsesCompressed};
use crate::random::random_compressed_x8;

const PRE_ITERATION_INPUT_DURATION: usize = 28;
const PRE_ITERATION_OUTPUT_DURATION: usize = 10;
const HEAD_THRESHOLD: usize = 622;
const TAIL_THRESHOLD: usize = 628;
const TAIL_ITERATION_DURATION: usize = 10;

#[inline(always)]
//#[unroll::unroll_for_loops]
pub fn qubichash_compressed_x8(nonce: &[u64x8; 4], computor_public_key: &[u64x8; 4], precompressed_mining_data: &[u8; DATA_LENGTH/8], neurons: &mut [NeuronsCompressed; 8], synapses: &mut [SynapsesCompressed; 8], solution_threshold: u32) -> [bool; 8] {
    //random seeding
    random_compressed_x8(computor_public_key, nonce, synapses);

    let mut results = [false; 8];

    '_iteration: for iteration in 0..8 {
        synapses[iteration].post_setup();

        //seed input neurons
        neurons[iteration].setup_input(precompressed_mining_data);

        let mut update_index;

        let mut saved_precalc = [0i32; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH];
        let mut neuron_indicies = [0; NUMBER_OF_INPUT_NEURONS + INFO_LENGTH];
        let mut length_index = 0;

        // precalculate first DATA_LENGTH results
        for input_neuron_index in 0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH {
            unsafe {
                let mut pos_count = u64x8::default();
                let mut neg_count = u64x8::default();
                let base_index = ((input_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (input_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);
                update_index = DATA_LENGTH + input_neuron_index as usize;

                for another_input_neuron_index in 0..2 {
                    let sign = neurons.get_unchecked(iteration).input_sign.get_unchecked(another_input_neuron_index*64) as *const u8 as *const u64;
                    let pos = synapses.get_unchecked(iteration).input.pos.get_unchecked(base_index + another_input_neuron_index*64)  as *const u8 as *const u64;
                    let neg = synapses.get_unchecked(iteration).input.neg.get_unchecked(base_index + another_input_neuron_index*64)  as *const u8 as *const u64;
                    ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                }

                let sign = neurons.get_unchecked(iteration).input_sign.get_unchecked(2*64) as *const u8 as *const u64;
                let pos = synapses.get_unchecked(iteration).input.pos.get_unchecked(base_index + 2*64)  as *const u8 as *const u64;
                let neg = synapses.get_unchecked(iteration).input.neg.get_unchecked(base_index + 2*64)  as *const u8 as *const u64;
                ddp_opt256(sign, neg, pos, &mut neg_count, &mut pos_count);

                saved_precalc[update_index] = (pos_count.cast::<i32>() - neg_count.cast::<i32>()).reduce_sum() as i32;
            }
        }
        
        '_pre_iteration: {
            //input tick
            for _tick in 0..PRE_ITERATION_INPUT_DURATION {
                for i in 0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH {
                    neuron_indicies[i] = i as u16;
                }
                
                for number_of_remaining_neurons in (0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH).rev() {
                    let neuron_index_index = synapses[iteration].lengths[length_index] as u32 % (number_of_remaining_neurons as u32 + 1);
                    length_index += 1;

                    let input_neuron_index = neuron_indicies[neuron_index_index as usize];

                    neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];
        
                    let base_index = ((input_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (input_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);
        
                    update_index = DATA_LENGTH + input_neuron_index as usize;
                    
                    unsafe {
                        let mut pos_count = u64x8::default();
                        let mut neg_count = u64x8::default();
        
                        neurons[iteration].input[update_index] += saved_precalc[update_index];
        
                        for another_input_neuron_index in 3..10 {
                            let sign = neurons.get_unchecked(iteration).input_sign.get_unchecked(another_input_neuron_index*64 - 32) as *const u8 as *const u64;
                            let pos = synapses.get_unchecked(iteration).input.pos.get_unchecked(base_index + another_input_neuron_index*64 - 32)  as *const u8 as *const u64;
                            let neg = synapses.get_unchecked(iteration).input.neg.get_unchecked(base_index + another_input_neuron_index*64 - 32)  as *const u8 as *const u64;
                            ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                        }

                        let sign = neurons.get_unchecked(iteration).input_sign.get_unchecked(10*64 - 32) as *const u8 as *const u16;
                        let pos = synapses.get_unchecked(iteration).input.pos.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;
                        let neg = synapses.get_unchecked(iteration).input.neg.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;

                        ddp_opt16(sign, neg, pos, &mut neg_count, &mut pos_count);
                        
                        neurons[iteration].input[update_index] += (pos_count.cast::<i64>() - neg_count.cast::<i64>()).reduce_sum() as i32;

                        neurons[iteration].update_input_sign(update_index); // this line has to stay here xD
                    }
                }
            }

            let mut length_index = MAX_INPUT_DURATION * (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH);
            let mut saved_precalc = [0i32; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH];

            neurons[iteration].setup_output();

            // precalculate first INFO_LENGTH results
            for output_neuron_index in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
                unsafe {
                        let mut pos_count = u64x8::default();
                        let mut neg_count = u64x8::default();
                        let base_index = ((output_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (output_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);
                        
                        update_index = INFO_LENGTH + output_neuron_index as usize;

                        for another_output_neuron_index in 0..2 {
                            let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(another_output_neuron_index*64) as *const u8 as *const u64;
                            let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + another_output_neuron_index*64)  as *const u8 as *const u64;
                            let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + another_output_neuron_index*64)  as *const u8 as *const u64;
                            ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                        }

                    let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(2*64) as *const u8 as *const u64;
                    let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + 2*64)  as *const u8 as *const u64;
                    let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + 2*64)  as *const u8 as *const u64;
                    ddp_opt256(sign, neg, pos, &mut neg_count, &mut pos_count);

                    saved_precalc[update_index] = (pos_count.cast::<i32>() - neg_count.cast::<i32>()).reduce_sum();
                }
            }

            //output tick
            for _tick in 0..PRE_ITERATION_OUTPUT_DURATION {
                for i in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
                    neuron_indicies[i] = i as u16;
                }

                for number_of_remaining_neurons in (0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH).rev() {
                    let neuron_index_index = synapses[iteration].lengths[length_index] as u32 % (number_of_remaining_neurons as u32 + 1);
                    length_index += 1;
                    let output_neuron_index = neuron_indicies[neuron_index_index as usize];
                    neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];

                    let base_index = ((output_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (output_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);

                    
                    update_index = INFO_LENGTH + output_neuron_index as usize;

                    unsafe {
                        let mut pos_count = u64x8::default();
                        let mut neg_count = u64x8::default();

                        neurons[iteration].output[update_index] += saved_precalc[update_index];

                        for another_output_neuron_index in 3..10 {
                            let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(another_output_neuron_index*64 - 32) as *const u8 as *const u64;
                            let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + another_output_neuron_index*64 - 32)  as *const u8 as *const u64;
                            let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + another_output_neuron_index*64 - 32)  as *const u8 as *const u64;
                            ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                        }

                        let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(10*64 - 32) as *const u8 as *const u16;
                        let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;
                        let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;

                        ddp_opt16(sign, neg, pos, &mut neg_count, &mut pos_count);

                        neurons[iteration].output[update_index] += (pos_count.cast::<i32>() - neg_count.cast::<i32>()).reduce_sum();

                        neurons[iteration].update_output_sign(update_index); // this line has to stay here xD
                    }
                }
            }

            let mut score = 0;

            //score evaluation
            for i in 0..DATA_LENGTH {
                if (select_nth(precompressed_mining_data, i) >= 0)  == (neurons[iteration].output[INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + i] >= 0) {
                    score += 1;
                }
            }

            if score < HEAD_THRESHOLD {
                continue '_iteration;
            }

            neurons[iteration].reset_output();
        }

        //input tick
        for _tick in PRE_ITERATION_INPUT_DURATION..MAX_INPUT_DURATION {
            for i in 0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH {
                neuron_indicies[i] = i as u16;
            }
            
            for number_of_remaining_neurons in (0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH).rev() {
                let neuron_index_index = synapses[iteration].lengths[length_index] as u32 % (number_of_remaining_neurons as u32 + 1);
                length_index += 1;

                let input_neuron_index = neuron_indicies[neuron_index_index as usize];

                neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];
    
                let base_index = ((input_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (input_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);
    
                update_index = DATA_LENGTH + input_neuron_index as usize;
                
                unsafe {
                    let mut pos_count = u64x8::default();
                    let mut neg_count = u64x8::default();
    
                    neurons[iteration].input[update_index] += saved_precalc[update_index];
    
                    for another_input_neuron_index in 3..10 {
                        let sign = neurons.get_unchecked(iteration).input_sign.get_unchecked(another_input_neuron_index*64 - 32) as *const u8 as *const u64;
                        let pos = synapses.get_unchecked(iteration).input.pos.get_unchecked(base_index + another_input_neuron_index*64 - 32)  as *const u8 as *const u64;
                        let neg = synapses.get_unchecked(iteration).input.neg.get_unchecked(base_index + another_input_neuron_index*64 - 32)  as *const u8 as *const u64;
                        ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                    }

                    let sign = neurons.get_unchecked(iteration).input_sign.get_unchecked(10*64 - 32) as *const u8 as *const u16;
                    let pos = synapses.get_unchecked(iteration).input.pos.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;
                    let neg = synapses.get_unchecked(iteration).input.neg.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;

                    ddp_opt16(sign, neg, pos, &mut neg_count, &mut pos_count);
                    
                    neurons[iteration].input[update_index] += (pos_count.cast::<i64>() - neg_count.cast::<i64>()).reduce_sum() as i32;

                    neurons[iteration].update_input_sign(update_index); // this line has to stay here xD
                }
            }
        }

        saved_precalc = [0i32; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH];

        neurons[iteration].setup_output();
        let mut neuron_indicies = [0u16; NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH];

        // precalculate first INFO_LENGTH results
        for output_neuron_index in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
            unsafe {
                    let mut pos_count = u64x8::default();
                    let mut neg_count = u64x8::default();
                    let base_index = ((output_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (output_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);
                    
                    update_index = INFO_LENGTH + output_neuron_index as usize;

                    for another_output_neuron_index in 0..2 {
                        let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(another_output_neuron_index*64) as *const u8 as *const u64;
                        let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + another_output_neuron_index*64)  as *const u8 as *const u64;
                        let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + another_output_neuron_index*64)  as *const u8 as *const u64;
                        ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                    }

                let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(2*64) as *const u8 as *const u64;
                let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + 2*64)  as *const u8 as *const u64;
                let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + 2*64)  as *const u8 as *const u64;
                ddp_opt256(sign, neg, pos, &mut neg_count, &mut pos_count);

                saved_precalc[update_index] = (pos_count.cast::<i32>() - neg_count.cast::<i32>()).reduce_sum();
            }
        }

        //output tick
        for _tick in 0..TAIL_ITERATION_DURATION {
            for i in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
                neuron_indicies[i] = i as u16;
            }

            for number_of_remaining_neurons in (0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH).rev() {
                let neuron_index_index = synapses[iteration].lengths[length_index] as u32 % (number_of_remaining_neurons as u32 + 1);
                length_index += 1;
                let output_neuron_index = neuron_indicies[neuron_index_index as usize];
                neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];

                let base_index = ((output_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (output_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);

                
                update_index = INFO_LENGTH + output_neuron_index as usize;

                unsafe {
                    let mut pos_count = u64x8::default();
                    let mut neg_count = u64x8::default();

                    neurons[iteration].output[update_index] += saved_precalc[update_index];

                    for another_output_neuron_index in 3..10 {
                        let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(another_output_neuron_index*64 - 32) as *const u8 as *const u64;
                        let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + another_output_neuron_index*64 - 32)  as *const u8 as *const u64;
                        let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + another_output_neuron_index*64 - 32)  as *const u8 as *const u64;
                        ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                    }

                    let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(10*64 - 32) as *const u8 as *const u16;
                    let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;
                    let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;

                    ddp_opt16(sign, neg, pos, &mut neg_count, &mut pos_count);

                    neurons[iteration].output[update_index] += (pos_count.cast::<i32>() - neg_count.cast::<i32>()).reduce_sum();

                    neurons[iteration].update_output_sign(update_index); // this line has to stay here xD
                }
            }
        }

        let mut score = 0;

        //score evaluation
        for i in 0..DATA_LENGTH {
            if (select_nth(precompressed_mining_data, i) >= 0)  == (neurons[iteration].output[INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + i] >= 0) {
                score += 1;
            }
        }

        if score < TAIL_THRESHOLD {
            continue '_iteration;
        }

        //output tick
        for _tick in TAIL_ITERATION_DURATION..MAX_OUTPUT_DURATION {
            for i in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
                neuron_indicies[i] = i as u16;
            }

            for number_of_remaining_neurons in (0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH).rev() {
                let neuron_index_index = synapses[iteration].lengths[length_index] as u32 % (number_of_remaining_neurons as u32 + 1);
                length_index += 1;
                let output_neuron_index = neuron_indicies[neuron_index_index as usize];
                neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];

                let base_index = ((output_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO) + (output_neuron_index as usize * VECTOR_COMPUTATION_OFFSET[0]);

                
                update_index = INFO_LENGTH + output_neuron_index as usize;

                unsafe {
                    let mut pos_count = u64x8::default();
                    let mut neg_count = u64x8::default();

                    neurons[iteration].output[update_index] += saved_precalc[update_index];

                    for another_output_neuron_index in 3..10 {
                        let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(another_output_neuron_index*64 - 32) as *const u8 as *const u64;
                        let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + another_output_neuron_index*64 - 32)  as *const u8 as *const u64;
                        let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + another_output_neuron_index*64 - 32)  as *const u8 as *const u64;
                        ddp_opt512(sign, neg, pos, &mut neg_count, &mut pos_count);
                    }

                    let sign = neurons.get_unchecked(iteration).output_sign.get_unchecked(10*64 - 32) as *const u8 as *const u16;
                    let pos = synapses.get_unchecked(iteration).output.pos.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;
                    let neg = synapses.get_unchecked(iteration).output.neg.get_unchecked(base_index + 10*64 - 32)  as *const u8 as *const u16;

                    ddp_opt16(sign, neg, pos, &mut neg_count, &mut pos_count);

                    neurons[iteration].output[update_index] += (pos_count.cast::<i32>() - neg_count.cast::<i32>()).reduce_sum();

                    neurons[iteration].update_output_sign(update_index); // this line has to stay here xD
                }
            }
        }

        let mut score = 0;

        //score evaluation
        for i in 0..DATA_LENGTH {
            if (select_nth(precompressed_mining_data, i) >= 0)  == (neurons[iteration].output[INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + i] >= 0) {
                score += 1;
            }
        }

        //dbg!(score);

        if score >= solution_threshold {
            results[iteration] = true;
        }
    }

    results
}

// set your test nonce and public key here
const _TEST_NONCE: &str = "0x338016c96a5ac28fe2fd89b9348f86dab106e6bcff89c4beb293ae4216663365";  //"0xcae5b74282086e57268955ca2486414b75d09cff4f438fd18a30951e7b4bd51e";
#[test]
fn test() {
    use std::str::FromStr;
    stacker::grow(256*1024*1024, || {
        use qubic_types::QubicId;
        
        let id = QubicId::from_str("COTZPAFPVDGDJGEFQIKRQJFSTPZCKNQMPYNSXFNUTAWGIRWADJCNPTGEXYUO").unwrap();
        let nonce = hex::decode(&_TEST_NONCE[2..]).unwrap();
        println!("nonce: {:?}", nonce);
        println!("pk: {:?}", id.0);
        let nonce: [u64; 4] = nonce.chunks_exact(8).into_iter().map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
        let public_key: [u64; 4] = id.to_le_u64();
        let mut random_seed = [0u8; 32];

        random_seed[0] = 55;
        random_seed[1] = 35;
        random_seed[2] = 31;
        random_seed[3] = 89;
        random_seed[4] = 23;
        random_seed[5] = 67;
        random_seed[6] = 255;
        random_seed[7] = 17;

        let random_seed: [u64; 4] = random_seed.chunks_exact(8).into_iter().map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
        let mut mining_data = [0i32; DATA_LENGTH];

        let mut neurons = [NeuronsCompressed::default(); 8];
        let mut synapses = [SynapsesCompressed::default();  8];
        let mut n = [u64x8::default(); 4];
        let mut pk = [u64x8::default(); 4];
        
        crate::random::random(&random_seed, &random_seed, mining_data.as_mut_ptr() as *mut u64, std::mem::size_of::<[i32; DATA_LENGTH]>());

        let pc_data = crate::compression::compress_mining_data(mining_data);

        for i in 0..4 {
            n[i][0] = nonce[i];
            pk[i][0] = public_key[i];
        }

        dbg!(qubichash_compressed_x8(&n,&pk, &pc_data, &mut neurons, &mut synapses, 692));
    });
}