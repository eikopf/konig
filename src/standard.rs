//! A concrete implementation of standard chess.

/// Defines a [`Board`] and related concepts.
pub mod board;

/// Defines a [`Square`] and related concepts.
pub mod square;

/// Defines a [`Move`] and [`LegalMove`].
pub mod r#move;

/// Defines a [`Piece`] and related concepts.
pub mod piece;

pub use board::Board;
pub use board::CastlingPermissions;
pub use piece::Color;
pub use piece::Piece;
pub use piece::PieceKind;
pub use r#move::LegalMove;
pub use r#move::Move;
pub use square::Square;
