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

use thiserror::Error;

use crate::standard::{index::StandardIndex, piece::StandardPiece};

/// The error returned when attempting to
/// parse an invalid SAN literal.
#[derive(Error, Debug)]
pub enum SanParseError<'a> {
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
    annotation: Option<SuffixAnnotation>,
}

enum SanData {
    CastleMove(CastleMove),
    NormalMove(NormalMove),
    PawnMove(PawnMove),
}

struct CastleMove {
    side: CastleSide,
    is_check: bool,
    is_checkmate: bool,
}

enum CastleSide {
    Queen,
    King,
}

struct NormalMove {
    piece: StandardPiece,
    disambiguation_field: Option<DisambiguationField>,
    target: StandardIndex,
    is_capture: bool,
    is_check: bool,
    is_checkmate: bool,
}

struct PawnMove {
    disambiguation_field: Option<DisambiguationField>,
    target: StandardIndex,
    is_capture: bool,
    is_check: bool,
    is_checkmate: bool,
    promotion_piece: Option<StandardPiece>,
}

/// Describes the optional field
/// used to disambiguate potentially
/// ambiguous moves from one another
/// in the SAN standard, according
/// to section 8.2.3.4 of the PGN spec.
#[derive(Debug, PartialEq, Eq)]
enum DisambiguationField {
    FileLetter(char),
    RankDigit(u8),
    SourceSquare(StandardIndex),
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
