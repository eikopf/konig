use crate::core::{position::Position, piece::{Piece, PieceColor}};
use thiserror::Error;

/// An error denoting the ways
/// in which a FEN string may
/// be invalid.
#[derive(Error, Debug)]
pub enum FenParseError {
    #[error("invalid FEN representation of piece placement")]
    InvalidPositionComponent,

    #[error("invalid FEN representation of the piece to move: expected 'w' or 'b'")]
    InvalidPieceToMoveComponent,

    #[error("invalid FEN representation of castling permissions")]
    InvalidCastlingPermissionsComponent,

    #[error("invalid FEN representation of the en passant target square")]
    InvalidEnPassantTargetSquareComponent,

    #[error("invalid FEN representation of the halfmove clock")]
    InvalidHalfmoveClockComponent,

    #[error("invalid FEN representation of the fullmove counter")]
    InvalidFullmoveCounterComponent,

    #[error("failed to parse enough fields: a valid FEN string has 6")]
    TooFewFields,

    #[error("parsed too many fields: a valid FEN string has 6")]
    TooManyFields,

    #[error("invalid FEN string")]
    Unknown,
}

/// Provides a sequence of board indices
/// corresponding to the order in which
/// they appear in a FEN string.
struct FenIndexIterator {
    index: u8,
    rank_index: u8
}

impl Iterator for FenIndexIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 7 {
            return None
        };

        if self.rank_index == 8 {
            self.index -= 15;
            self.rank_index = 1;
        } else {
            self.index += 1;
            self.rank_index += 1;
        }

        return Some(self.index)
    }
}

impl FenIndexIterator {

    /// Returns a FenIndexIterator with
    /// the expected default values.
    pub fn new() -> FenIndexIterator {
        FenIndexIterator {
            index: 55,
            rank_index: 0
        }
    }

    /// Advances by `steps` steps by
    /// repeatedly calling next and
    /// discarding the result.
    ///
    /// This function will return an
    /// error if it tries to advance
    /// beyond the end of the iterator.
    pub fn advance_by(&mut self, steps: usize) -> Result<(), ()> {
        for _ in 0..steps {
            match self.next() {
                Some(_) => continue,
                None => return Err(()),
            }
        }

        Ok(())
    }
}

/// Represents the possible castling
/// permissions described by a FEN
/// string.
///
/// This struct is 4 bytes in size,
/// an 8-fold increase over the
/// corresponding Zig implementation
/// encoded as a u4.
#[derive(Debug, PartialEq, Eq)]
pub struct CastlingPermissions {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingPermissions {
    #[inline(always)]
    pub fn none() -> CastlingPermissions {
        CastlingPermissions {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false
        }
    }

