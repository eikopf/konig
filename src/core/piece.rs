//! Definitions for representing piecesets.

/// Represents a set of chess pieces,
/// equipped with a notion of color and kind.
///
/// This trait is purposefully "lax," as
/// piecesets can vary wildly between variants.
/// Broadly, they all agree that chess variants
/// should have differently colored sides, composed
/// of a variety of members from the same set of kinds.
///
/// If an implementation doesn't use color or kind, then
/// it can set the corresponding associated types to `()`
/// as necessary.
///
/// > Note that the term *kind* is used throughout [`konig`](crate)
/// to avoid conflicting with the `type` keyword.
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
