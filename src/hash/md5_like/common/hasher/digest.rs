use crate::hash::md5_like::common::{pad, zero_pad, BitLength, Hasher, HasherState};

impl<State: HasherState, Length: BitLength> Hasher<State, Length>
where
    [u8; State::BLOCK_SIZE]: Sized,
    [u8; std::mem::size_of::<Length>()]: Sized,
{
    /// Finalizes the hash
    pub(in crate::hash::md5_like) fn digest(mut self) -> State {
        let (padded_block, first_byte) =
            pad(&mut self.block, self.length, State::BIG_ENDIAN_LENGTH);
        self.state.compress(padded_block);

        if let Some(first_byte) = first_byte {
            let padded_block = zero_pad(
                &mut self.block,
                self.length,
                first_byte,
                State::BIG_ENDIAN_LENGTH,
            );
            self.state.compress(padded_block);
        }

        self.state
    }
}
