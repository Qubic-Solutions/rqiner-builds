#![feature(portable_simd)]
#![feature(const_mut_refs)]

// double-dot-product 
mod ddp;
pub mod qubichash_compressed;
pub mod compressed_data;
pub mod consts;
pub mod compression;
pub mod random;

use consts::*;

#[derive(Debug, Clone, Copy)]
pub struct Neurons {
    pub input: [i32; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH],
    pub output: [i32; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH],
    pub input_sign: [i8; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH],
    pub output_sign: [i8; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH]
}

impl Default for Neurons {
    fn default() -> Self {
        Self {
            input: [0; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH],
            output: [0; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH],
            input_sign: [1; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH],
            output_sign: [1; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Synapses {
    pub input: [i8; (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH)],
    pub output: [i8; (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH)],
    pub lengths: [u16; MAX_INPUT_DURATION * (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + MAX_OUTPUT_DURATION * (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH)]
}

impl Default for Synapses {
    fn default() -> Self {
        Self {
            input: [i8::default(); (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH)],
            output: [i8::default(); (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) * (INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH)],
            lengths: [0; MAX_INPUT_DURATION * (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + MAX_OUTPUT_DURATION * (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH)]
        }
    }
}

#[inline(always)]
pub fn qubichash(nonce: &[u64; 4], mining_data: &[i32; DATA_LENGTH], computor_public_key: &[u64; 4], neurons: &mut Neurons, synapses: &mut Synapses, solution_threshold: u32) -> bool {
    crate::random::random(computor_public_key, nonce, synapses as *mut Synapses as *mut u64, std::mem::size_of::<Synapses>());

    for input_neuron_index in 0..(NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) {
        for another_input_neuron_offset in 0..(INFO_LENGTH + NUMBER_OF_INPUT_NEURONS + DATA_LENGTH) {
            let offset = input_neuron_index * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + another_input_neuron_offset;
            synapses.input[offset] = (synapses.input[offset] as u8 % 3) as i8 - 1;
        }
    }

    for output_neuron_index in 0..(NUMBER_OF_INPUT_NEURONS + DATA_LENGTH) {
        for another_output_neuron_offset in 0..(INFO_LENGTH + NUMBER_OF_INPUT_NEURONS + DATA_LENGTH) {
            let offset = output_neuron_index * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + another_output_neuron_offset;
            synapses.output[offset] = (synapses.output[offset] as u8 % 3) as i8 - 1;
        }
    }

    for input_neuron_index in 0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH {
        let idx = input_neuron_index * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + (DATA_LENGTH + input_neuron_index);
        synapses.input[idx] = 0;
    }
    for output_neuron_index in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
        let idx = output_neuron_index * (INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) + (INFO_LENGTH + output_neuron_index);
        synapses.output[idx] = 0;
    }

    //seed input neurons
    neurons.input[0..DATA_LENGTH].copy_from_slice(&mining_data.as_slice());

    let mut length_index = 0;
    let mut neuron_indicies = [0; NUMBER_OF_INPUT_NEURONS + INFO_LENGTH];
    for i in 0..DATA_LENGTH {
        neurons.input_sign[i] = if neurons.input[i] < 0 { -1 } else { 1 };
    }

    let mut update_index = 0;

    //input tick
    for _tick in 0..MAX_INPUT_DURATION {
        let mut number_of_remaining_neurons = NUMBER_OF_INPUT_NEURONS + INFO_LENGTH;
        for i in 0..number_of_remaining_neurons {
            neuron_indicies[i] = i as u16;
        }
        while number_of_remaining_neurons != 0 {
            let neuron_index_index = synapses.lengths[length_index] as u32 % (number_of_remaining_neurons as u32);
            length_index += 1;
            number_of_remaining_neurons -= 1;
            let input_neuron_index = neuron_indicies[neuron_index_index as usize];
            neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];
            

            let base_index = input_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH);

            neurons.input_sign[update_index] = if neurons.input[update_index] < 0 { -1 } else { 1 };

            update_index = DATA_LENGTH + input_neuron_index as usize;

            for another_input_neuron_index in 0..(DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) {
                neurons.input[update_index] += (neurons.input_sign[another_input_neuron_index] * synapses.input[base_index + another_input_neuron_index]) as i32;
            }
        }
    }

    neurons.output_sign[0..INFO_LENGTH].copy_from_slice(&neurons.input_sign[DATA_LENGTH + NUMBER_OF_INPUT_NEURONS..DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH]);

    let mut neuron_indicies = [0u16; NUMBER_OF_INPUT_NEURONS + DATA_LENGTH];
    //output tick
    for _tick in 0..MAX_OUTPUT_DURATION {
        let mut number_of_remaining_neurons = NUMBER_OF_INPUT_NEURONS + DATA_LENGTH;

        for i in 0..number_of_remaining_neurons {
            neuron_indicies[i] = i as u16;
        }

        while number_of_remaining_neurons != 0 {
            let neuron_index_index = synapses.lengths[length_index] as u32 % (number_of_remaining_neurons as u32);
            length_index += 1;
            number_of_remaining_neurons -= 1;
            let output_neuron_index = neuron_indicies[neuron_index_index as usize];
            neuron_indicies[neuron_index_index as usize] = neuron_indicies[number_of_remaining_neurons];
            

            let base_index = output_neuron_index as usize * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH);
            neurons.output_sign[update_index] = if neurons.output[update_index] < 0 { -1 } else { 1 };
            update_index = INFO_LENGTH + output_neuron_index as usize;

            for another_output_neuron_index in 0..(DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) {
                neurons.output[update_index] += (neurons.output_sign[another_output_neuron_index] * synapses.output[base_index + another_output_neuron_index]) as i32;
            }
        }
    }

    let mut score = 0;

    //score evaluation
    for i in 0..DATA_LENGTH {
        if (mining_data[i] >= 0) == (neurons.output[INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + i] >= 0) {
            score += 1;
        }
    }

    score >= solution_threshold
}

// set your test nonce and public key here
const _TEST_NONCE: [u8; 32] = [0; 32];
#[test]
fn test() {
    use std::str::FromStr;
    stacker::grow(256*1024*1024, || {
        use qubic_types::QubicId;
        let id = QubicId::from_str("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
        println!("{:?}", id.0);
        let nonce: [u64; 4] = _TEST_NONCE.chunks_exact(8).into_iter().map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
        let public_key: [u64; 4] = id.0.chunks_exact(8).into_iter().map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
        let mut random_seed = [0u8; 32];
        random_seed[0] = 66;
        random_seed[1] = 99;
        random_seed[2] = 25;
        random_seed[3] = 11;
        random_seed[4] = 169;
        random_seed[5] = 122;
        random_seed[6] = 77;
        random_seed[7] = 137;

        let random_seed: [u64; 4] = random_seed.chunks_exact(8).into_iter().map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect::<Vec<_>>().try_into().unwrap();
        let mut mining_data = [0i32; DATA_LENGTH];
        crate::random::random(&random_seed, &random_seed, mining_data.as_mut_ptr() as *mut u64, std::mem::size_of::<[i32; DATA_LENGTH]>());


        let mut neurons = Neurons::default();
        let mut synapses = Synapses::default();

        dbg!(qubichash(&nonce, &mining_data, &public_key, &mut neurons, &mut synapses, 1115));
    });
}


#[test]
fn test_ddp() {
    /*
        double dot product test

        n1 and n2 are neuron signs with encoding 1 := 0b00 | -1 := 0b01 | 

        s1 and s2 are synapse signs with encoding 0 := 0b00 | 1 := 0b01 | -1 := 0b10
    */

    let n1: u8 = 0b01;
    let n2: u8 = 0b10;

    let s1: u8 = 0b01;
    let s2: u8 = 0b01;

    const HI_MASK: u8 = 0b10;
    const LO_MASK: u8 = 0b01;

    // align n1 to lo part and n2 to hi part
    let n = n1 | (n2 << 1);

    // mask s1 and s2 and put s2 lo value to hi position
    let lo = (s1 & LO_MASK) | ((s2 & LO_MASK) << 1);

    // mask s1 and s2 and put s1 hi value to lo position
    let hi = ((s1 & HI_MASK) >> 1) | (s2 & HI_MASK);

    // XOR hi with n to get the dot product of the -1 part
    let hi = hi ^ n;

    // XOR lo with n to get the dot product of the +1 part
    let lo = lo ^ n;

    // displays the result of n1 * s1 + n2 * s2
    dbg!(lo.count_ones() as i32 - hi.count_ones() as i32);
}