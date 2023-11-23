//! Traits for representing moves on chessboards.

use std::error::Error;

use super::{index::Index, position::Validate};

/// Represents an error which occurs during the verification
/// of a candidate [`Move`].
pub trait IllegalMoveError: Error {
    /// The associated [`Validate`] on which moves can act.
    type Board: Validate<
        Index = Self::Index,
        Move = Self::Move,
        LegalMove = Self::LegalMove,
        ValidationError = Self,
    >;
    /// The associated [`Index`] type (metric space) in which moves act.
    type Index: Index;
    /// The potentially illegal candidate moves.
    type Move: Move<Index = Self::Index, Board = Self::Board>;
    /// The verified-legal moves.
    type LegalMove: LegalMove<Index = Self::Index, Move = Self::Move>;
}

/// Represents a (potentially illegal) move on the associated [`Validate`].
///
/// A [`Move`] is essentially a pair of points in the metric space defined
/// by its generic [`Index`] type parameter; the convention in [`konig`](crate) is
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
    /// Returns the source and target [indices](Index) of the move as a pair.
    fn as_pair(&self) -> (Self::Index, Self::Index) {
        (self.source(), self.target())
    }
}

/// Represents a legal move on the associated [`Validate`]. Almost always,
/// this will be a simple wrapper type around the associated [`Move`] type.
///
/// This struct and the corresponding [`validate()`](Validate::validate)
/// method should be considered the source of truth for "completely legal"
/// moves within an implementation. In any other context, when receiving a
/// [`LegalMove`], you can and *should* assume it to be valid.
pub trait LegalMove: Move {
    /// The associated [`Move`] type of which [`LegalMove`]s are a subset.
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
