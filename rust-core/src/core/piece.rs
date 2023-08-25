use std::fmt::Display;

use thiserror::Error;

/// An error enumerating the ways
/// in which the `u8` representation
/// of a `Piece` may fail.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum PieceRepresentationError {
    #[error("invalid integer representation of a PieceColor")]
    InvalidColorBit,

    #[error("invalid integer representation of a PieceType")]
    InvalidTypeBits,

    #[error("invalid character representation of a Piece")]
    InvalidChar,

    #[error("invalid representation of a Piece")]
    Unknown,
}

/// Represents the color associated with
/// a piece in a binary choice.
///
/// When a `Piece` is mapped to a 4 bit
/// integer, the discriminant of this
/// enum will be the 4th bit.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceColor {
    Black = 0,
    White = 1,
}

/// Represents the type associated
/// with a piece. By convention
/// this may also be called a "kind,"
/// as Rust reserves the type keyword.
///
/// When a `Piece` is mapped to a 4 bit
/// integer, the discriminant of this
/// enum will be the lower 3 bits.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    None = 0,
    Pawn = 1,
    Rook = 2,
    Knight = 3,
    Bishop = 4,
    Queen = 5,
    King = 6,
}

/// Represents a chess piece as a
/// combination of a `PieceColor` and
/// a `PieceType`.
///
/// The associated implementations
/// of `Into<u8>` and `TryFrom<u8>`
/// describe a mapping to a 4-bit
/// integer, where the color becomes
/// the most significant (4th) bit,
/// and the type is mapped to the
/// lower 3 bits.
///
/// The intention of this mapping
/// is to efficiently store the
/// state of a game in just 32 bytes
/// as each "channel" (bit) of the
/// 4-bit representation can be
/// stored in a `u64`, and read/write
/// operations can be accomplished
/// with left and right shifts.
///
/// Consider that an equivalent
/// representation as a `[Piece; 64]`
/// would use 128 bytes, as a single
/// `Piece` has `size = 2`.
#[derive(Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceType,
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_bit: u8 = (self.color as u8) << 3;
        let type_bits: u8 = self.kind as u8;
        write!(f, "{}", color_bit | type_bits)
    }
}

impl TryFrom<u8> for Piece {
    type Error = PieceRepresentationError;

    /// Attempts to convert a `u8` into a `Piece`.
    ///
    /// This function expects the input to adhere
    /// to the mapping described by the implementation
    /// of `Piece::Into<u8>`, i.e. all inputs must:
    /// - Be zero in the upper 4 bits;
    /// - Be in the range \[0, 6\] in the lower 3 bits.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let color = match value >> 3 {
            0 => PieceColor::Black,
            1 => PieceColor::White,
            _ => return Err(PieceRepresentationError::InvalidColorBit),
        };

        let kind = match value % 8 {
            0 => PieceType::None,
            1 => PieceType::Pawn,
            2 => PieceType::Rook,
            3 => PieceType::Knight,
            4 => PieceType::Bishop,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => return Err(PieceRepresentationError::InvalidTypeBits),
        };

        return Ok(Piece{color, kind})
    }
}

impl TryFrom<char> for Piece {
    type Error = PieceRepresentationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' => Ok(Piece{ color: PieceColor::Black, kind: PieceType::Pawn }),
            'P' => Ok(Piece { color: PieceColor::White, kind: PieceType::Pawn }),
            'r' => Ok(Piece { color: PieceColor::Black, kind: PieceType::Rook }),
            'R' => Ok(Piece { color: PieceColor::White, kind: PieceType::Rook }),
            'b' => Ok(Piece { color: PieceColor::Black, kind: PieceType::Bishop }),
            'B' => Ok(Piece { color: PieceColor::White, kind: PieceType::Bishop }),
            'n' => Ok(Piece { color: PieceColor::Black, kind: PieceType::Knight }),
            'N' => Ok(Piece { color: PieceColor::White, kind: PieceType::Knight }),
            'q' => Ok(Piece { color: PieceColor::Black, kind: PieceType::Queen }),
            'Q' => Ok(Piece { color: PieceColor::White, kind: PieceType::Queen }),
            'k' => Ok(Piece { color: PieceColor::Black, kind: PieceType::King }),
            'K' => Ok(Piece { color: PieceColor::White, kind: PieceType::King }),
            _ => Err(PieceRepresentationError::InvalidChar),
        }
    }
}

impl Into<u8> for Piece {

    /// Maps a `Piece` to its 4-bit integer
    /// representation. This is the canonical
    /// mapping for all numeric representations
    /// of `Piece`.
    fn into(self) -> u8 {
        return ((self.color as u8) << 3) + (self.kind as u8)
    }
}


#[cfg(test)]
mod tests {
    use super::{Piece, PieceColor, PieceType};

    #[test]
    fn validate_piece_into_u8() {
        let black_king = Piece{
            color: PieceColor::Black,
            kind: PieceType::King,
        };

        let white_rook = Piece{
            color: PieceColor::White,
            kind: PieceType::Rook,
        };

        let black_king_value: u8 = black_king.into();
        let white_rook_value: u8 = white_rook.into();

        assert_eq!(black_king_value, 6);
        assert_eq!(white_rook_value, 10);
    }

    #[test]
    fn validate_piece_try_from_u8() {
        let black_king = Piece{
            color: PieceColor::Black,
            kind: PieceType::King,
        };

        let white_rook = Piece{
            color: PieceColor::White,
            kind: PieceType::Rook,
        };

        assert_eq!(black_king, Piece::try_from(6).unwrap());
        assert_eq!(white_rook, Piece::try_from(10).unwrap());
    }
}
