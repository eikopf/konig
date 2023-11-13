use crate::core::board::StaticBoard;
use crate::standard::index::StandardIndex;
use crate::standard::piece::StandardPiece;

use thiserror::Error;

/// Represents the ways in which a FEN string may be invalid.
#[derive(Error, Debug)]
pub enum FenParseError {
    /// Occurs if the first component of the FEN string is invalid.
    #[error("invalid FEN representation of piece placement")]
    InvalidPositionComponent,

    /// Occurs if the second component of the FEN string is invalid.
    #[error("invalid FEN representation of the piece to move: expected 'w' or 'b'")]
    InvalidPieceToMoveComponent,

    /// Occurs if the third component of the FEN string is invalid.
    #[error("invalid FEN representation of castling permissions")]
    InvalidCastlingPermissionsComponent,

    /// Occurs if the fourth component of the FEN string is invalid.
    #[error("invalid FEN representation of the en passant target square")]
    InvalidEnPassantTargetSquareComponent,

    /// Occurs if the fifth component of the FEN string is invalid.
    #[error("invalid FEN representation of the halfmove clock")]
    InvalidHalfmoveClockComponent,

    /// Occurs if the sixth component of the FEN string is invalid.
    #[error("invalid FEN representation of the fullmove counter")]
    InvalidFullmoveCounterComponent,

    /// Occurs if the FEN string has less than six fields.
    #[error("failed to parse enough fields: a valid FEN string has 6")]
    TooFewFields,

    /// Occurs if the FEN string has more than six fields.
    #[error("parsed too many fields: a valid FEN string has 6")]
    TooManyFields,
}

// TODO: move castling permissions to crate::standard::board

/// Represents the possible castling
/// permissions described by a FEN
/// string.
#[derive(Debug, PartialEq, Eq)]
pub struct CastlingPermissions {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingPermissions {
    /// Convienience function for the empty set of castling permissions.
    pub fn none() -> CastlingPermissions {
        CastlingPermissions {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }
}

impl Default for CastlingPermissions {
    fn default() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

/// Represents the data derived
/// from parsing a valid FEN string.
#[derive(Debug, PartialEq, Eq)]
pub struct FenData {
    pieces: [StandardPiece; 64],
    white_to_move: bool,
    castling_permissions: CastlingPermissions,
    en_passant_square: Option<StandardIndex>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

struct FenBoard {}

impl StaticBoard for FenBoard {
    type Index = StandardIndex;
    type Piece = StandardPiece;
}

impl Default for FenBoard {
    fn default() -> Self {
        // TODO: implement fen board default correctly
        Self {}
    }
}

impl std::ops::Index<StandardIndex> for FenBoard {
    // TODO: implement fen board indexing correctly
    type Output = ();

