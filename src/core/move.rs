//! An abstract `Move` trait.

use std::error::Error;

use super::board::Board;

/// Represents an error which occurs during the verification
/// of a candidate move.
pub trait IllegalMoveError: Error {
    /// The associated board on which moves can act.
    type Board: Board<Move = Self::Move, LegalMove = Self::LegalMove>;
    /// The potentially illegal candidate moves.
    type Move: Move<Board = Self::Board>;
    /// The verified-legal moves.
    type LegalMove: LegalMove<Board = Self::Board>;
}

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
