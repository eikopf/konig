/// Represents a set of chess pieces.
pub trait Piece: TryFrom<char> + Into<char> {}
