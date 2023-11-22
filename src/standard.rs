//! A concrete implementation of standard chess.

mod bitboard;

/// Defines a [`Board`] and related concepts.
pub mod board;

/// Defines a [`Square`] and related concepts.
pub mod square;

/// Defines a `StandardMove` and `LegalStandardMove`.
pub mod r#move;

/// Defines a `StandardPiece` and related concepts.
pub mod piece;

pub use board::Board;
pub use square::Square;