    fn index(&self, index: StandardIndex) -> &Self::Output {
        todo!()
    }
}

impl FenData {
    /// Returns a relevant subset of FenData as a Board
    pub fn into_board(&self) -> impl StaticBoard {
        FenBoard::default()
    }
}

// impl TryFrom<&str> for FenData {
//     type Error = FenParseError;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         if value.split(' ').count() > 6 {
//             return Err(FenParseError::TooManyFields);
//         }

//         let mut source_iterator = value.split(' ');
//         let ret = || return FenParseError::TooFewFields;

//         Ok(FenData {
//             position: try_parse_piece_placement(source_iterator.next().ok_or_else(ret)?)?,
//             side_to_move: try_parse_side_to_move(source_iterator.next().ok_or_else(ret)?)?,
//             castling_permissions: try_parse_castling_permissions(
//                 source_iterator.next().ok_or_else(ret)?,
//             )?,
//             en_passant_target_square: try_parse_en_passant_target_square(
//                 source_iterator.next().ok_or_else(ret)?,
//             )?,
//             halfmove_clock: try_parse_halfmove_clock(source_iterator.next().ok_or_else(ret)?)?,
//             fullmove_counter: try_parse_fullmove_counter(source_iterator.next().ok_or_else(ret)?)?,
//         })
//     }
// }

// /// Parses the "Piece placement" (1st) component
// /// of a FEN string, returning a valid `Position`
// /// or a `FenParseError`.
// fn try_parse_piece_placement(source: &str) -> Result<Position, FenParseError> {
//     let mut position = Position::empty();
//     let fen_index_iterator = &mut FenIndexIterator::new();

//     for char in source.chars() {
//         match char {
//             p @ 'p'
//             | p @ 'P'
//             | p @ 'r'
//             | p @ 'R'
//             | p @ 'b'
//             | p @ 'B'
//             | p @ 'q'
//             | p @ 'Q'
//             | p @ 'k'
//             | p @ 'K'
//             | p @ 'n'
//             | p @ 'N' => {
//                 let piece = Piece::try_from(p).unwrap(); // guaranteed never to panic
//                 let board_index = fen_index_iterator
//                     .next()
//                     .ok_or(FenParseError::InvalidPositionComponent)?;

//                 position
//                     .try_write(board_index, piece)
//                     .or_else(|_| return Err(FenParseError::InvalidPositionComponent))?;
//             }

//             '/' => continue,
//             fill @ '1'..='8' => {
//                 let _ = fen_index_iterator.advance_by(
//                     fill.to_digit(10)
//                         .unwrap()
//                         .try_into()
//                         .or_else(|_| return Err(FenParseError::InvalidPositionComponent))?,
//                 );
//             }
//             _ => return Err(FenParseError::InvalidPositionComponent),
//         }
//     }

//     Ok(position)
// }

// /// Parses the "Side to move" (2nd) component
// /// of a FEN string, returning a `PieceColor`
// /// or a `FenParseError`.
// fn try_parse_side_to_move(source: &str) -> Result<PieceColor, FenParseError> {
//     if source.len() != 1 {
//         return Err(FenParseError::InvalidPieceToMoveComponent);
//     };

//     match source.chars().next() {
//         Some('w') => Ok(PieceColor::White),
//         Some('b') => Ok(PieceColor::Black),
//         _ => Err(FenParseError::InvalidPieceToMoveComponent),
//     }
// }

// /// Parses the "Castling  permissions" (3rd)
// /// component of a FEN string, returning a
// /// `CastlingPermissions` or a `FenParseError`.
// fn try_parse_castling_permissions(source: &str) -> Result<CastlingPermissions, FenParseError> {
//     match source {
//         "-" => Ok(CastlingPermissions::none()),

//         "K" => Ok(CastlingPermissions {
//             white_king_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "Q" => Ok(CastlingPermissions {
//             white_queen_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "k" => Ok(CastlingPermissions {
//             black_king_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "q" => Ok(CastlingPermissions {
//             black_queen_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "KQ" => Ok(CastlingPermissions {
//             white_king_side: true,
//             white_queen_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "Kk" => Ok(CastlingPermissions {
//             white_king_side: true,
//             black_king_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "Kq" => Ok(CastlingPermissions {
//             white_king_side: true,
//             black_queen_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "Qk" => Ok(CastlingPermissions {
//             white_queen_side: true,
//             black_king_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "Qq" => Ok(CastlingPermissions {
//             white_queen_side: true,
//             black_queen_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "kq" => Ok(CastlingPermissions {
//             black_king_side: true,
//             black_queen_side: true,
//             ..CastlingPermissions::none()
//         }),

//         "KQk" => Ok(CastlingPermissions {
//             white_king_side: true,
//             white_queen_side: true,
//             black_king_side: true,
//             black_queen_side: false,
//         }),

//         "KQq" => Ok(CastlingPermissions {
//             white_king_side: true,
//             white_queen_side: true,
//             black_king_side: false,
//             black_queen_side: true,
//         }),

//         "Kkq" => Ok(CastlingPermissions {
//             white_king_side: true,
//             white_queen_side: false,
//             black_king_side: true,
//             black_queen_side: true,
//         }),

//         "Qkq" => Ok(CastlingPermissions {
//             white_king_side: false,
//             white_queen_side: true,
//             black_king_side: true,
//             black_queen_side: true,
//         }),

//         "KQkq" => Ok(CastlingPermissions::default()),

//         _ => Err(FenParseError::InvalidCastlingPermissionsComponent),
//     }
// }

// /// Parses the "En passant target square" (4th)
// /// component of a FEN string, returning an
// /// `Option<u8>` or a `FenParseError`.
// fn try_parse_en_passant_target_square(source: &str) -> Result<Option<u8>, FenParseError> {
//     if source == "-" {
//         return Ok(None);
//     };
//     if source.len() != 2 {
//         return Err(FenParseError::InvalidEnPassantTargetSquareComponent);
//     };

//     let mut source_char_iterator = source.chars();
//     let rank = match source_char_iterator.next() {
//         Some(rank_char @ 'a'..='h') => rank_char,
//         _ => return Err(FenParseError::InvalidEnPassantTargetSquareComponent),
//     };

//     let file = match source_char_iterator.next() {
//         Some('3') => 3,
//         Some('6') => 6,
//         _ => return Err(FenParseError::InvalidEnPassantTargetSquareComponent),
//     };

//     let index = (rank as u8) * 8 + (file as u8);
//     return Ok(Some(index));
// }

// /// Parses the "Halfmove clock" (5th) component
// /// of a FEN string, returning a `u8` or a `FenParseError`.
// ///
// /// The resulting `u8` will be in the range \[0, 50\].
// fn try_parse_halfmove_clock(source: &str) -> Result<u8, FenParseError> {
//     match source.parse::<u8>() {
//         Ok(value @ 0..=50) => Ok(value),
//         _ => Err(FenParseError::InvalidHalfmoveClockComponent),
//     }
// }

// /// Parses the "Fullmove counter" (6th) component
// /// of a FEN string, returning a `u16` or `FenParseError`.
// ///
// /// In practice, the fullmove counter will never reach
// /// a size even close to 2^16, so a `u16` is sufficient
// /// to describe all valid inputs.
// fn try_parse_fullmove_counter(source: &str) -> Result<u16, FenParseError> {
//     match source.parse::<u16>() {
//         Ok(value @ 1..) => Ok(value),
//         _ => Err(FenParseError::InvalidFullmoveCounterComponent),
//     }
// }
