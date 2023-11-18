//! Traits for representing chessboards.

use super::index::Index;
use super::piece::Piece;
use super::r#move::{IllegalMoveError, LegalMove, Move};

/// Represents a static view into a single board position, with
/// no notion of moves or move legality.
///
/// For a notion of legality see [`Validate`].
/// For a notion of moves acting on state, see [`Process`].
pub trait Board: std::fmt::Debug {
    /// Represents a specific place on the board.
    type Index: Index;

    /// Represents the pieces which may be on the board.
    type Piece: Piece;

    /// Returns the piece at the given index by reference
    /// if it exists, otherwise returns none.
    fn get_piece_at(&self, index: Self::Index) -> Option<&Self::Piece>;
}

/// Represents a board which can validate candidate moves.
///
/// Implementations should try to prevent their users from
/// manually constructing any [`LegalMove`]s outside of the
/// context of the [`Validate`] API itself. This can be done
/// manually, but is most effectively implemented with an
/// opaque type.
pub trait Validate: Board {
    /// Represents a move which may or may not be legal.
    type Move: Move<Board = Self, Index = Self::Index>;
    /// Represents a move which has been confirmed to be legal.
    type LegalMove: LegalMove<Board = Self, Index = Self::Index>;
    /// The error created when move validation fails.
    type ValidationError: IllegalMoveError<
        Board = Self,
        Index = Self::Index,
        Move = Self::Move,
        LegalMove = Self::LegalMove,
    >;
    /// Validates the given candidate move based on the current state of self.
    fn validate(&self, candidate: Self::Move) -> Result<Self::LegalMove, Self::ValidationError>;
}

/// Represents a board which can process validated moves.
///
/// The details of move validation are encoded into the
/// corresponding [`Validate`] impl, and [`Process`] is
/// explicitly only concerned with processing [`LegalMove`]s;
/// therefore it is the responsibility of the implementer to
/// ensure that the only source of [`LegalMove`]s in scope is
/// the [`Validate::validate`] implementation.
pub trait Process: Validate {
    /// Updates the board state with the given [`LegalMove`] and returns the new state.
    ///
    /// Note that the only valid source for the candidate move is from [`Validate`]'s
    /// `validate` method, and in general you should prefer `validate_and_process` for
    /// updating the board's state with a single [`Move`].
    fn process(&self, candidate: Self::LegalMove) -> Self
    where
        Self: Sized;

    /// First validates the given candidate move, and then either returns an [`IllegalMoveError`]
    /// or uses the resulting [`LegalMove`] to update the board state and returns it.
    fn validate_and_process(&self, candidate: Self::Move) -> Result<Self, Self::ValidationError>
    where
        Self: Sized,
    {
        let legal_move = self.validate(candidate)?;
        Ok(self.process(legal_move))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::standard::{
        board::StandardBoard,
        index::StandardIndex,
        piece::StandardPiece,
        r#move::{IllegalStandardMoveError, LegalStandardMove, StandardMove},
    };

    // The following tests just won't compile if their types are object-safe.
    // I feel like there should really be a better way to test for object-safety,
    // especially since the compiler provides that info in the documentation.
    //
    // As it turns out, this information is available in the `rustc_trait_selection`
    // crate, as a function called `check_is_object_safe`. I obviously can't access the
    // AST of my code at will, though, that that strategy is a no go.

    #[test]
    fn board_is_object_safe() {
        let _board: Box<dyn Board<Index = StandardIndex, Piece = StandardPiece>> =
            Box::new(StandardBoard::default());
    }

    #[test]
    fn validate_is_object_safe() {
        let _validate: Box<
            dyn Validate<
                Index = StandardIndex,
                Piece = StandardPiece,
                Move = StandardMove,
                LegalMove = LegalStandardMove,
                ValidationError = IllegalStandardMoveError,
            >,
        > = Box::new(StandardBoard::default());
    }

    #[test]
    fn process_is_object_safe() {
        let _process: Box<
            dyn Process<
                Index = StandardIndex,
                Piece = StandardPiece,
                Move = StandardMove,
                LegalMove = LegalStandardMove,
                ValidationError = IllegalStandardMoveError,
            >,
        > = Box::new(StandardBoard::default());
    }
}
