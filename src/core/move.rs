//! An abstract `Move` trait.

use std::error::Error;

use super::board::Validate;

/// Represents an error which occurs during the verification
/// of a candidate move.
pub trait IllegalMoveError: Error {
    /// The associated board on which moves can act.
    type Board: Validate;
    /// The potentially illegal candidate moves.
    type Move: Move<Board = Self::Board>;
    /// The verified-legal moves.
    type LegalMove: LegalMove;
}

/// Represents a (potentially illegal) move on the associated [`Board`].
pub trait Move {
    /// A [`Board`] on which moves can act.
    type Board: Validate;
}

/// Represents a legal move on the associated [`Board`].
pub trait LegalMove: Move {
    /// The associated [`Move`] type from which [`LegalMove`]s are derived.
    type Move: Move<Board = Self::Board>;
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
