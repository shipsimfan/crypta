use crate::hash::common::HasherState;

/// The state used by SHA-2 hash functions
pub struct SHA1HasherState([u32; 5]);

const INITIAL_HASH_VALUE: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];

impl SHA1HasherState {
    /// Creates a new [`SHA2HasherState`]
    pub(super) fn new() -> Self {
        SHA1HasherState(INITIAL_HASH_VALUE)
    }

    /// Unwraps the state to the contained hash
    pub(super) fn unwrap(self) -> [u8; 20] {
        let mut output = [0; 20];

        for i in 0..5 {
            let base = i * 4;

            output[base..base + 4].copy_from_slice(&self.0[i].to_be_bytes());
        }

        output
    }
}

impl HasherState for SHA1HasherState {
    const BLOCK_SIZE: usize = 64;

    fn compress(&mut self, block: &[u8]) {}
}
