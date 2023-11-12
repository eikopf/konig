use crate::core::piece::Piece;

/// Represents the standard set of chess pieces.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StandardPiece {
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
}

impl Piece for StandardPiece {}

impl TryFrom<char> for StandardPiece {
    type Error = (); // there's basically only one reason for this conversion to fail

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' => Ok(StandardPiece::BlackPawn),
            'r' => Ok(StandardPiece::BlackRook),
            'n' => Ok(StandardPiece::BlackKnight),
            'b' => Ok(StandardPiece::BlackBishop),
            'q' => Ok(StandardPiece::BlackQueen),
            'k' => Ok(StandardPiece::BlackKing),
            'P' => Ok(StandardPiece::WhitePawn),
            'R' => Ok(StandardPiece::WhiteRook),
            'N' => Ok(StandardPiece::WhiteKnight),
            'B' => Ok(StandardPiece::WhiteBishop),
            'Q' => Ok(StandardPiece::WhiteQueen),
            'K' => Ok(StandardPiece::WhiteKing),
            _ => Err(()),
        }
    }
}

impl Into<char> for StandardPiece {
    fn into(self) -> char {
        match self {
            Self::BlackPawn => 'p',
            Self::BlackRook => 'r',
            Self::BlackKnight => 'n',
            Self::BlackBishop => 'b',
            Self::BlackQueen => 'q',
            Self::BlackKing => 'k',
            Self::WhitePawn => 'P',
            Self::WhiteRook => 'R',
            Self::WhiteKnight => 'N',
            Self::WhiteBishop => 'B',
            Self::WhiteQueen => 'Q',
            Self::WhiteKing => 'K',
        }
    }
}

impl StandardPiece {
    pub fn into_utf8_chess_symbol(self) -> char {
        match self {
            Self::WhiteKing => '\u{2654}',
            Self::WhiteQueen => '\u{2655}',
            Self::WhiteRook => '\u{2656}',
            Self::WhiteBishop => '\u{2657}',
            Self::WhiteKnight => '\u{2658}',
            Self::WhitePawn => '\u{2659}',
            Self::BlackKing => '\u{265A}',
            Self::BlackQueen => '\u{265B}',
            Self::BlackRook => '\u{265C}',
            Self::BlackBishop => '\u{265D}',
            Self::BlackKnight => '\u{265E}',
            Self::BlackPawn => '\u{265F}',
        }
    }
}
