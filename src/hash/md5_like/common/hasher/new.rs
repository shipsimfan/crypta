use crate::hash::md5_like::common::{BitLength, Block, Hasher, HasherState};

impl<State: HasherState, Length: BitLength> Hasher<State, Length>
where
    [u8; State::BLOCK_SIZE]: Sized,
    [u8; core::mem::size_of::<Length>()]: Sized,
{
    /// Creates a new [`Hasher`]
    pub(in crate::hash::md5_like) fn new(initial_state: State) -> Self {
        Hasher {
            block: Block::new(),
            state: initial_state,
            length: Length::ZERO,
        }
    }
}
