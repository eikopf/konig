use super::{board::StandardBoard, index::StandardIndex};
use crate::core::r#move::{IllegalMoveError, LegalMove, Move, WrapMove};
use thiserror::Error;

/// Results when a [`StandardMove`] cannot be converted into a [`LegalStandardMove`]
#[derive(Debug, Error)]
pub enum IllegalStandardMoveError {
    /// Results when a [`StandardMove`] is illegal because the friendly king is in check.
    #[error("Invalid move {0:?}: the friendly king is in check.")]
    Check(StandardMove),
    /// Results when a [`StandardMove`] is illegal because it has an invalid source index.
    #[error("Invalid move source: {0:?}")]
    InvalidSource(StandardIndex),
    /// Results when a [`StandardMove`] is illegal because it has an invalid target index.
    #[error("Invalid move target: {0:?}")]
    InvalidTarget(StandardIndex),
}

impl IllegalMoveError for IllegalStandardMoveError {
    type Board = StandardBoard;
    type Index = StandardIndex;
    type Move = StandardMove;
    type LegalMove = LegalStandardMove;
}

/// Represents a possible move on a `StandardBoard`,
/// including illegal moves.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct StandardMove {
    /// The position to take a [piece](crate::standard::piece::StandardPiece) from.
    pub source: StandardIndex,
    /// The position to move a [piece](crate::standard::piece::StandardPiece) to.
    pub target: StandardIndex,
}

/// Represents a legal move on a `StandardBoard`.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct LegalStandardMove(StandardMove);

impl Move for StandardMove {
    type Board = StandardBoard;
    type Index = StandardIndex;

    fn source(&self) -> Self::Index {
        self.source
    }

    fn target(&self) -> Self::Index {
        self.target
    }
}

impl Move for LegalStandardMove {
    type Board = StandardBoard;
    type Index = StandardIndex;

    fn source(&self) -> Self::Index {
        self.0.source
    }

    fn target(&self) -> Self::Index {
        self.0.target
    }
}

impl LegalMove for LegalStandardMove {
    type Move = StandardMove;
}

impl WrapMove for LegalStandardMove {
    fn wrap(value: Self::Move) -> Self {
        Self(value)
    }
}

impl From<(StandardIndex, StandardIndex)> for StandardMove {
    fn from(value: (StandardIndex, StandardIndex)) -> Self {
        Self {
            source: value.0,
            target: value.1,
        }
    }
}
