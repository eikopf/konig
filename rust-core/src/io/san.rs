use crate::core::pieces::PieceType;

/// Describes the optional field
/// used to disambiguate potentially
/// ambiguous moves from one another
/// in the SAN standard, according
/// to section 8.2.3.4 of the PGN spec.
#[derive(Debug, PartialEq, Eq)]
enum SanDisambiguationField {
    FileLetter(u8),
    RankDigit(u8),
    SourceSquare(u8),
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
enum SanSuffixAnnotation {
    Bang,
    Hook,
    BangBang,
    BangHook,
    HookBang,
    HookHook,
}

/// A struct representing the data
/// communicated by a standard SAN
/// move.
#[derive(Debug, PartialEq, Eq)]
struct SanStandardMoveData {
    target: u8,
    piece_type: Option<PieceType>,
    promotion_piece_type: Option<PieceType>,
    disambiguation_field: Option<SanDisambiguationField>,
    is_capture: bool,
    is_check: bool,
    is_checkmate: bool,
    is_promotion: bool,
    suffix: Option<SanSuffixAnnotation>,
}

/// A struct representing the data
/// communicated by a SAN move which
/// describes castling.
#[derive(Debug, PartialEq, Eq)]
struct SanCastleMoveData {
    is_check: bool,
    is_checkmate: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SanMove {
    Normal(SanStandardMoveData),
    KingSideCastle(SanCastleMoveData),
    QueenSideCastle(SanCastleMoveData),
}
