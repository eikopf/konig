//! An abstract `Board` trait.

use super::index::Index;
use super::piece::Piece;
use super::r#move::{IllegalMoveError, LegalMove, Move};

/// Represents a chessboard at the highest level, as an
/// object that can modify itself based on a legal move,
/// and which can determine whether a given move is legal.
pub trait Board: Default + std::ops::Index<Self::Index> {
    /// This error is returned if a move cannot be validated.
    type IllegalMoveError: IllegalMoveError;
    /// Represents a specific place on the board.
    type Index: Index<Board = Self>;
    /// Represents a move on the board which is known to be legal.
    type LegalMove: LegalMove<Board = Self>;
    /// Represents an arbitrary move on the board, which may be illegal.
    type Move: Move<Board = Self>;
    /// Represents the pieces which may be on the board.
    type Piece: Piece;

    /// Applies the given LegalMove, and returns the new state of the board.
    fn process(&mut self, candidate: Self::LegalMove) -> Self;

    /// Tries to validate the given candidate `Move` and convert it into a `LegalMove`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the candidate move is illegal given the current state of the board.
    fn validate(&self, candidate: Self::Move) -> Result<Self::LegalMove, Self::IllegalMoveError>;

    /// Tries to first validate and then process the given candidate `Move`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the candidate move is illegal given the current state of the board.
    fn validate_and_process(
        &mut self,
        candidate: Self::Move,
    ) -> Result<Self, Self::IllegalMoveError> {
        let legal_move = self.validate(candidate)?;
        Ok(self.process(legal_move))
    }

    /// A simple constructor yielding the default position.
    fn new() -> Self {
        Self::default()
    }
}
