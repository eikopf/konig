//! Traits for representing moves on chessboards.

use std::error::Error;

use super::{board::Validate, index::Index};

/// Represents an error which occurs during the verification
/// of a candidate move.
pub trait IllegalMoveError: Error {
    /// The associated board on which moves can act.
    type Board: Validate<
        Index = Self::Index,
        Move = Self::Move,
        LegalMove = Self::LegalMove,
        ValidationError = Self,
    >;
    /// The associated index type (metric space) in which moves act.
    type Index: Index;
    /// The potentially illegal candidate moves.
    type Move: Move<Index = Self::Index, Board = Self::Board>;
    /// The verified-legal moves.
    type LegalMove: LegalMove<Index = Self::Index, Move = Self::Move>;
}

/// Represents a (potentially illegal) move on the associated [`Validate`].
///
/// A [`Move`] is essentially a pair of points in the metric space defined
/// by its generic [`Index`] type parameter; the convention in `konig` is
/// to call the first point the `source` and the second point the `target`.
pub trait Move {
    /// A [`Validate`] against which moves can be checked.
    type Board: Validate<Index = Self::Index>;
    /// An [`Index`] metric space in which moves are considered pairs.
    type Index: Index;

    /// Returns the source [`Index`] of the move.
    fn source(&self) -> Self::Index;
    /// Returns the target [`Index`] of the move.
    fn target(&self) -> Self::Index;
    /// Returns the source and target [indices](Index) of the move.
    fn as_pair(&self) -> (Self::Index, Self::Index) {
        (self.source(), self.target())
    }
}

/// Represents a legal move on the associated [`Validate`].
pub trait LegalMove: Move {
    /// The associated [`Move`] type from which [`LegalMove`]s are derived.
    type Move: Move<Index = Self::Index, Board = Self::Board>;
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
