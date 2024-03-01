use crate::hash::common::HasherState;

/// The state used by MD5 hash function
pub struct MD5HasherState([u32; 4]);

const INITIAL_HASH_VALUE: [u32; 4] = [0x01234567, 0x89ABCDEF, 0xFEDCBA98, 0x76543210];

impl MD5HasherState {
    /// Creates a new [`SHA2HasherState`]
    pub(super) fn new() -> Self {
        MD5HasherState(INITIAL_HASH_VALUE)
    }

    /// Unwraps the state to the contained hash
    pub(super) fn unwrap(self) -> [u8; 16] {
        let mut output = [0; 16];

        for i in 0..4 {
            let base = i * 4;

            output[base..base + 4].copy_from_slice(&self.0[i].to_be_bytes());
        }

        output
    }
}

impl HasherState for MD5HasherState {
    const BLOCK_SIZE: usize = 64;

    fn compress(&mut self, block: &[u8]) {}
}
