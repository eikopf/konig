//! An abstract `Piece` trait.

/// Represents a set of chess pieces.
pub trait Piece: PartialEq + Eq + std::fmt::Debug + TryFrom<char> + Into<char> {}
