//! An implementation of standard chess.

/// Defines a [`Board`] and related concepts.
mod board;

/// Defines a [`Square`] and related concepts.
mod square;

/// Defines a [`Move`] and [`LegalMove`].
mod r#move;

/// Defines a [`Piece`] and related concepts.
mod piece;

pub use board::Board;
pub use board::CastlingPermissions;
pub use piece::Color;
pub use piece::Piece;
pub use piece::PieceKind;
pub use r#move::IllegalMoveError;
pub use r#move::LegalMove;
pub use r#move::Move;
pub use square::Square;
