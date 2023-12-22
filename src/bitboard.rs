//! Provides essential bitboard utilities.
//!
//! A *bitboard* is a basic component of chess programming,
//! and typically refers to a `u64` representing a set of
//! squares that fulfil some property. This representation
//! lends itself well to fast set operations using bitwise
//! operations.
//!
//! Derivative constructions can be built from collections
//! of bitboards; a notable example is the *quadboard*,
//! a memory-efficient representation of the pieces on a standard
//! chessboard encoded into nibbles.

mod bitboard;
mod quadboard;

pub use bitboard::BitBoard;
pub use quadboard::NibbleDecode;
pub use quadboard::NibbleDecodingError;
pub use quadboard::NibbleEncode;
pub use quadboard::QuadBoard;
