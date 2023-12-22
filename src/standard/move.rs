use super::{board::Board, square::Square};
use crate::core;
use thiserror::Error;

/// Results when a [`Move`] cannot be converted into a [`LegalMove`]
#[derive(Debug, Error)]
pub enum IllegalMoveError {
    /// Results when a [`Move`] is illegal because the friendly king is in check.
    #[error("Invalid move {0:?}: the friendly king is in check.")]
    Check(Move),
    /// Results when a [`Move`] is illegal because it has an invalid source index.
    #[error("Invalid move source: {0:?}")]
    InvalidSource(Square),
    /// Results when a [`Move`] is illegal because it has an invalid target index.
    #[error("Invalid move target: {0:?}")]
    InvalidTarget(Square),
}

impl core::IllegalMoveError for IllegalMoveError {
    type Board = Board;
    type Index = Square;
    type Move = Move;
    type LegalMove = LegalMove;
}

/// Represents a possible move on a [`Board`],
/// including illegal moves.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Move {
    /// The position to take a [piece](crate::standard::piece::StandardPiece) from.
    source: Square,
    /// The position to move a [piece](crate::standard::piece::StandardPiece) to.
    target: Square,
}

/// Represents a legal move on a [`Board`].
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct LegalMove(Move);

impl core::Move for Move {
    type Index = Square;

    fn source(&self) -> Self::Index {
        self.source
    }

    fn target(&self) -> Self::Index {
        self.target
    }
}

impl core::Move for LegalMove {
    type Index = Square;

    fn source(&self) -> Self::Index {
        self.0.source
    }

    fn target(&self) -> Self::Index {
        self.0.target
    }
}

impl core::LegalMove for LegalMove {
    type Board = Board;
    type Move = Move;
}

impl core::WrapMove for LegalMove {
    unsafe fn wrap_unchecked(value: Self::Move) -> Self {
        Self(value)
    }
}

impl From<(Square, Square)> for Move {
    fn from(value: (Square, Square)) -> Self {
        Self {
            source: value.0,
            target: value.1,
        }
    }
}
