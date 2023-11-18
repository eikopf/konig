use crate::core::board::Board;
use crate::core::index::Index;
use crate::core::piece::Piece;
use crate::standard::board::StandardCastlingPermissions;
use crate::standard::index::StandardIndex;
use crate::standard::piece::{StandardColor, StandardPiece, StandardPieceKind};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{one_of, space0, space1, u16, u8};
use nom::combinator::opt;
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::{pair, Tuple};
use nom::{Finish, IResult};
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

    /// Occurs if the FEN string has more than 8 ranks.
    #[error("the piece placement field had too many ranks: a valid FEN string has 8")]
    TooManyRanks,

    /// Occurs if the FEN string has less than 8 ranks.
    #[error("the piece placement field had too few ranks: a valid FEN string has 8")]
    TooFewRanks,

    /// Occurs if the FEN string doesn't end with (optional) whitespace.
    #[error("the given FEN string did not terminate with whitespace")]
    TrailingGarbage,

    /// Occurs if a particular error kind cannot be ascertained.
    #[error("an unknown error occurred while parsing a FEN string")]
    UnknownError,
}

type PieceArray = [Option<FenPiece>; 64];

/// Represents the data derived
/// from parsing a valid FEN string.
///
/// Parsing is provided via the `TryFrom<&'a str>`
/// impl, and the default FEN position is given
/// by the [`Default`] impl, i.e:
///
/// ```
/// use konig::io::fen::Fen;
///
/// let from_string =
///     Fen::try_from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
///     .unwrap();
/// let from_default = Fen::default();
/// assert_eq!(from_string, from_default); // <= succeeds
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fen {
    pieces: PieceArray,
    white_to_move: bool,
    castling_permissions: StandardCastlingPermissions,
    en_passant_square: Option<StandardIndex>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

impl Default for Fen {
    fn default() -> Self {
        let (_, res) =
            parse_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        res.unwrap() // safe unwrap
    }
}

impl<'a> TryFrom<&'a str> for Fen {
    type Error = FenParseError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match parse_fen_string(value).finish() {
            Ok((_, Ok(data))) => Ok(data),
            Ok((_, Err(err))) => Err(err),
            Err(_) => Err(FenParseError::UnknownError),
        }
    }
}

impl Fen {
    /// Returns the piece placement component of the FEN string
    /// as a [`Board`].
    pub fn as_board(
        self,
    ) -> impl Board<
        Piece = impl Piece + Into<StandardPiece> + From<StandardPiece> + std::fmt::Debug + Eq + Copy,
        Index = impl Index<MetricTarget = u8> + Into<StandardIndex> + From<StandardIndex>,
    > {
        FenBoard::from(self)
    }

    /// Returns `true` if it is white's turn to move, and `false` if it is black's.
    pub fn white_to_move(&self) -> bool {
        self.white_to_move
    }

    // TODO: define API for castling permissions

    /// Returns the index of the en passant target square, if it exists.
    pub fn en_passant_square(
        &self,
    ) -> Option<impl Index<MetricTarget = u8> + TryInto<StandardIndex>> {
        self.en_passant_square
    }

    /// Returns the value of the halfmove clock as a [`u8`], in
    /// which it is always guaranteed to fit.
    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    /// Returns the value of the fullmove counter as a [`u16`],
    /// in which it is always guaranteed to fit.
    pub fn fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }
}

/// Wraps a [`Fen`] to provide a [`Board`].
#[derive(Debug, PartialEq, Eq)]
struct FenBoard {
    data: Fen,
}

impl Board for FenBoard {
    type Index = FenIndex;
    type Piece = FenPiece;

    fn get_piece_at(&self, index: Self::Index) -> Option<&Self::Piece> {
        self.data.pieces[usize::from(index)].as_ref()
    }
}

impl Default for FenBoard {
    fn default() -> Self {
        Self {
            data: Fen::default(),
        }
    }
}

impl From<Fen> for FenBoard {
    fn from(value: Fen) -> Self {
        Self { data: value }
    }
}

/// The index type into the return type of [`Fen`]'s `as_board` method.
struct FenIndex(StandardIndex);

impl From<StandardIndex> for FenIndex {
    fn from(value: StandardIndex) -> Self {
        Self(value)
    }
}

impl From<FenIndex> for StandardIndex {
    fn from(value: FenIndex) -> Self {
        value.0
    }
}

impl From<FenIndex> for usize {
    fn from(value: FenIndex) -> Self {
        value.0.into()
    }
}

