mod random_compressed_x8;

pub use random_compressed_x8::*;

use std::ptr::copy_nonoverlapping;

use keccak_p::keccak_p1600_12;

#[inline(always)]
pub fn random(public_key: &[u64; 4], nonce: &[u64; 4], mut output: *mut u64, output_size: usize) {
    let mut state: [u64; 25] = [0; 25];
    unsafe {
        for i in 0..4 {
            state[i] = public_key[i];
            state[4 + i] = nonce[i]
        }

        for _ in 0..output_size/std::mem::size_of::<[u64; 25]>() {
            keccak_p1600_12(&mut state);

            copy_nonoverlapping(state.as_ptr(), output, 25);

            output = output.add(25)
        }

        if output_size%std::mem::size_of::<[u64; 25]>() != 0 {
            keccak_p1600_12(&mut state);
            copy_nonoverlapping(state.as_ptr(), output, (output_size%std::mem::size_of::<[u64; 25]>())/8);
        }
    } 
}