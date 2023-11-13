// use crate::core::piece::PieceType;
// use thiserror::Error;

// #[derive(Error, Debug)]
// enum SanParseError<'a> {
//     #[error("Expected one of 'O', 'K', 'Q', 'B', 'R', 'N'; got {0}")]
//     InvalidLeadingChar(char),

//     #[error("Expected a target in the range [0, 63]; got {0:?}")]
//     InvalidTargetSquare(Option<u8>),

//     #[error("Expected a value fulfilling [xX-]; got {0}")]
//     InvalidCaptureChar(char),

//     #[error("Expected a value fulfilling [a-h]?[1-8]?; got {0}")]
//     InvalidDisambiguationField(&'a str),

//     #[error("Expected a value fulfilling [?!]?[?!]?; got {0}")]
//     InvalidSuffixField(&'a str),

//     #[error("Expected a value fulfilling [+]?; got {0}")]
//     InvalidCheckChar(char),

//     #[error("Expected a value fulfilling [#]?; got {0}")]
//     InvalidCheckmateChar(char),

//     #[error("Expected a value fulfilling =[NBRQ]; got {0}")]
//     InvalidPromotionStr(&'a str),

//     #[error("Expected either \"O-O\" or \"O-O-O\"; got {0}")]
//     InvalidCastleMoveStr(&'a str),

//     #[error("Parsed {0} at an invalid location")]
//     InvalidChar(char),

//     #[error("Expected a literal with at least 2 and at most 12 characters; got {0}")]
//     InvalidLiteralLength(u8),

//     #[error("Failed to parse the provided SAN literal")]
//     Unknown,
// }

// /// Describes the optional field
// /// used to disambiguate potentially
// /// ambiguous moves from one another
// /// in the SAN standard, according
// /// to section 8.2.3.4 of the PGN spec.
// #[derive(Debug, PartialEq, Eq)]
// enum DisambiguationField {
//     FileLetter(u8),
//     RankDigit(u8),
//     SourceSquare(u8),
// }

// /// Describes the traditional
// /// suffix annotation used to
// /// describe the qualitative
// /// aspects of a move.
// ///
// /// Here the word bang corresponds
// /// to the exclamation mark (!) and
// /// the word hook corresponds to the
// /// question mark (?).
// #[derive(Debug, PartialEq, Eq)]
// enum SuffixAnnotation {
//     Bang,
//     Hook,
//     BangBang,
//     BangHook,
//     HookBang,
//     HookHook,
// }

// /// A struct representing the data
// /// communicated by a standard SAN
// /// move.
// #[derive(Debug, PartialEq, Eq)]
// pub struct StandardMoveData {
//     target: u8,
//     piece_type: Option<PieceType>,
//     promotion_piece_type: Option<PieceType>,
//     disambiguation_field: Option<DisambiguationField>,
//     is_capture: bool,
//     is_check: bool,
//     is_checkmate: bool,
//     is_promotion: bool,
//     suffix: Option<SuffixAnnotation>,
// }

// /// Denotes which side of the board
// /// a castle is directed towards.
// #[derive(Debug, PartialEq, Eq)]
// enum CastleSide {
//     KingSide,
//     QueenSide,
// }

// /// Represents the value parsed from
// /// the very first character in a SAN
// /// literal, which will denote (or imply)
// /// the piece being moved.
// enum LeadingCharValue {
//     Piece(PieceType),
//     Castle,
// }

// /// A struct representing the data
// /// communicated by a SAN move which
// /// describes castling.
// #[derive(Debug, PartialEq, Eq)]
// pub struct CastleMoveData {
//     side: CastleSide,
//     is_check: bool,
//     is_checkmate: bool,
// }

// /// Describes the data parsed from a
// /// SAN literal, split between normal
// /// moves and the two castling variants.
// #[derive(Debug, PartialEq, Eq)]
// pub enum SanMove {
//     Normal(StandardMoveData),
//     Castle(CastleMoveData),
// }

// struct LiteralParser {
//     source: String,
//     index: u8,
// }

// impl From<String> for LiteralParser {
//     fn from(value: String) -> Self {
//         LiteralParser {
//             source: value,
//             index: 0,
//         }
//     }
// }

// impl LiteralParser {
//     fn try_parse_leading_char(&mut self) -> Result<LeadingCharValue, SanParseError> {
//         let value = match self.source.chars().nth(self.index.into()) {
//             Some('a'..='h') => return Ok(LeadingCharValue::Piece(PieceType::Pawn)), // early return, no increment
//             Some('R') => Ok(LeadingCharValue::Piece(PieceType::Rook)),
//             Some('N') => Ok(LeadingCharValue::Piece(PieceType::Knight)),
//             Some('B') => Ok(LeadingCharValue::Piece(PieceType::Bishop)),
//             Some('Q') => Ok(LeadingCharValue::Piece(PieceType::Queen)),
//             Some('K') => Ok(LeadingCharValue::Piece(PieceType::King)),
//             Some('O') => Ok(LeadingCharValue::Castle),
//             other @ _ => return Err(SanParseError::InvalidLeadingChar(other.unwrap_or('\0'))),
//         };

//         self.index += 1;
//         return value
//     }

//     fn try_parse_target_square(&mut self) -> Result<u8, SanParseError> {
//         let mut source_iter = self.source.chars();
//         let rank = match source_iter.nth(self.index.into()) {
//             Some(rank @ 'a'..='h') => rank,
//             _ => return Err(SanParseError::InvalidTargetSquare(None)),
//         };

//         let file = match source_iter.next() {
//             Some(file @ '1'..='8') => file,
//             _ => return Err(SanParseError::InvalidTargetSquare(None)),
//         };

//         self.index += 2;
//         return Ok((rank as u8) * 8 + (file as u8))
//     }

//     fn try_parse_capture_field(&mut self) -> Result<bool, SanParseError> {
//         todo!();
//     }

//     // TODO: complete remainder of parser
// }
