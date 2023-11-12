//! An abstract `Move` trait.

pub trait Move {
    type Board;
    type Index;
    type Piece;
}
