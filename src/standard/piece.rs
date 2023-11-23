use crate::core;

/// Represents the standard set of chess pieces.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Piece {
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
pub enum PieceKind {
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

impl core::Piece for Piece {
    type Color = Color;
    type Kind = PieceKind;

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
            Self::BlackPawn => PieceKind::Pawn,
            Self::BlackRook => PieceKind::Rook,
            Self::BlackKnight => PieceKind::Knight,
            Self::BlackBishop => PieceKind::Bishop,
            Self::BlackQueen => PieceKind::Queen,
            Self::BlackKing => PieceKind::King,
            Self::WhitePawn => PieceKind::Pawn,
            Self::WhiteRook => PieceKind::Rook,
            Self::WhiteKnight => PieceKind::Knight,
            Self::WhiteBishop => PieceKind::Bishop,
            Self::WhiteQueen => PieceKind::Queen,
            Self::WhiteKing => PieceKind::King,
        }
    }

    fn new(color: Self::Color, kind: Self::Kind) -> Self {
        match (color, kind) {
            (Color::Black, PieceKind::Pawn) => Self::BlackPawn,
            (Color::Black, PieceKind::Rook) => Self::BlackRook,
            (Color::Black, PieceKind::Knight) => Self::BlackKnight,
            (Color::Black, PieceKind::Bishop) => Self::BlackBishop,
            (Color::Black, PieceKind::Queen) => Self::BlackQueen,
            (Color::Black, PieceKind::King) => Self::BlackKing,
            (Color::White, PieceKind::Pawn) => Self::WhitePawn,
            (Color::White, PieceKind::Rook) => Self::WhiteRook,
            (Color::White, PieceKind::Knight) => Self::WhiteKnight,
            (Color::White, PieceKind::Bishop) => Self::WhiteBishop,
            (Color::White, PieceKind::Queen) => Self::WhiteQueen,
            (Color::White, PieceKind::King) => Self::WhiteKing,
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = (); // there's basically only one reason for this conversion to fail

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' => Ok(Piece::BlackPawn),
            'r' => Ok(Piece::BlackRook),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            'P' => Ok(Piece::WhitePawn),
            'R' => Ok(Piece::WhiteRook),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            _ => Err(()),
        }
    }
}

impl Into<char> for Piece {
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

impl Piece {
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
