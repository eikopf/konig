// SAN is not a single standard (despite the name), but rather
// a set of vague conventions around recording algebraic moves.
//
// This is unfortunate, but this implementation will seek to
// accept as many variants of this notation as is reasonable,
// and to produce a single standardised variant. For now, this
// is limited to the English variant of SAN.
//
// The canonical version of SAN here will be FIDE's, as described in
// appendix C of their Laws of Chess document: https://handbook.fide.com/chapter/E012023

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::opt,
    sequence::{pair, preceded, tuple, Tuple},
    IResult,
};
use thiserror::Error;

use crate::standard::piece::StandardPieceKind;

/// The error returned when attempting to
/// parse an invalid SAN literal.
#[derive(Error, Debug)]
pub enum ParseError<'a> {
    /// Returned if the optional leading character of the literal is invalid
    #[error("Expected one of 'O', 'K', 'Q', 'B', 'R', 'N'; got {0}")]
    InvalidLeadingPiece(char),

    /// Returned if the mandatory target square field is invalid.
    #[error("Expected a valid target square; got {0}")]
    InvalidTargetSquare(&'a str),

    /// Returned if the optional capture field is invalid.
    #[error("Expected one of [x, X, -, :]; got {0}")]
    InvalidCaptureField(char),

    /// Returned if the optional disambiguation field is invalid.
    #[error("Expected a value fulfilling [a-h]?[1-8]?; got {0}")]
    InvalidDisambiguationField(&'a str),

    /// Returned if the optional annotation suffix field is invalid.
    #[error("Expected a value fulfilling [?!]?[?!]?; got {0}")]
    InvalidAnnotationSuffixField(&'a str),

    /// Returned if the optional en passant suffix is invalid.
    #[error("Expected a value equal to \"e.p\"; got {0}")]
    InvalidEnPassantSuffix(&'a str),

    /// Returned if the optional check field is invalid.
    #[error("Expected a value fulfilling [+]?; got {0}")]
    InvalidCheckField(char),

    /// Returned if the optional checkmate field is invalid.
    #[error("Expected a value fulfilling [#]? or [++]?; got {0}")]
    InvalidCheckmateField(char),

    /// Returned if the optional promotion field is invalid.
    #[error("Expected a value fulfilling [=/]?[NBRQ] or ([NBRQ]); got {0}")]
    InvalidPromotionField(&'a str),

    /// Returned if the optional castling field is invalid.
    #[error("Expected either [0O]-[0O] or [0O]-[0O]-[0O]; got {0}")]
    InvalidCastlingField(&'a str),

    /// Returned if the length of the literal is invalid.
    #[error("Expected a literal with at least 2 and at most 12 characters; got {0} characters")]
    InvalidLiteralLength(u8),

    /// Returned if the terminating score field is invalid.
    #[error("Expected the pattern [-+01½]-[-+01½]; got {0}")]
    InvalidTerminalScore(&'a str),

    /// Returned if an unknown error occurs while parsing a SAN literal.
    #[error("Failed to parse the provided SAN literal")]
    Unknown,
}

/// Represents the data derived from parsing a
/// valid SAN literal.
///
/// Parsing is provided via the `TryFrom<&'a str>` impl.
pub struct San {
    data: SanData,
    is_check: bool,
    is_checkmate: bool,
    annotation: Option<SuffixAnnotation>,
}

/// The distinct kinds of data conveyed by a SAN literal.
///
/// Keep in mind that a SAN literal conveys information about
/// a move which may or may not be valid in the context of a
/// given board position. This struct stores the exact data
/// conveyed by the literal, but needs a [`Board`](crate::core::Board) to be converted
/// into a [`Move`](crate::core::Move), and a [`Validate`](crate::core::Validate) to be converted into a
/// [`LegalMove`](crate::core::LegalMove).
enum SanData {
    AbbreviatedPawnMove(AbbreviatedPawnMove),
    CastleMove(CastleMove),
    NormalMove(NormalMove),
    PawnMove(PawnMove),
}

/// Represents a SAN literal denoting a castling move.
///
/// ## Rough Specification
/// Castle moves are entirely described by their direction,
/// and are normally given as 0-0 (kingside) and 0-0-0 (queenside).
/// It's also common to use O instead of 0, and so this should be
/// accepted as an input format.
///
/// While it is impossible to capture while castling, it is
/// theoretically possible to put the opponent both in check
/// and checkmate; hence when parsing a SAN-castle literal you
/// still must check for the common check/checkmate suffixes. As
/// usual, you also want to look for the annotation suffixes as well.
enum CastleMove {
    QueenSide,
    KingSide,
}

/// Represents a SAN literal denoting a normal (non-pawn) move.
///
/// ## Rough Specification
/// Normal moves are always preceded by a capital letter corresponding
/// to the piece, since the absence of this indicates a pawn move. This
/// is optionally followed by a disambiguating field, and then the optional
/// capture indicator. This is followed by a mandatory target square. Finally,
/// we also include the optional check, checkmate, and annotation suffixes.
struct NormalMove {
    piece: StandardPieceKind,
    disambiguation_field: Option<DisambiguationField>,
    target: (char, char),
    is_capture: bool,
}

/// Represents a SAN literal denoting a normal pawn move.
///
/// ## Rough Specification
/// A pawn move is little more than a normal move with no
/// leading character, and which permits an additional
/// promotion piece component.
struct PawnMove {
    target: (char, char),
    is_capture: bool,
    capture_rank: Option<char>,
    promotion_piece: Option<StandardPieceKind>,
}

/// Represents a SAN literal denoting an abbreviated pawn move.
///
/// ## Rough Specification
/// Abbreviated pawn moves are hugely contextual; they are limited
/// to listing just the source and target files, with a capture
/// glyph in between if necessary.
struct AbbreviatedPawnMove {
    source_rank: char,
    target_rank: char,
    is_capture: bool,
    promotion_piece: Option<StandardPieceKind>,
}

/// Describes the optional field
/// used to disambiguate potentially
/// ambiguous moves from one another.
#[derive(Debug, PartialEq, Eq)]
enum DisambiguationField {
    FileLetter(char),
    RankDigit(char),
    SourceSquare((char, char)),
}

/// Describes the traditional
/// suffix annotation used to
/// describe the qualitative
/// aspects of a move.
///
/// Here the word bang corresponds
/// to the exclamation mark (!) and
/// the word hook corresponds to the
/// question mark (?).
#[derive(Debug, PartialEq, Eq)]
enum SuffixAnnotation {
    Bang,     // good move
    Hook,     // mistake
    BangBang, // brilliant move
    BangHook, // interesting move (ambiguous value)
    HookBang, // dubious move (potentially negative value)
    HookHook, // blunder
}

/// The second return type of [`parse_san_literal`].
///
/// This is the most disgusting thing i've ever put to
/// code, and it'll need to be replaced, but for now i'm
/// concerned with getting it to work.
type ParsedSanLiteral<'a> = (
    Option<&'a str>,
    Option<(char, Option<char>, char, Option<char>)>,
    Option<(
        char,
        Option<(Option<char>, Option<char>)>,
        Option<char>,
        (char, char),
    )>,
    Option<(Option<(char, char)>, (char, char), Option<char>)>,
    Option<&'a str>,
    Option<(Option<char>, Option<char>)>,
);

/// Entrypoint in the SAN parser.
fn parse_san_literal<'a>(source: &'a str) -> IResult<&'a str, ParsedSanLiteral<'a>> {
    // common definitions
    let check = tag("+");
    let checkmate = tag("#");
    let annotation = pair(opt(one_of("!?")), opt(one_of("!?")));
    let promotion_piece = one_of("RNBQ");
    let promotion = preceded(tag("="), promotion_piece);

    // normal move grammar
    let piece = one_of("KQRBN");
    let disambiguation_field = pair(opt(one_of("abcdefgh")), opt(one_of("12345678")));
    let capture = one_of("xX×:");
    let target = pair(one_of("abcdefgh"), one_of("12345678"));
    let normal_move = tuple((piece, opt(disambiguation_field), opt(capture), target));

    // castle move grammar
    let castle = alt((tag("0-0"), tag("O-O"), tag("0-0-0"), tag("O-O-O")));

    // abbreviated pawn move grammar
    let file = one_of("abcdefgh");
    let abbrev_move = tuple((file, opt(capture), file, opt(promotion)));

    // normal pawn move grammar
    let pawn_move = tuple((opt(pair(file, capture)), target, opt(promotion)));

    // full parser
    let mut parser = (
        opt(castle),
        opt(abbrev_move),
        opt(normal_move),
        opt(pawn_move),
        opt(alt((check, checkmate))),
        opt(annotation),
    );

    parser.parse(source)
}

fn process_parsed_san_literal<'a>(source: ParsedSanLiteral<'a>) -> Result<San, ParseError<'a>> {
    // the first four elements of source are mutually exclusive,
    // so this if tree should evaluate to a single Result<SanData, ...>
    let data = if let Some(castle) = source.0 {
        match castle {
            "0-0" | "O-O" => Ok(SanData::CastleMove(CastleMove::KingSide)),
            "0-0-0" | "O-O-O" => Ok(SanData::CastleMove(CastleMove::QueenSide)),
            _ => Err(ParseError::InvalidCastlingField(castle)),
        }
    } else if let Some((source, capture, target, promotion)) = source.1 {
        Ok(SanData::AbbreviatedPawnMove(AbbreviatedPawnMove {
            source_rank: source,
            target_rank: target,
            is_capture: capture.is_some(),
            promotion_piece: match promotion {
                None => None,
                Some('R') => Some(StandardPieceKind::Rook),
                Some('N') => Some(StandardPieceKind::Knight),
                Some('B') => Some(StandardPieceKind::Bishop),
                Some('Q') => Some(StandardPieceKind::Queen),
                _ => unreachable!(), // TODO: replace with error
            },
        }))
    } else if let Some((piece, disambiguation_field, capture, target)) = source.2 {
        Ok(SanData::NormalMove(NormalMove {
            piece: match piece {
                'R' => StandardPieceKind::Rook,
                'N' => StandardPieceKind::Knight,
                'B' => StandardPieceKind::Bishop,
                'Q' => StandardPieceKind::Queen,
                'K' => StandardPieceKind::King,
                _ => unreachable!(), // TODO: replace with error
            },
            disambiguation_field: match disambiguation_field {
                None => None,
                Some((None, None)) => None,
                Some((Some(file), None)) => Some(DisambiguationField::FileLetter(file)),
                Some((None, Some(rank))) => Some(DisambiguationField::RankDigit(rank)),
                Some((Some(file), Some(rank))) => {
                    Some(DisambiguationField::SourceSquare((file, rank)))
                }
            },
            target,
            is_capture: capture.is_some(),
        }))
    } else if let Some((file_capture_pair, target, promotion)) = source.3 {
        Ok(SanData::PawnMove(PawnMove {
            target,
            is_capture: file_capture_pair.is_some(),
            capture_rank: file_capture_pair.map(|(file, _)| file),
            promotion_piece: match promotion {
                None => None,
                Some('R') => Some(StandardPieceKind::Rook),
                Some('N') => Some(StandardPieceKind::Knight),
                Some('B') => Some(StandardPieceKind::Bishop),
                Some('Q') => Some(StandardPieceKind::Queen),
                _ => unreachable!(), // TODO: replace with error
            },
        }))
    } else {
        Err(ParseError::Unknown)
    };

    Ok(San {
        data: data?,
        is_check: source.4.is_some_and(|c| c == "+"),
        is_checkmate: source.4.is_some_and(|c| c == "#"),
        annotation: source.5.map(|pair| match pair {
            (Some('?'), None) => SuffixAnnotation::Hook,
            (Some('!'), None) => SuffixAnnotation::Bang,
            (Some('?'), Some('?')) => SuffixAnnotation::HookHook,
            (Some('!'), Some('!')) => SuffixAnnotation::BangBang,
            (Some('!'), Some('?')) => SuffixAnnotation::BangHook,
            (Some('?'), Some('!')) => SuffixAnnotation::HookBang,
            _ => unreachable!(),
        }),
    })
}
