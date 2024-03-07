use crate::consts::{VECTOR_COMPUTATION_OFFSET, DATA_LENGTH, INFO_LENGTH, NUMBER_OF_INPUT_NEURONS, NUMBER_OF_OUTPUT_NEURONS, INNER_COMPRESSION_RATIO};

#[derive(Debug, Clone, Copy)]
//#[repr(C, align(8))]
pub struct NeuronsCompressed {
    pub input_sign: [u8; ((DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH)/(INNER_COMPRESSION_RATIO*2)) + VECTOR_COMPUTATION_OFFSET[0]],
    pub output_sign: [u8; ((DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH)/(INNER_COMPRESSION_RATIO*2)) + VECTOR_COMPUTATION_OFFSET[0]],
    pub input: [i32; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH],
    pub output: [i32; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH],
}

impl NeuronsCompressed {
    #[inline(always)]
    pub(crate) fn update_input_sign(&mut self, idx: usize) {
        let sub_idx = idx%200;
        let exp_idx = idx/200;
        let byte_group = sub_idx/64;
        
        if byte_group != 3 {
            let sub_byte_group = sub_idx%8;
            let select_bit = (sub_idx/8)%8;

            self.input_sign[exp_idx*25 + byte_group*8 + sub_byte_group + VECTOR_COMPUTATION_OFFSET[0]] &= !(1 << select_bit);
            self.input_sign[exp_idx*25 + byte_group*8 + sub_byte_group + VECTOR_COMPUTATION_OFFSET[0]] |= (((self.input[idx] >> 31) & 1) as u8) << select_bit;
        } else {
            self.input_sign[idx/8 + VECTOR_COMPUTATION_OFFSET[0]] &= !(1 << (idx % 8));
            self.input_sign[idx/8 + VECTOR_COMPUTATION_OFFSET[0]] |= (((self.input[idx] >> 31) & 1) as u8) << (idx % 8);
        }   
    }

    #[inline(always)]
    pub(crate) fn update_output_sign(&mut self, idx: usize) {
        let sub_idx = idx%200;
        let exp_idx = idx/200;
        let byte_group = sub_idx/64;
        
        if byte_group != 3 {
            let sub_byte_group = sub_idx%8;
            let select_bit = (sub_idx/8)%8;

            self.output_sign[exp_idx*25 + byte_group*8 + sub_byte_group + VECTOR_COMPUTATION_OFFSET[0]] &= !(1 << select_bit);
            self.output_sign[exp_idx*25 + byte_group*8 + sub_byte_group + VECTOR_COMPUTATION_OFFSET[0]] |= (((self.output[idx] >> 31) & 1) as u8) << select_bit;
        } else {
            self.output_sign[idx/8 + VECTOR_COMPUTATION_OFFSET[0]] &= !(1 << (idx % 8));
            self.output_sign[idx/8 + VECTOR_COMPUTATION_OFFSET[0]] |= (((self.output[idx] >> 31) & 1) as u8) << (idx % 8);
        }  
    }

    #[inline(always)]
    pub(crate) fn setup_input(&mut self, precompressed_mining_data: &[u8; DATA_LENGTH/8]) {
        self.input_sign[0..DATA_LENGTH/8].copy_from_slice(precompressed_mining_data);
    }

    #[inline(always)]
    pub(crate) fn setup_output(&mut self) {
        self.output_sign[0..INFO_LENGTH/8].copy_from_slice(&self.input_sign[(DATA_LENGTH + NUMBER_OF_INPUT_NEURONS)/8 + VECTOR_COMPUTATION_OFFSET[0]..(DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH)/8 + VECTOR_COMPUTATION_OFFSET[0]])
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn reset_output(&mut self) {
        self.output_sign = [0u8; ((DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH)/(INNER_COMPRESSION_RATIO*2)) + VECTOR_COMPUTATION_OFFSET[0]];
        self.output = [0; INFO_LENGTH + DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS];
    }
}

impl Default for NeuronsCompressed {
    fn default() -> Self {
        Self {
            input: [0; DATA_LENGTH + NUMBER_OF_INPUT_NEURONS + INFO_LENGTH],
            output: [0; DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH],
            input_sign: [0; ((DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH)/(INNER_COMPRESSION_RATIO*2)) + VECTOR_COMPUTATION_OFFSET[0]],
            output_sign: [0; ((DATA_LENGTH + NUMBER_OF_OUTPUT_NEURONS + INFO_LENGTH)/(INNER_COMPRESSION_RATIO*2)) + VECTOR_COMPUTATION_OFFSET[0]],
        }
    }
}