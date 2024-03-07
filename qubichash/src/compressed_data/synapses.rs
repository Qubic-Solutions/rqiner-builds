use crate::consts::{NUMBER_OF_INPUT_NEURONS, INFO_LENGTH, DATA_LENGTH, NUMBER_OF_OUTPUT_NEURONS, OUTER_COMPRESSION_RATIO, VECTOR_COMPUTATION_OFFSET, MAX_OUTPUT_DURATION, MAX_INPUT_DURATION};

const INPUT_LENGTH_OVERFLOWING: usize =  ((NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO + ((NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) * VECTOR_COMPUTATION_OFFSET[0]);
const OUTPUT_LENGTH_OVERFLOWING: usize =  ((NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) * (DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH))/OUTER_COMPRESSION_RATIO + ((NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) * VECTOR_COMPUTATION_OFFSET[0]);

const LENGTH_LENGTH: usize = MAX_INPUT_DURATION * (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + MAX_OUTPUT_DURATION * (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH);

#[derive(Debug, Clone, Copy)]
//#[repr(C, align(8))]
pub struct SynapsesCompressed {
    pub input: SynapseBase<INPUT_LENGTH_OVERFLOWING>,
    pub output: SynapseBase<OUTPUT_LENGTH_OVERFLOWING>,
    pub lengths: [u16; LENGTH_LENGTH]
}

#[test]
fn test_size() {
    dbg!(std::mem::size_of::<SynapsesCompressed>());
}

#[derive(Debug, Clone, Copy)]
//#[repr(C)]
pub struct SynapseBase<const LENGTH: usize> {
    pub pos: [u8; LENGTH],
    pub neg: [u8; LENGTH]
}

impl<const LENGTH: usize> Default for SynapseBase<LENGTH> {
    fn default() -> Self {
        Self {
            pos: [0; LENGTH],
            neg: [0; LENGTH]
        }
    }
}

impl SynapsesCompressed {
    #[inline(always)]
    pub(crate) fn post_setup(&mut self) {
        for input_neuron_index in 0..NUMBER_OF_INPUT_NEURONS + INFO_LENGTH {
            let idx = input_neuron_index * (DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + (DATA_LENGTH + input_neuron_index);
            let sub_idx = idx%200;
            let exp_idx = idx/200;
            let byte_group = sub_idx/64;
            
            if byte_group != 3 {
                let sub_byte_group = sub_idx%8;
                let select_bit = (sub_idx/8)%8;

                self.input.pos[exp_idx*25 + 8*byte_group + sub_byte_group + input_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << select_bit);
                self.input.neg[exp_idx*25 + 8*byte_group + sub_byte_group + input_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << select_bit);
            } else {
                self.input.pos[idx/8 + input_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << (idx%8));
                self.input.neg[idx/8 + input_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << (idx%8));
            }   
        }
        for output_neuron_index in 0..NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH {
            let idx = output_neuron_index * (INFO_LENGTH + NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH) + (INFO_LENGTH + output_neuron_index);
            let sub_idx = idx%200;
            let exp_idx = idx/200;
            let byte_group = sub_idx/64;

            if byte_group != 3 {
                let sub_byte_group = sub_idx%8;
                let select_bit = (sub_idx/8)%8;

                self.output.pos[exp_idx*25 + 8*byte_group + sub_byte_group + output_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << select_bit);
                self.output.neg[exp_idx*25 + 8*byte_group + sub_byte_group + output_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << select_bit);
            } else {
                self.output.pos[idx/8 + output_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << (idx%8));
                self.output.neg[idx/8 + output_neuron_index * VECTOR_COMPUTATION_OFFSET[0] + VECTOR_COMPUTATION_OFFSET[0]] &= !(0b1 << (idx%8));
            }   
        }
    }
}

impl Default for SynapsesCompressed {
    fn default() -> Self {
        Self {
            input: SynapseBase::default(),
            output: SynapseBase::default(),
            lengths: [0; MAX_INPUT_DURATION * (NUMBER_OF_INPUT_NEURONS + INFO_LENGTH) + MAX_OUTPUT_DURATION * (NUMBER_OF_OUTPUT_NEURONS + DATA_LENGTH)]
        }
    }
}