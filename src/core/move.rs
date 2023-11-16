//! An abstract `Move` trait.

use std::error::Error;

use super::board::Board;

/// Represents an error which occurs during the verification
/// of a candidate move.
pub trait IllegalMoveError: Error {
    /// The associated board on which moves can act.
    type Board: Board;
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
    type Board: Board;
    /// The corresponding [`Move`] implementation, which may or may not be illegal.
    type Move: Move<Board = <Self as LegalMove>::Board>;
}

/// Crate-internal constructor trait for [`LegalMove`]s.
///
/// The visibility modifier here prevents a crate consumer from
/// constructing an invalid [`LegalMove`].
pub(crate) trait WrapMove: LegalMove {
    /// Directly wraps a [`Move`] with a [`LegalMove`],
    /// without a validation step.
    fn wrap(value: Self::Move) -> Self;
}

impl<T: LegalMove> Move for T {
    type Board = <T as LegalMove>::Board;
}
