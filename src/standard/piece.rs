use crate::core::piece::Piece;

/// Represents the standard set of chess pieces.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StandardPiece {
    /// A black pawn.
    BlackPawn,
    /// A black rook.
    BlackRook,
    /// A black knight.
    BlackKnight,
    /// A black bishop.
    BlackBishop,
    /// A black queen.
    BlackQueen,
    /// A black king.
    BlackKing,
    /// A white pawn.
    WhitePawn,
    /// A white rook.
    WhiteRook,
    /// A white knight.
    WhiteKnight,
    /// A white bishop.
    WhiteBishop,
    /// A white queen.
    WhiteQueen,
    /// A white king.
    WhiteKing,
}

/// Represents the standard set of chess piece colors.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum StandardColor {
    /// The second-playing side.
    Black,
    /// The first-playing side.
    White,
}

/// Represents the standard set of chess piece kinds.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum StandardPieceKind {
    /// A pawn.
    Pawn,
    /// A rook.
    Rook,
    /// A knight.
    Knight,
    /// A bishop.
    Bishop,
    /// A queen.
    Queen,
    /// A king.
    King,
}

impl Piece for StandardPiece {
    type Color = StandardColor;
    type Kind = StandardPieceKind;

    fn color(&self) -> Self::Color {
        match self {
            Self::BlackPawn => StandardColor::Black,
            Self::BlackRook => StandardColor::Black,
            Self::BlackKnight => StandardColor::Black,
            Self::BlackBishop => StandardColor::Black,
            Self::BlackQueen => StandardColor::Black,
            Self::BlackKing => StandardColor::Black,
            Self::WhitePawn => StandardColor::White,
            Self::WhiteRook => StandardColor::White,
            Self::WhiteKnight => StandardColor::White,
            Self::WhiteBishop => StandardColor::White,
            Self::WhiteQueen => StandardColor::White,
            Self::WhiteKing => StandardColor::White,
        }
    }

    fn kind(&self) -> Self::Kind {
        match self {
            Self::BlackPawn => StandardPieceKind::Pawn,
            Self::BlackRook => StandardPieceKind::Rook,
            Self::BlackKnight => StandardPieceKind::Knight,
            Self::BlackBishop => StandardPieceKind::Bishop,
            Self::BlackQueen => StandardPieceKind::Queen,
            Self::BlackKing => StandardPieceKind::King,
            Self::WhitePawn => StandardPieceKind::Pawn,
            Self::WhiteRook => StandardPieceKind::Rook,
            Self::WhiteKnight => StandardPieceKind::Knight,
            Self::WhiteBishop => StandardPieceKind::Bishop,
            Self::WhiteQueen => StandardPieceKind::Queen,
            Self::WhiteKing => StandardPieceKind::King,
        }
    }

    fn new(color: Self::Color, kind: Self::Kind) -> Self {
        match (color, kind) {
            (StandardColor::Black, StandardPieceKind::Pawn) => Self::BlackPawn,
            (StandardColor::Black, StandardPieceKind::Rook) => Self::BlackRook,
            (StandardColor::Black, StandardPieceKind::Knight) => Self::BlackKnight,
            (StandardColor::Black, StandardPieceKind::Bishop) => Self::BlackBishop,
            (StandardColor::Black, StandardPieceKind::Queen) => Self::BlackQueen,
            (StandardColor::Black, StandardPieceKind::King) => Self::BlackKing,
            (StandardColor::White, StandardPieceKind::Pawn) => Self::WhitePawn,
            (StandardColor::White, StandardPieceKind::Rook) => Self::WhiteRook,
            (StandardColor::White, StandardPieceKind::Knight) => Self::WhiteKnight,
            (StandardColor::White, StandardPieceKind::Bishop) => Self::WhiteBishop,
            (StandardColor::White, StandardPieceKind::Queen) => Self::WhiteQueen,
            (StandardColor::White, StandardPieceKind::King) => Self::WhiteKing,
        }
    }
}

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
    /// Converts `self` into the corresponding UTF-8 [`char`].
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
