use crate::hash::common::{BitLength, Hasher, HasherState};

impl<State: HasherState, Length: BitLength> Hasher<State, Length>
where
    [u8; State::BLOCK_SIZE]: Sized,
    [u8; std::mem::size_of::<Length>()]: Sized,
{
    /// Adds `bytes` to the hash
    pub(in crate::hash) fn push_iter(&mut self, mut bytes: impl Iterator<Item = u8>) {
        loop {
            let count = self.block.push_bytes(&mut bytes);
            self.length.add_bytes(count);

            if self.block.remaining() == 0 {
                self.compress();
            } else {
                break;
            }
        }
    }

    /// Adds `bytes` to the hash
    pub(in crate::hash) fn push_slice(&mut self, bytes: impl AsRef<[u8]>) {
        let mut bytes = bytes.as_ref();
        self.length.add_bytes(bytes.len());

        // Is this one aligned and full block?
        if self.block.is_empty() && bytes.len() == State::BLOCK_SIZE {
            self.state.compress(bytes);
            return;
        }

        // Will this slice fully fit in the block?
        if bytes.len() > self.block.remaining() {
            // Fill the remaining block
            if !self.block.is_empty() {
                self.block.push_slice(&bytes[..self.block.remaining()]);
                self.compress();

                bytes = &bytes[self.block.remaining()..];
            }

            // Fill full blocks
            while bytes.len() >= State::BLOCK_SIZE {
                self.state.compress(&bytes[..State::BLOCK_SIZE]);

                bytes = &bytes[State::BLOCK_SIZE..];
            }
        }

        // Fill remaining bytes
        self.block.push_slice(bytes);

        if self.block.remaining() == 0 {
            self.compress();
        }
    }
}
