pub const DATA_LENGTH: usize = 1200;
pub const INFO_LENGTH: usize = 1200;
pub const NUMBER_OF_INPUT_NEURONS: usize = 2400;
pub const NUMBER_OF_OUTPUT_NEURONS: usize = 2400;
pub const MAX_INPUT_DURATION: usize = 200;
pub const MAX_OUTPUT_DURATION: usize = 200;
pub const SOLUTION_THRESHOLD: u32 = 690;
pub const INNER_COMPRESSION_RATIO: usize = 4;
pub const OUTER_COMPRESSION_RATIO: usize = 8; 
pub const KECCAK_LANE_SIZE: usize = OUTER_COMPRESSION_RATIO * 25;

/// 2x512 + 256, 2x512 + 256 + 5x512, in bytes
pub const VECTOR_COMPUTATION_OFFSET: [usize; 2] = [10, 30];