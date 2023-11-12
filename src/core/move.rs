//! An abstract `Move` trait.

use super::board::Board;

/// Represents a (potentially illegal) move on the associated [`Board`].
pub trait Move {
    /// A [`Board`] on which moves can act.
    type Board: Board;
}

/// Represents a legal move on the associated [`Board`].
pub trait LegalMove {
    /// A [`Board`] whose legal moves coincide with an implementor of this trait.
    type Board: Board<LegalMove = Self, Move = <Self as LegalMove>::Move>;
    /// The corresponding [`Move`] implementation, which may or may not be illegal.
    type Move: Move<Board = <Self as LegalMove>::Board>;
}

impl<T: LegalMove> Move for T {
    type Board = <T as LegalMove>::Board;
}

// TODO: create an IllegalMoveError trait
// TODO: generalize the Move -> LegalMove relationship with traits
