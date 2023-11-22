use crate::core::board::{Board, Standard};
use crate::core::index::Index;
use crate::core::piece::Piece;
use crate::standard::board::{StandardBoard, StandardCastlingPermissions};
use crate::standard::index::StandardIndex;
use crate::standard::piece::{StandardColor, StandardPiece};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of, space1, u16, u8};
use nom::combinator::{eof, success, verify};
use nom::error::VerboseError;
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::{pair, Tuple};
use nom::{Finish, IResult, Parser};
use thiserror::Error;

/// Represents the ways in which a FEN string may be invalid.
#[derive(Error, Debug)]
enum ParseError {
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

type PieceArray = [Option<StandardPiece>; 64];

/// Represents the data derived
/// from parsing a valid FEN string.
///
/// Parsing is provided via the `TryFrom<&'a str>`
/// impl, and the default FEN position is given
/// by the [`Default`] impl, i.e:
///
/// ```
/// use konig::io::Fen;
/// use konig::io::fen::FEN_STARTING_POSITION;
///
/// let from_string = Fen::try_from(FEN_STARTING_POSITION).unwrap();
/// let from_default = Fen::default();
/// assert_eq!(from_string, from_default); // <= succeeds
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Fen {
    pieces: PieceArray,
    side_to_move: StandardColor,
    castling_permissions: StandardCastlingPermissions,
    en_passant_square: Option<StandardIndex>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

/// The initial position of a standard chess game as a FEN string.
pub const FEN_STARTING_POSITION: &'static str =
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Default for Fen {
    fn default() -> Self {
        Fen::try_from(FEN_STARTING_POSITION).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for Fen {
    type Error = VerboseError<&'a str>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Ok(fen_literal(value).finish()?.1)
    }
}

impl Fen {
    /// Consumes `self` and returns a [`Standard`].
    pub fn into_board(
        self,
    ) -> impl std::ops::Index<StandardIndex, Output = Option<StandardPiece>>
           + std::fmt::Debug
           + Standard<
        Color = StandardColor,
        CastlingPermissions = StandardCastlingPermissions,
        Index = StandardIndex,
        Piece = StandardPiece,
    > {
        FenBoard::from(self)
    }

    /// Consumes `self` and constructs a [`StandardBoard`] representing
    /// the same position.
    ///
    /// This operation is potentially expensive, and unless you
    /// specifically need a [`StandardBoard`], you should prefer
    /// [`Fen`]'s `into_board` method.
    pub fn to_standard_board(self) -> StandardBoard {
        self.into()
    }

    /// Returns a [`StandardColor`] corresponding the side whose turn it is to move.
    pub fn side_to_move(&self) -> StandardColor {
        self.side_to_move
    }

    /// Returns the castling permissions described by this FEN string.
    pub fn castling_permissions(&self) -> StandardCastlingPermissions {
        self.castling_permissions
    }

    /// Returns the index of the en passant target square, if it exists.
    pub fn en_passant_square(&self) -> Option<StandardIndex> {
        self.en_passant_square
    }

    /// Returns the value of the halfmove clock as a `u8`, in
    /// which it is always guaranteed to fit.
    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    /// Returns the value of the fullmove counter as a `u16`,
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
    type Index = StandardIndex;
    type Piece = StandardPiece;

    fn get_piece_at(&self, index: Self::Index) -> Option<&Self::Piece> {
        self.data.pieces[usize::from(index)].as_ref()
    }
}

impl Standard for FenBoard {
    type Color = StandardColor;

    type CastlingPermissions = StandardCastlingPermissions;

    fn side_to_move(&self) -> Self::Color {
        self.data.side_to_move
    }

    fn castling_permissions(&self) -> Self::CastlingPermissions {
        self.data.castling_permissions
    }

    fn en_passant_target_square(&self) -> Option<Self::Index> {
        self.data.en_passant_square
    }
}

impl Default for FenBoard {
    fn default() -> Self {
        Self {
            data: Fen::default(),
        }
    }
}

impl std::ops::Index<StandardIndex> for FenBoard {
    type Output = Option<StandardPiece>;

    fn index(&self, index: StandardIndex) -> &Self::Output {
        &self.data.pieces[usize::from(index)]
    }
}

impl From<Fen> for FenBoard {
    fn from(value: Fen) -> Self {
        Self { data: value }
    }
}

/// The return type of the parsers in this module.
type FenResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

/// Parses a single digit character from 1 to 8, i.e. \[12345678\].
fn digit18(source: &str) -> FenResult<char> {
    let mut digit18 = one_of("12345678");
    digit18.parse(source)
}

/// Parses a single piece character of the form \[pnbrqkPNBRQK\].
fn piece(source: &str) -> FenResult<char> {
    let mut piece = one_of("pnbrqkPNBRQK");
    piece.parse(source)
}

/// Parses a single rank field into the component pieces.
fn rank<'a>(source: &'a str) -> FenResult<[Option<StandardPiece>; 8]> {
    let mut pieces = [None; 8];
    let mut index: usize = 0; // write-index into pieces
    let mut rank = verify(
        many_m_n(1, 8, alt((digit18, piece))),
        // this verify call checks that rank will have exactly 8 values
        |chars: &Vec<char>| {
            chars
                .iter()
                .map(|&c| match c {
                    digit @ '1'..='8' => (digit as u8) - 48,
                    _ => 1,
                })
                .reduce(|acc, elem| acc + elem)
                .unwrap()
                == 8
        },
    );

    let (tail, rank) = rank.parse(source)?;
    for character in rank {
        match character {
            space @ '1'..='8' => {
                let length = ((space as u8) - 48) as usize;
                let initial_index = index;
                while index < initial_index + length {
                    pieces[index] = None;
                    index += 1;
                }
            }
            piece @ _ => {
                pieces[index] = match piece {
                    'p' => Some(StandardPiece::BlackPawn),
                    'n' => Some(StandardPiece::BlackKnight),
                    'b' => Some(StandardPiece::BlackBishop),
                    'r' => Some(StandardPiece::BlackRook),
                    'q' => Some(StandardPiece::BlackQueen),
                    'k' => Some(StandardPiece::BlackKing),
                    'P' => Some(StandardPiece::WhitePawn),
                    'N' => Some(StandardPiece::WhiteKnight),
                    'B' => Some(StandardPiece::WhiteBishop),
                    'R' => Some(StandardPiece::WhiteRook),
                    'Q' => Some(StandardPiece::WhiteQueen),
                    'K' => Some(StandardPiece::WhiteKing),
                    _ => unreachable!(),
                };

                index += 1;
            }
        }
    }

    Ok((tail, pieces))
}

/// Parses the entire piece placement field, with ranks flattened.
fn piece_placement(source: &str) -> FenResult<PieceArray> {
    let mut piece_placement = verify(separated_list1(tag("/"), rank), |v: &Vec<_>| v.len() == 8);
    piece_placement.parse(source).map(|(tail, mut files)| {
        (tail, {
            // this will succeed iff we have exactly 8 ranks,
            // which is guaranteed by the verify parser
            // wrapped around the separated_list1.
            //
            // you could also do this in unsafe
            // with unwrap_unchecked, but bounds checks
            // are cheap and segfaults are infuriating.
            files.reverse();
            files.flatten().try_into().unwrap()
        })
    })
}

/// Parses the entire side-to-move field, which is simply \[wb\].
fn side_to_move(source: &str) -> FenResult<StandardColor> {
    let mut side_to_move = one_of("wb");
    side_to_move.parse(source).map(|(tail, side)| {
        (
            tail,
            match side {
                'w' => StandardColor::White,
                'b' => StandardColor::Black,
                _ => unreachable!(),
            },
        )
    })
}

/// Parses the entire castling-ability field.
fn castling_ability(source: &str) -> FenResult<StandardCastlingPermissions> {
    let mut castling_ability = alt((
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

    castling_ability.parse(source).map(|(tail, permissions)| {
        (
            tail,
            match permissions {
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

                _ => unreachable!(),
            },
        )
    })
}

/// Parses the entire en-passant-target-square field.
fn en_passant_target_square(source: &str) -> FenResult<Option<StandardIndex>> {
    // return a dummy success value to make this a pair
    let ep_empty = pair(char('-'), success('-'));
    let ep_square = pair(one_of("abcdefgh"), one_of("36"));
    let mut en_passant_target_square = alt((ep_empty, ep_square));

    en_passant_target_square.parse(source).map(|(tail, pair)| {
        (
            tail,
            match pair {
                ('-', '-') => None,
                (file, rank) => {
                    let rank_offset = match rank {
                        '3' => 16,
                        '6' => 40,
                        _ => unreachable!(),
                    };
                    let file_offset = (file as u8) - 97;

                    // this is entirely safe, it only gets called if the field is parsed correctly
                    unsafe { Some(StandardIndex::new_unchecked(rank_offset + file_offset)) }
                }
            },
        )
    })
}

/// Parses the entire halfmove-clock field
fn halfmove_clock(source: &str) -> FenResult<u8> {
    let mut halfmove_clock = verify(u8, |&clock| clock <= 100);
    halfmove_clock.parse(source)
}

/// Parses the entire fullmove-counter field
fn fullmove_counter(source: &str) -> FenResult<u16> {
    let mut fullmove_counter = u16;
    fullmove_counter.parse(source)
}

/// Parses a complete FEN literal.
fn fen_literal(source: &str) -> FenResult<Fen> {
    let mut fen_literal = (
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
        eof,
    );

    let (
        _tail,
        (
            pieces,
            _,
            side_to_move,
            _,
            castling_permissions,
            _,
            en_passant_square,
            _,
            halfmove_clock,
            _,
            fullmove_counter,
            _,
        ),
    ) = fen_literal.parse(source)?;

    Ok((
        _tail,
        Fen {
            pieces,
            side_to_move,
            castling_permissions,
            en_passant_square,
            halfmove_clock,
            fullmove_counter,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::standard::board::StandardBoard;

    #[test]
    fn check_fen_parser_on_initial_position() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let data = Fen::try_from(start).unwrap();
        let default = StandardBoard::default();

        for i in 0..=63 {
            let index = StandardIndex::try_from(i as u8).unwrap();
            assert_eq!(
                default.get_piece_at(index).map(|x| x.to_owned()),
                data.into_board()
                    .get_piece_at(index.into())
                    .map(|x| x.to_owned().into())
            )
        }

        assert_eq!(data.side_to_move, StandardColor::White);
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
        let data = Fen::try_from(start).unwrap();
        let default = StandardBoard::default();

        // for each position on the board, check that the pieces match
        for i in 0..=63 {
            let index = StandardIndex::try_from(i as u8).unwrap();
            assert_eq!(
                default.get_piece_at(index).map(|x| x.to_owned()),
                data.into_board()
                    .get_piece_at(index.into())
                    .map(|x| x.to_owned().into())
            )
        }

        assert_eq!(data.side_to_move, StandardColor::White);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(data.en_passant_square, None);
        assert_eq!(data.halfmove_clock, 0);
        assert_eq!(data.fullmove_counter, 1);

        // GAME AFTER 1. e4
        let move1 = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let data = Fen::try_from(move1).unwrap();

        assert_eq!(
            data.into_board()
                .get_piece_at(StandardIndex::try_from(28u8).unwrap().into()),
            Some(&StandardPiece::WhitePawn.into())
        );
        assert_eq!(data.side_to_move, StandardColor::Black);
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
        let data = Fen::try_from(move2).unwrap();
        assert_eq!(
            data.into_board()
                .get_piece_at(StandardIndex::try_from(28u8).unwrap().into()),
            Some(&StandardPiece::WhitePawn.into())
        );
        assert_eq!(
            data.into_board()
                .get_piece_at(StandardIndex::try_from(34u8).unwrap().into()),
            Some(&StandardPiece::BlackPawn.into())
        );
        assert_eq!(
            // check black pawn has properly moved
            data.into_board()
                .get_piece_at(StandardIndex::try_from(50u8).unwrap().into()),
            None
        );

        assert_eq!(data.side_to_move, StandardColor::White);
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
        let data = Fen::try_from(move3).unwrap();
        assert_eq!(
            data.into_board()
                .get_piece_at(StandardIndex::try_from(28u8).unwrap().into()),
            Some(&StandardPiece::WhitePawn.into())
        );
        assert_eq!(
            data.into_board()
                .get_piece_at(StandardIndex::try_from(34u8).unwrap().into()),
            Some(&StandardPiece::BlackPawn.into())
        );
        assert_eq!(
            // check black pawn has properly moved
            data.into_board()
                .get_piece_at(StandardIndex::try_from(50u8).unwrap().into()),
            None
        );
        assert_eq!(
            data.into_board()
                .get_piece_at(StandardIndex::try_from(21u8).unwrap().into()),
            Some(&StandardPiece::WhiteKnight.into())
        );

        assert_eq!(data.side_to_move, StandardColor::Black);
        assert_eq!(
            data.castling_permissions,
            StandardCastlingPermissions::default()
        );
        assert_eq!(data.en_passant_square, None);
        assert_eq!(data.halfmove_clock, 1);
        assert_eq!(data.fullmove_counter, 2);
    }

    // taken from https://gist.github.com/peterellisjones/8c46c28141c162d1d8a0f0badbc9cff9
    //
    // on the commandline, do `set FEN_JSON = <link-to-raw-json>`, and then
    // use the command `curl $FEN_JSON | jq ".[] | .fen" | sed "s/\$/,/g"` to
    // get the fen strings correctly formatted.
    #[test]
    fn check_fen_parser_on_misc_positions() {
        let fen_strings = vec![
            "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ - 3 2",
            "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0 3",
            "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 2 2",
            "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQkq - 3 2",
            "2kr3r/p1ppqpb1/bn2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2",
            "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP/RNB1K2R w KQ - 3 9",
            "2r5/3pk3/8/2P5/8/2K5/8/8 w - - 5 4",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0 1",
            "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 0 1",
            "8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1",
            "5k2/8/8/8/8/8/8/4K2R w K - 0 1",
            "3k4/8/8/8/8/8/8/R3K3 w Q - 0 1",
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1",
            "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1",
            "2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1",
            "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1",
            "4k3/1P6/8/8/8/8/K7/8 w - - 0 1",
            "8/P1k5/K7/8/8/8/8/8 w - - 0 1",
            "K1k5/8/P7/8/8/8/8/8 w - - 0 1",
            "8/k1P5/8/1K6/8/8/8/8 w - - 0 1",
            "8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1",
        ];

        for string in fen_strings {
            let fen = Fen::try_from(string).expect(string);
            println!(
                "parsed {}; got this:\nwhite_to_move: {:?}, ep square: {:?}\nhalfmove clock: {}, fullmove counter: {}\ncastling perms: {:?}\nboard:\n8: {:?}\n7: {:?}\n6: {:?}\n5: {:?}\n4: {:?}\n3: {:?}\n2: {:?}\n1: {:?}\n",
                string,
                fen.side_to_move,
                fen.en_passant_square,
                fen.halfmove_clock,
                fen.fullmove_counter,
                fen.castling_permissions,
                &fen.pieces[56..64],
                &fen.pieces[48..56],
                &fen.pieces[40..48],
                &fen.pieces[32..40],
                &fen.pieces[24..32],
                &fen.pieces[16..24],
                &fen.pieces[8..16],
                &fen.pieces[0..8],
            )
        }
    }

    #[test]
    fn check_fen_parser_rejects_bad_positions() {
        let fen_strings = vec![
            "r6r/1b2k1bq/8/8/7B/8/8/R3K2R b KQ 3 2",
            "8/8/8/2k5/2pP4/8/B7/4K3 b - d3 0",
            "r1bqkbnr/pppppppp/n7/8/8/P7/1PPPPPPPRNBQKBNR KQkq - 2 2",
            "r3k2r/p1pp1pb1/bn2Qnp1/2qP1N3/1p2P3/25/PPPBBPPP/R3K2R b KQkq",
            "2kr3rp1ppqb1/n2Qnp1/3PN3/1p2P3/2N5/PPPBBPPP/R3K2R b KQ - 3 2",
            "rnb2k1r/pp1Pbppp/2p5/q7/2B5/8/PPPQNnPP KQ - 3 9",
            "2r5/3pk3/8/2P5/8/2K5/8/8",
            "rnbq1k1r/pp1Pbppp/2p5/8B5/8/PPP1NnPP/RNBQK2R w - 1 8",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/R4RK1 w - 0 10",
            "3k4/3p4/8/K1P4r/8/8/8/8 b - - 0",
            "8/8/4k3/8/2p5/8/B2P2K1/8 w - - 1",
            "8/8/1k6/2b5/2pP4//8 b - 0 1",
            "5k2/8/8/8/8/8/4K2R w K - 0 1",
            "3k4/8/8/8/8/8/8/R w Q - 0 1",
            "r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1dagsa",
            "r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1dgsha123413",
            "2K2r2/4P3/8/8/8/8/8/3k4 w - -ewqyuio",
            "8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1!!!@1241h",
            "4k3/1P6/8/8/8/8/K7/8 w - aaaaaaa",
            "8/P1k5/K7/8/8/8/8/8 w - - 0 1         ",
            "K1k5/8/P7/8/8/8/8/8 w - - 1111 00000000dsaghj",
            "8/k1P5/8/1K6/8/8/8/8",
            "8/8/2k5/5q2/5n2/8/5K8 b - - 0 1",
        ];

        for string in fen_strings {
            Fen::try_from(string).expect_err(string);
        }
    }
}
