use super::{board::StandardBoard, index::StandardIndex, piece::StandardPiece};
use crate::core::r#move::Move;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IllegalStandardMoveError {
    #[error("Invalid move {0:?}: the friendly king is in check.")]
    Check(StandardMove),
    #[error("Invalid move source: {0:?}")]
    InvalidSource(StandardIndex),
    #[error("Invalid move target: {0:?}")]
    InvalidTarget(StandardIndex),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct StandardMove {
    source: StandardIndex,
    target: StandardIndex,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct LegalStandardMove(StandardMove);

impl Move for StandardMove {
    type Board = StandardBoard;
    type Index = StandardIndex;
    type Piece = StandardPiece;
}

impl Move for LegalStandardMove {
    type Board = StandardBoard;
    type Index = StandardIndex;
    type Piece = StandardPiece;
}
