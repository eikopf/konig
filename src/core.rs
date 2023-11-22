//! Abstract traits for implementing chess and chess variants.

pub mod board;
pub mod index;
pub mod r#move;
pub mod piece;

// reexported traits
pub use board::Position;
pub use board::Process;
pub use board::Standard;
pub use board::Validate;
pub use index::Index;
pub use piece::Piece;
pub use r#move::LegalMove;
pub use r#move::Move;
