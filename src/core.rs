//! Abstract traits for implementing chess and chess variants.

pub mod index;
pub mod r#move;
pub mod piece;
pub mod position;

// reexported traits
pub use index::Index;
pub use index::PieceMetric;
pub use piece::Piece;
pub use position::Position;
pub use position::Process;
pub use position::Standard;
pub use position::Validate;
pub use r#move::LegalMove;
pub use r#move::Move;
