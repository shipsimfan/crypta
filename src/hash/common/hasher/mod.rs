use crate::hash::common::{BitLength, Block, HasherState};

mod digest;
mod new;
mod push;

/// A helper class that handles feeding bytes to the hashing function
pub(in crate::hash) struct Hasher<State: HasherState, Length: BitLength>
where
    [u8; State::BLOCK_SIZE]: Sized,
    [u8; std::mem::size_of::<Length>()]: Sized,
{
    block: Block<{ State::BLOCK_SIZE }>,
    state: State,
    length: Length,
}

impl<State: HasherState, Length: BitLength> Hasher<State, Length>
where
    [u8; State::BLOCK_SIZE]: Sized,
    [u8; std::mem::size_of::<Length>()]: Sized,
{
    /// Compresses the current block using the state
    fn compress(&mut self) {
        self.state.compress(self.block.consume());
    }
}