    #[inline(always)]
    pub fn default() -> CastlingPermissions {
        CastlingPermissions {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

/// Represents the data derived
/// from parsing a valid FEN string.
///
/// This struct is 33.3% larger than
/// the equivalent Zig implementation.
#[derive(Debug, PartialEq, Eq)]
pub struct FenData {
    pub position: Position,
    pub side_to_move: PieceColor,
    pub castling_permissions: CastlingPermissions,
    pub en_passant_target_square: Option<u8>,
    pub halfmove_clock: u8,
    pub fullmove_counter: u16,
}

impl TryFrom<&str> for FenData {
    type Error = FenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.split(' ').count() > 6 { return Err(FenParseError::TooManyFields) }

        let mut source_iterator = value.split(' ');
        let ret = || { return FenParseError::TooFewFields };

        Ok(FenData {
            position: try_parse_piece_placement(source_iterator.next().ok_or_else(ret)?)?,
            side_to_move: try_parse_side_to_move(source_iterator.next().ok_or_else(ret)?)?,
            castling_permissions: try_parse_castling_permissions(source_iterator.next().ok_or_else(ret)?)?,
            en_passant_target_square: try_parse_en_passant_target_square(source_iterator.next().ok_or_else(ret)?)?,
            halfmove_clock: try_parse_halfmove_clock(source_iterator.next().ok_or_else(ret)?)?,
            fullmove_counter: try_parse_fullmove_counter(source_iterator.next().ok_or_else(ret)?)?,
        })
    }
}

/// Parses the "Piece placement" (1st) component
/// of a FEN string, returning a valid `Position`
/// or a `FenParseError`.
fn try_parse_piece_placement(source: &str) -> Result<Position, FenParseError> {
    let mut position = Position::empty();
    let fen_index_iterator = &mut FenIndexIterator::new();

    for char in source.chars() {
        match char {
           p @ 'p' | p @ 'P' | p @ 'r' | p @ 'R' |
           p @ 'b' | p @ 'B' | p @ 'q' | p @ 'Q' |
           p @ 'k' | p @ 'K' | p @ 'n' | p @ 'N' => {
               let piece = Piece::try_from(p).unwrap(); // guaranteed never to panic
               let board_index = fen_index_iterator
                   .next()
                   .ok_or(FenParseError::InvalidPositionComponent)?;

               position.try_write(board_index, piece).or_else(|_| {
                   return Err(FenParseError::InvalidPositionComponent)
               })?;
           }

        '/' => continue,
        fill @ '1'..='8' => {
            let _ = fen_index_iterator.advance_by(
                fill.to_digit(10)
                    .unwrap()
                    .try_into()
                    .or_else(|_| {
                        return Err(FenParseError::InvalidPositionComponent)
                    })?
            );
        }
        _ => return Err(FenParseError::InvalidPositionComponent),
       }
    }

    Ok(position)
}

/// Parses the "Side to move" (2nd) component
/// of a FEN string, returning a `PieceColor`
/// or a `FenParseError`.
fn try_parse_side_to_move(source: &str) -> Result<PieceColor, FenParseError> {
    if source.len() != 1 { return Err(FenParseError::InvalidPieceToMoveComponent) };

    match source.chars().next() {
        Some('w') => Ok(PieceColor::White),
        Some('b') => Ok(PieceColor::Black),
        _ => Err(FenParseError::InvalidPieceToMoveComponent)
    }
}

/// Parses the "Castling  permissions" (3rd)
/// component of a FEN string, returning a
/// `CastlingPermissions` or a `FenParseError`.
fn try_parse_castling_permissions(source: &str) -> Result<CastlingPermissions, FenParseError> {
    match source {
        "-" => Ok(CastlingPermissions::none()),

        "K" => Ok(CastlingPermissions{
            white_king_side: true,
            ..CastlingPermissions::none()
        }),

        "Q" => Ok(CastlingPermissions{
            white_queen_side: true,
            ..CastlingPermissions::none()
        }),

        "k" => Ok(CastlingPermissions{
            black_king_side: true,
            ..CastlingPermissions::none()
        }),

        "q" => Ok(CastlingPermissions{
            black_queen_side: true,
            ..CastlingPermissions::none()
        }),

        "KQ" => Ok(CastlingPermissions{
            white_king_side: true,
            white_queen_side: true,
            ..CastlingPermissions::none()
        }),

        "Kk" => Ok(CastlingPermissions{
            white_king_side: true,
            black_king_side: true,
            ..CastlingPermissions::none()
        }),

        "Kq" => Ok(CastlingPermissions{
            white_king_side: true,
            black_queen_side: true,
            ..CastlingPermissions::none()
        }),

        "Qk" => Ok(CastlingPermissions{
            white_queen_side: true,
            black_king_side: true,
            ..CastlingPermissions::none()
        }),

        "Qq" => Ok(CastlingPermissions{
            white_queen_side: true,
            black_queen_side: true,
            ..CastlingPermissions::none()
        }),

        "kq" => Ok(CastlingPermissions{
            black_king_side: true,
            black_queen_side: true,
            ..CastlingPermissions::none()
        }),

        "KQk" => Ok(CastlingPermissions{
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: false,
        }),

        "KQq" => Ok(CastlingPermissions{
            white_king_side: true,
            white_queen_side: true,
            black_king_side: false,
            black_queen_side: true,
        }),

        "Kkq" => Ok(CastlingPermissions{
            white_king_side: true,
            white_queen_side: false,
            black_king_side: true,
            black_queen_side: true,
        }),

        "Qkq" => Ok(CastlingPermissions{
            white_king_side: false,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }),

        "KQkq" => Ok(CastlingPermissions::default()),

        _ => Err(FenParseError::InvalidCastlingPermissionsComponent),
    }
}

/// Parses the "En passant target square" (4th)
/// component of a FEN string, returning an
/// `Option<u8>` or a `FenParseError`.
fn try_parse_en_passant_target_square(source: &str) -> Result<Option<u8>, FenParseError> {
    if source == "-" { return Ok(None) };
    if source.len() != 2 { return Err(FenParseError::InvalidEnPassantTargetSquareComponent) };

    let mut source_char_iterator = source.chars();
    let rank = match source_char_iterator.next() {
        Some(rank_char @ 'a'..='h') => rank_char,
        _ => return Err(FenParseError::InvalidEnPassantTargetSquareComponent)
    };

    let file = match source_char_iterator.next() {
        Some('3') => 3,
        Some('6') => 6,
        _ => return Err(FenParseError::InvalidEnPassantTargetSquareComponent)
    };

    let index = (rank as u8) * 8 + (file as u8);
    return Ok(Some(index))
}

/// Parses the "Halfmove clock" (5th) component
/// of a FEN string, returning a `u8` or a `FenParseError`.
///
/// The resulting `u8` will be in the range \[0, 50\].
fn try_parse_halfmove_clock(source: &str) -> Result<u8, FenParseError> {
    match source.parse::<u8>() {
        Ok(value @ 0..=50) => Ok(value),
        _ => Err(FenParseError::InvalidHalfmoveClockComponent),
    }
}

/// Parses the "Fullmove counter" (6th) component
/// of a FEN string, returning a `u16` or `FenParseError`.
///
/// In practice, the fullmove counter will never reach
/// a size even close to 2^16, so a `u16` is sufficient
/// to describe all valid inputs.
fn try_parse_fullmove_counter(source: &str) -> Result<u16, FenParseError> {
    match source.parse::<u16>() {
        Ok(value @ 1..) => Ok(value),
        _ => Err(FenParseError::InvalidFullmoveCounterComponent),
    }
}

#[cfg(test)]
mod tests {
    use crate::{core::pieces::{PieceColor, Piece, PieceType}, io::fen::CastlingPermissions};
    use super::{FenData, FenIndexIterator};


    #[test]
    fn validate_fen_parsing() {
        let initial_state = FenData::try_from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        ).unwrap();

        println!("{:?}", initial_state);

        assert_eq!(initial_state.position.try_get(0), Ok(
            Piece {
                color: PieceColor::White,
                kind: PieceType::Rook,
            }
        ));

        assert_eq!(initial_state.position.try_get(63), Ok(
            Piece {
                color: PieceColor::Black,
                kind: PieceType::Rook,
            }
        ));

        assert_eq!(initial_state.side_to_move, PieceColor::White);
        assert_eq!(initial_state.castling_permissions, CastlingPermissions::default());
        assert_eq!(initial_state.en_passant_target_square, None);
        assert_eq!(initial_state.halfmove_clock, 0);
        assert_eq!(initial_state.fullmove_counter, 1);
    }

    #[test]
    fn debug_fen_index_iterator() {
        let fii = &mut FenIndexIterator::new();

        assert_eq!(fii.next(), Some(56));
        assert_eq!(fii.next(), Some(57));
        let _ = fii.advance_by(7).unwrap();
        assert_eq!(fii.next(), Some(49));
    }
}
