use crate::core::index::{Index, IndexError};
use crate::standard::board::StandardBoard;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct StandardIndex(u8);

impl Index for StandardIndex {
    type Board = StandardBoard;

    fn get_in(
        self,
        board: &Self::Board,
    ) -> &<<StandardIndex as crate::core::index::Index>::Board as crate::core::board::Board>::Piece
    {
        todo!()
    }
}

impl TryFrom<u8> for StandardIndex {
    type Error = IndexError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            index @ 0..=63 => Ok(Self(index)),
            index @ _ => Err(IndexError::OutOfBounds(index)),
        }
    }
}