impl Index for FenIndex {
    type MetricTarget = u8;

    fn distance(a: Self, b: Self) -> Self::MetricTarget {
        // TODO: complete this function
        todo!()
    }
}

/// The piece type in the return type of [`Fen`]'s `as_board` method.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct FenPiece(StandardPiece);

impl From<StandardPiece> for FenPiece {
    fn from(value: StandardPiece) -> Self {
        Self(value)
    }
}

impl From<FenPiece> for StandardPiece {
    fn from(value: FenPiece) -> Self {
        value.0
    }
}

impl Piece for FenPiece {
    type Color = FenColor;

    type Kind = FenPieceKind;

    fn color(&self) -> Self::Color {
        self.0.color().into()
    }

    fn kind(&self) -> Self::Kind {
        self.0.kind().into()
    }

    fn new(color: Self::Color, kind: Self::Kind) -> Self
    where
        Self: Sized,
    {
        Self(StandardPiece::new(color.into(), kind.into()))
    }
}

/// The color type associated with a [`FenPiece`].
#[derive(Eq, PartialEq, Debug)]
struct FenColor(StandardColor);

impl From<StandardColor> for FenColor {
    fn from(value: StandardColor) -> Self {
        Self(value)
    }
}

impl From<FenColor> for StandardColor {
    fn from(value: FenColor) -> Self {
        value.0
    }
}

/// The kind type associated with a [`FenPiece`].
#[derive(Eq, PartialEq, Debug)]
struct FenPieceKind(StandardPieceKind);

impl From<StandardPieceKind> for FenPieceKind {
    fn from(value: StandardPieceKind) -> Self {
        Self(value)
    }
}

impl From<FenPieceKind> for StandardPieceKind {
    fn from(value: FenPieceKind) -> Self {
        value.0
    }
}

