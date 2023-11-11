use super::board::Board;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexError<T> {
    #[error("Received an out-of-bounds index: {0}")]
    OutOfBounds(T),
    #[error("Received an index with invalid formatting: {0}")]
    InvalidFormat(T),
}

pub trait Index {
    type Board: Board;

    // indexes into the provided board and returns a reference
    // to the piece at the corresponding position
    fn get_in(self, board: &Self::Board) -> &<Self::Board as Board>::Piece;
}
