use super::piece::Piece;
use super::r#move::Move;
use std::error::Error;

pub trait Board: Default {
    type IllegalMoveError: Error;
    type LegalMove: Move<Board = Self, Piece = Self::Piece>;
    type Move: Move<Board = Self, Piece = Self::Piece>;
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
}
