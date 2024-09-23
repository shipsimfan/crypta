use crate::hash::common::HasherState;
use compress::compress;

mod compress;

/// The state used by SHA-1 hash functions
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

    fn compress(&mut self, block: &[u8]) {
        let (a, b, c, d, e) =
            compress(block, self.0[0], self.0[1], self.0[2], self.0[3], self.0[4]);

        // Compute the intermediate hash value
        self.0[0] = a.wrapping_add(self.0[0]);
        self.0[1] = b.wrapping_add(self.0[1]);
        self.0[2] = c.wrapping_add(self.0[2]);
        self.0[3] = d.wrapping_add(self.0[3]);
        self.0[4] = e.wrapping_add(self.0[4]);
    }
}
