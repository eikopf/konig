//! Abstract traits for implementing chess and chess variants.

mod index;
mod r#move;
mod piece;
mod position;

// reexported traits
pub use index::Algebraic;
pub use index::Index;
pub use index::IndexError;
pub use index::Metric;
pub use piece::Piece;
pub use position::Position;
pub use position::Process;
pub use position::Standard;
pub use position::Validate;
pub use r#move::LegalMove;
pub use r#move::Move;
pub use r#move::IllegalMoveError;

// crate reexports
pub(crate) use r#move::WrapMove;
