use crate::core::board::StaticBoard;
use crate::standard::board::StandardCastlingPermissions;
use crate::standard::index::StandardIndex;
use crate::standard::piece::StandardPiece;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of, u16, u8};
use nom::combinator::opt;
use nom::error::{Error, ParseError};
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::pair;
use nom::sequence::Tuple;
use nom::IResult;
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

type PieceArray = [Option<StandardPiece>; 64];

/// Represents the data derived
/// from parsing a valid FEN string.
#[derive(Debug, PartialEq, Eq)]
pub struct FenData {
    pieces: PieceArray,
    white_to_move: bool,
    castling_permissions: StandardCastlingPermissions,
    en_passant_square: Option<StandardIndex>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

impl Default for FenData {
    fn default() -> Self {
        todo!()
    }
}

impl TryFrom<&str> for FenData {
    type Error = FenParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // TODO: impl using nom
        todo!()
    }
}

impl FenData {
    /// Returns a relevant subset of [`FenData`] as a [`StaticBoard`].
    pub fn as_static_board(self) -> impl StaticBoard<Index = StandardIndex, Piece = StandardPiece> {
        FenBoard::from(self)
    }
}

/// Wraps a [`FenData`] to provide an `impl [StaticBoard]`.
#[derive(Debug, PartialEq, Eq)]
struct FenBoard {
    data: FenData,
}

impl StaticBoard for FenBoard {
    type Index = StandardIndex;
    type Piece = StandardPiece;
}

impl Default for FenBoard {
    fn default() -> Self {
        Self {
            data: FenData::default(),
        }
    }
}

impl From<FenData> for FenBoard {
    fn from(value: FenData) -> Self {
        Self { data: value }
    }
}

impl std::ops::Index<StandardIndex> for FenBoard {
    type Output = Option<StandardPiece>;

    fn index(&self, index: StandardIndex) -> &Self::Output {
        let i: usize = index.into();
        &self.data.pieces[i]
    }
}

fn parse_fen_string(source: &str) -> IResult<&str, FenData> {
    // piece placement grammar
    let digit17 = one_of::<&str, &str, nom::error::Error<_>>("1234567");
    let white_piece = one_of("PNBRQK");
    let black_piece = one_of("pnbrqk");
    let piece = alt((white_piece, black_piece));
    let rank_component = pair(opt(digit17), piece);
    let rank = many_m_n(1, 8, one_of("12345678pnbrqkPNBRQK"));
    let piece_placement = separated_list1(tag("/"), rank);

    // side to move grammar
    let side_to_move = one_of("wb");

    // castling ability grammar
    let castling_ability = alt((
        // the order of the tags is loadbearing
        tag("-"),
        tag("KQkq"),
        tag("Qkq"),
        tag("Kkq"),
        tag("KQq"),
        tag("KQk"),
        tag("kq"),
        tag("Qq"),
        tag("Qk"),
        tag("Kq"),
        tag("Kk"),
        tag("KQ"),
        tag("q"),
        tag("k"),
        tag("Q"),
        tag("K"),
    ));

    // en passant target square grammar
    // let ep_rank = one_of("36");
    // let file = one_of("abcdefgh");
    // let ep_square = pair(file, ep_rank);
    let en_passant_target_square = alt((tag("-"), digit1));

    // halfmove clock grammar
    let halfmove_clock = u8;

    // fullmove counter grammar
    let fullmove_counter = u16;

    // finally parse
    let mut fen_parser = (
        piece_placement,
        tag(" "),
        side_to_move,
        tag(" "),
        castling_ability,
        tag(" "),
        en_passant_target_square,
        tag(" "),
        halfmove_clock,
        tag(" "),
        fullmove_counter,
    );

    let (tail, (pieces, _, side, _, castle, _, ep, _, half, _, full)) = fen_parser.parse(source)?;

    return Ok((
        tail,
        FenData {
            pieces: expand_piece_placement(pieces),
            white_to_move: side == 'w',
            castling_permissions: expand_castling_permissions(castle),
            en_passant_square: None, // TODO: implement algebraic notation in std::index
            halfmove_clock: half,
            fullmove_counter: full,
        },
    ));
}

/// Converts a parsed piece placement component into an array of pieces.
///
/// This function assumes its input is valid, and will panic otherwise.
fn expand_piece_placement(source: Vec<Vec<char>>) -> PieceArray {
    let mut pieces = [None; 64];
    let mut i: usize = 0; // write index into pieces array

    for rank in source.into_iter().rev() {
        for c in rank {
            match c {
                blank @ '1'..='8' => i += blank.to_digit(10).unwrap() as usize,
                piece @ _ => {
                    pieces[i] = Some(StandardPiece::try_from(piece).unwrap());
                    i += 1;
                }
            }
        }
    }

    pieces
}

/// Converts a parsed castling permissions component into a [`StandardCastlingPermissions`].
///
/// This function assumes its input is valid, and will panic otherwise.
fn expand_castling_permissions(source: &str) -> StandardCastlingPermissions {
    match source {
        "-" => StandardCastlingPermissions::none(),

        "K" => StandardCastlingPermissions {
            white_king_side: true,
            ..StandardCastlingPermissions::none()
        },

        "Q" => StandardCastlingPermissions {
            white_queen_side: true,
            ..StandardCastlingPermissions::none()
        },

        "k" => StandardCastlingPermissions {
            black_king_side: true,
            ..StandardCastlingPermissions::none()
        },

        "q" => StandardCastlingPermissions {
            black_queen_side: true,
            ..StandardCastlingPermissions::none()
        },

        "KQ" => StandardCastlingPermissions {
            white_king_side: true,
            white_queen_side: true,
            ..StandardCastlingPermissions::none()
        },

        "Kk" => StandardCastlingPermissions {
            white_king_side: true,
            black_king_side: true,
            ..StandardCastlingPermissions::none()
        },

        "Kq" => StandardCastlingPermissions {
            white_king_side: true,
            black_queen_side: true,
            ..StandardCastlingPermissions::none()
        },

        "Qk" => StandardCastlingPermissions {
            white_queen_side: true,
            black_king_side: true,
            ..StandardCastlingPermissions::none()
        },

        "Qq" => StandardCastlingPermissions {
            white_queen_side: true,
            black_queen_side: true,
            ..StandardCastlingPermissions::none()
        },

        "kq" => StandardCastlingPermissions {
            black_king_side: true,
            black_queen_side: true,
            ..StandardCastlingPermissions::none()
        },

        "KQk" => StandardCastlingPermissions {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: false,
        },

        "KQq" => StandardCastlingPermissions {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: false,
            black_queen_side: true,
        },

        "Kkq" => StandardCastlingPermissions {
            white_king_side: true,
            white_queen_side: false,
            black_king_side: true,
            black_queen_side: true,
        },

        "Qkq" => StandardCastlingPermissions {
            white_king_side: false,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        },

        "KQkq" => StandardCastlingPermissions::default(),

        _ => panic!("bad input"),
    }
}

#[cfg(test)]
mod tests {
    use crate::standard::board::StandardBoard;

    use super::*;

    #[test]
    fn check_fen_parser_and_fail() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let (_, data) = parse_fen_string(start).unwrap();
        let default = StandardBoard::default();

        // for each position on the board, check that the pieces match
        for i in 0..=63 {
            let index = StandardIndex::try_from(i).unwrap();
            println!("{:?}", data.pieces[i]);
            println!("{:?}", default[index]);
            // assert_eq!(data.as_static_board()[index], default[index])
        }

        assert!(false);
    }
}
