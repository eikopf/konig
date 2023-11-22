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
pub enum Color {
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
    type Color = Color;
    type Kind = StandardPieceKind;

    fn color(&self) -> Self::Color {
        match self {
            Self::BlackPawn => Color::Black,
            Self::BlackRook => Color::Black,
            Self::BlackKnight => Color::Black,
            Self::BlackBishop => Color::Black,
            Self::BlackQueen => Color::Black,
            Self::BlackKing => Color::Black,
            Self::WhitePawn => Color::White,
            Self::WhiteRook => Color::White,
            Self::WhiteKnight => Color::White,
            Self::WhiteBishop => Color::White,
            Self::WhiteQueen => Color::White,
            Self::WhiteKing => Color::White,
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
            (Color::Black, StandardPieceKind::Pawn) => Self::BlackPawn,
            (Color::Black, StandardPieceKind::Rook) => Self::BlackRook,
            (Color::Black, StandardPieceKind::Knight) => Self::BlackKnight,
            (Color::Black, StandardPieceKind::Bishop) => Self::BlackBishop,
            (Color::Black, StandardPieceKind::Queen) => Self::BlackQueen,
            (Color::Black, StandardPieceKind::King) => Self::BlackKing,
            (Color::White, StandardPieceKind::Pawn) => Self::WhitePawn,
            (Color::White, StandardPieceKind::Rook) => Self::WhiteRook,
            (Color::White, StandardPieceKind::Knight) => Self::WhiteKnight,
            (Color::White, StandardPieceKind::Bishop) => Self::WhiteBishop,
            (Color::White, StandardPieceKind::Queen) => Self::WhiteQueen,
            (Color::White, StandardPieceKind::King) => Self::WhiteKing,
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