/// Entrypoint to FEN string parsing.
///
/// This function is made available in the public API via
/// [`Fen`]'s [`TryFrom`] implementation.
fn parse_fen_string(source: &str) -> IResult<&str, Result<Fen, FenParseError>> {
    // piece placement grammar
    // let digit17 = one_of::<&str, &str, nom::error::Error<_>>("1234567");
    // let white_piece = one_of("PNBRQK");
    // let black_piece = one_of("pnbrqk");
    // let piece = alt((white_piece, black_piece));
    // let rank_component = pair(opt(digit17), piece);
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
    let ep_rank = one_of("36");
    // let ep_square = pair(file, ep_rank);
    let en_passant_target_square = pair(one_of("-abcdefgh"), opt(ep_rank));

    // halfmove clock grammar
    let halfmove_clock = u8;

    // fullmove counter grammar
    let fullmove_counter = u16;

    // finally parse
    let mut fen_parser = (
        piece_placement,
        space1,
        side_to_move,
        space1,
        castling_ability,
        space1,
        en_passant_target_square,
        space1,
        halfmove_clock,
        space1,
        fullmove_counter,
        space0,
    );

    let (tail, (pieces, _, side, _, castle, _, ep, _, half, _, full, _)) =
        fen_parser.parse(source)?;

    // error handling
    if tail.len() > 0 {
        return Ok((tail, Err(FenParseError::TrailingGarbage)));
    }

    if pieces.len() > 8 {
        return Ok((tail, Err(FenParseError::TooManyRanks)));
    } else if pieces.len() < 8 {
        return Ok((tail, Err(FenParseError::TooFewRanks)));
    }

    if ep.0 == '-' && ep.1 != None {
        return Ok((
            tail,
            Err(FenParseError::InvalidEnPassantTargetSquareComponent),
        ));
    } else if ep.0 != '-' && ep.1 == None {
        return Ok((
            tail,
            Err(FenParseError::InvalidEnPassantTargetSquareComponent),
        ));
    }

    if half > 100 {
        return Ok((tail, Err(FenParseError::InvalidHalfmoveClockComponent)));
    }

    return Ok((
        tail,
        Ok(Fen {
            pieces: expand_piece_placement(pieces),
            white_to_move: side == 'w',
            castling_permissions: expand_castling_permissions(castle),
            en_passant_square: expand_en_passant_target_square(ep),
            halfmove_clock: half,
            fullmove_counter: full,
        }),
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
                    pieces[i] = Some(StandardPiece::try_from(piece).unwrap().into());
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

/// Converts a parsed en passant target square component into an [`Option<StandardIndex>`].
///
/// This function assumes its input is valid, and will panic otherwise.
fn expand_en_passant_target_square(source: (char, Option<char>)) -> Option<StandardIndex> {
    match source {
        ('-', None) => None,
        (rank, Some('3')) => Some(StandardIndex::try_from(16 + (rank as u8) - 97).unwrap()),
        (rank, Some('6')) => Some(StandardIndex::try_from(40 + (rank as u8) - 97).unwrap()),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::standard::board::StandardBoard;

    #[test]
    fn check_fen_parser_on_initial_position() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let data = parse_fen_string(start).unwrap().1.unwrap();
        let default = StandardBoard::default();

        for i in 0..=63 {
            let index = StandardIndex::try_from(i as u8).unwrap();
            assert_eq!(
                default.get_piece_at(index).map(|x| x.to_owned()),
                data.as_board()
                    .get_piece_at(index.into())
                    .map(|x| x.to_owned().into())
            )
        }

        assert_eq!(data.white_to_move, true);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(data.en_passant_square, None);
        assert_eq!(data.halfmove_clock, 0);
        assert_eq!(data.fullmove_counter, 1);
    }

    #[test]
    fn check_fen_parser_on_several_moves() {
        // This test is based on the example game from https://www.chessprogramming.org/Forsyth-Edwards_Notation

        // INITIAL STATE
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let data = parse_fen_string(start).unwrap().1.unwrap();
        let default = StandardBoard::default();

        // for each position on the board, check that the pieces match
        for i in 0..=63 {
            let index = StandardIndex::try_from(i as u8).unwrap();
            assert_eq!(
                default.get_piece_at(index).map(|x| x.to_owned()),
                data.as_board()
                    .get_piece_at(index.into())
                    .map(|x| x.to_owned().into())
            )
        }

        assert_eq!(data.white_to_move, true);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(data.en_passant_square, None);
        assert_eq!(data.halfmove_clock, 0);
        assert_eq!(data.fullmove_counter, 1);

        // GAME AFTER 1. e4
        let move1 = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let data = parse_fen_string(move1).unwrap().1.unwrap();

        assert_eq!(
            data.as_board()
                .get_piece_at(StandardIndex::try_from(28u8).unwrap().into()),
            Some(&StandardPiece::WhitePawn.into())
        );
        assert_eq!(data.white_to_move, false);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(
            data.en_passant_square,
            Some(StandardIndex::try_from(20u8).unwrap())
        );
        assert_eq!(data.halfmove_clock, 0);
        assert_eq!(data.fullmove_counter, 1);

        // GAME AFTER 1. e4 c5
        let move2 = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        let data = parse_fen_string(move2).unwrap().1.unwrap();
        assert_eq!(
            data.as_board()
                .get_piece_at(StandardIndex::try_from(28u8).unwrap().into()),
            Some(&StandardPiece::WhitePawn.into())
        );
        assert_eq!(
            data.as_board()
                .get_piece_at(StandardIndex::try_from(34u8).unwrap().into()),
            Some(&StandardPiece::BlackPawn.into())
        );
        assert_eq!(
            // check black pawn has properly moved
            data.as_board()
                .get_piece_at(StandardIndex::try_from(50u8).unwrap().into()),
            None
        );

        assert_eq!(data.white_to_move, true);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(
            data.en_passant_square,
            Some(StandardIndex::try_from(42u8).unwrap())
        );
        assert_eq!(data.halfmove_clock, 0);
        assert_eq!(data.fullmove_counter, 2);

        // GAME AFTER 1. e4 c5 2. Nf3
        let move3 = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let data = parse_fen_string(move3).unwrap().1.unwrap();
        assert_eq!(
            data.as_board()
                .get_piece_at(StandardIndex::try_from(28u8).unwrap().into()),
            Some(&StandardPiece::WhitePawn.into())
        );
        assert_eq!(
            data.as_board()
                .get_piece_at(StandardIndex::try_from(34u8).unwrap().into()),
            Some(&StandardPiece::BlackPawn.into())
        );
        assert_eq!(
            // check black pawn has properly moved
            data.as_board()
                .get_piece_at(StandardIndex::try_from(50u8).unwrap().into()),
            None
        );
        assert_eq!(
            data.as_board()
                .get_piece_at(StandardIndex::try_from(21u8).unwrap().into()),
            Some(&StandardPiece::WhiteKnight.into())
        );

        assert_eq!(data.white_to_move, false);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(data.en_passant_square, None);
        assert_eq!(data.halfmove_clock, 1);
        assert_eq!(data.fullmove_counter, 2);
    }
}
