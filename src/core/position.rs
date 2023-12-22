//! Traits for representing chessboards.

use crate::io::San;

use super::index::Index;
use super::piece::Piece;
use super::r#move::{IllegalMoveError, LegalMove, Move};
use super::Algebraic;

/// Represents a static view into a single board position, with
/// no notion of moves or move legality.
///
/// For a notion of legality see [`Validate`].
/// For a notion of moves acting on state, see [`Process`].
pub trait Position: std::fmt::Debug {
    /// Represents a specific place on the board.
    type Index: Index;

    /// Represents the pieces which may be on the board.
    type Piece: Piece;

    /// Returns the piece at the given index by reference
    /// if it exists, otherwise returns none.
    fn get_piece_at(&self, index: Self::Index) -> Option<Self::Piece>;
}

/// Represents a board that implements standard chess.
///
/// This is primarily used as a trait bound in [`Validate`]
/// and [`Process`] to add extra methods related to standard
/// chessboard representations.
///
/// This is also a marker trait, in the sense that it constitutes
/// a promise that this [`Position`] is part of an implementation
/// of standard chess.
pub trait Standard
where
    Self: Position,
    Self::Index: Algebraic,
    Self::Piece: Piece<Color = <Self as Standard>::Color>,
{
    /// The type representing the two sides of the game.
    type Color;

    /// The type representing the availability of castling
    /// for each of the four rooks.
    type CastlingPermissions;

    /// Returns the color corresponding to the side next to move.
    fn side_to_move(&self) -> Self::Color;

    /// Returns a struct describing whether each of
    /// the four rooks is still castleable.
    fn castling_permissions(&self) -> Self::CastlingPermissions;

    /// Returns `None` if there is no available en passant square
    /// for capturing, and an [`Index`] if one is available.
    fn en_passant_target_square(&self) -> Option<Self::Index>;
}

/// Represents a board which can validate candidate moves.
///
/// Implementations should try to prevent their users from
/// manually constructing any [`LegalMove`]s outside of the
/// context of the [`Validate`] API itself. This can be done
/// manually, but is most effectively implemented with an
/// opaque type.
pub trait Validate: Position {
    /// Represents a move which may or may not be legal.
    type Move: Move<Index = Self::Index>;
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

    /// Validates the given candidate SAN move based on the current state of self.
    fn validate_san(&self, candidate: San) -> Result<Self::LegalMove, Self::ValidationError>
    where
        Self: Standard + Sized,
        Self::Index: Algebraic,
        Self::Piece: Piece<Color = <Self as Standard>::Color>;
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
        Piece,
        {IllegalMoveError, LegalMove, Move},
        Board, Square,
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
        let _board: Box<dyn super::Position<Index = Square, Piece = Piece>> =
            Box::new(Board::default());
    }

    #[test]
    fn validate_is_object_safe() {
        let _validate: Box<
            dyn Validate<
                Index = Square,
                Piece = Piece,
                Move = Move,
                LegalMove = LegalMove,
                ValidationError = IllegalMoveError,
            >,
        > = Box::new(Board::default());
    }

    #[test]
    fn process_is_object_safe() {
        let _process: Box<
            dyn Process<
                Index = Square,
                Piece = Piece,
                Move = Move,
                LegalMove = LegalMove,
                ValidationError = IllegalMoveError,
            >,
        > = Box::new(Board::default());
    }
}
