//! Definitions for representing piecesets.

/// Represents a set of chess pieces,
/// equipped with a notion of color and kind.
pub trait Piece {
    /// The set of colors in the pieceset.
    type Color: Eq;

    /// The set of kinds in the pieceset.
    type Kind: Eq;

    /// Returns the color of the piece.
    fn color(&self) -> Self::Color;

    /// Returns the kind of the piece.
    fn kind(&self) -> Self::Kind;

    /// Constructs a piece from the given color and kind.
    fn new(color: Self::Color, kind: Self::Kind) -> Self
    where
        Self: Sized;
}
