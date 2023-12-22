//! Utilities for interacting with common chess formats.

// some modules temporarily hidden while refactoring

/// Provides utilities for the Extended Position Description (EPD) format.
mod epd;

/// Provides utilities for Forsyth-Edwards Notation (FEN).
mod fen;

/// Provides utilities for Portable Game Notation (PGN).
mod pgn;

/// Provides utilities for Standard Algebraic Notation (SAN).
mod san;

// NOTE: this is a list of standards to look at implementing after the core four
// - FEEN: https://github.com/sashite/specs/blob/main/forsyth-edwards-expanded-notation.md
// - X-FEN: https://en.wikipedia.org/wiki/X-FEN
// - Shredder-FEN: https://www.chessprogramming.org/Forsyth-Edwards_Notation#Shredder-FEN
//      - This probably requires an implementation of Chess960
// - UCI: https://www.chessprogramming.org/UCI
//      - This really requires a full game implementation with a playing AI first.
// - ICCF numeric notation: https://en.wikipedia.org/wiki/ICCF_numeric_notation

// public reexports
pub use fen::Fen;
pub use fen::FEN_STARTING_POSITION;
pub use san::San;
