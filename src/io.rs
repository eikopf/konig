//! Utilities for interacting with common chess formats.

// some modules temporarily hidden while refactoring

/// Provides utilities for the Extended Position Description (EPD) format.
mod epd;

/// Provides utilities for Forsyth-Edwards Notation (FEN).
pub mod fen;

/// Provides utilities for Portable Game Notation (PGN).
mod pgn;

/// Provides utilities for Standard Algebraic Notation (SAN).
pub mod san;

// NOTE: this is a list of standards to look at implementing after the core four
// - FEEN: https://github.com/sashite/specs/blob/main/forsyth-edwards-expanded-notation.md
// - X-FEN: https://en.wikipedia.org/wiki/X-FEN
// - Shredder-FEN: https://www.chessprogramming.org/Forsyth-Edwards_Notation#Shredder-FEN
//      - This probably requires an implementation of Chess960
// - UCI: https://www.chessprogramming.org/UCI
