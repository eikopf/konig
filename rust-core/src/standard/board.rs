use super::r#move::{IllegalStandardMoveError, LegalStandardMove, StandardMove};
use crate::{core::board::Board, standard::piece::StandardPiece};
use std::num::NonZeroU8;

impl Board for StandardBoard {
    type IllegalMoveError = IllegalStandardMoveError;
    type LegalMove = LegalStandardMove;
    type Move = StandardMove;
    type Piece = StandardPiece;

    fn process(&mut self, candidate: Self::LegalMove) -> Self {
        todo!()
    }

    fn validate(&self, candidate: Self::Move) -> Result<Self::LegalMove, Self::IllegalMoveError> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandardBoard {
    // pieces
    pieces: [Option<StandardPiece>; 64],

    // game state
    white_turn: bool,
    castling_rights: [bool; 4], // right-to-left, then white-to-black, tracks castling rights per rook
    en_passant_square: Option<NonZeroU8>,
}

impl Default for StandardBoard {
    fn default() -> Self {
        Self {
            pieces: [
                Some(StandardPiece::WhiteRook),
                Some(StandardPiece::WhiteKnight),
                Some(StandardPiece::WhiteBishop),
                Some(StandardPiece::WhiteQueen),
                Some(StandardPiece::WhiteKing),
                Some(StandardPiece::WhiteBishop),
                Some(StandardPiece::WhiteKnight),
                Some(StandardPiece::WhiteRook),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                Some(StandardPiece::WhitePawn),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackPawn),
                Some(StandardPiece::BlackRook),
                Some(StandardPiece::BlackKnight),
                Some(StandardPiece::BlackBishop),
                Some(StandardPiece::BlackQueen),
                Some(StandardPiece::BlackKing),
                Some(StandardPiece::BlackBishop),
                Some(StandardPiece::BlackKnight),
                Some(StandardPiece::BlackRook),
            ],
            white_turn: true,
            castling_rights: [true, true, true, true],
            en_passant_square: None,
        }
    }
}

/// Provides a simple forward iterator over the pieces on a `StandardBoard`.
pub struct StandardBoardIterator<'a> {
    board: &'a StandardBoard,
    index: usize, // alignment makes u8 and usize take the same space
}

impl<'a> Iterator for StandardBoardIterator<'a> {
    type Item = Option<<StandardBoard as Board>::Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 64 {
            None
        } else {
            let result = Some(self.board.pieces[self.index]);
            self.index += 1;
            result
        }
    }
}

impl<'a> ExactSizeIterator for StandardBoardIterator<'a> {
    fn len(&self) -> usize {
        64 - self.index
    }
}

impl<'a> IntoIterator for &'a StandardBoard {
    type Item = Option<<StandardBoard as Board>::Piece>;
    type IntoIter = StandardBoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            board: &self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_board_iterator_produces_correct_order() {
        let board = StandardBoard::default();
        let mut board_iter = board.into_iter();

        // first rank
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteRook)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteKnight)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteBishop)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteQueen)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteKing)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteBishop)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteKnight)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhiteRook)));

        // second rank
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::WhitePawn)));

        // third rank
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));

        // fourth rank
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));

        // fifth rank
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));

        // sixth rank
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));
        assert_eq!(board_iter.next(), Some(None));

        // seventh rank
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackPawn)));

        // eighth rank
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackRook)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackKnight)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackBishop)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackQueen)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackKing)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackBishop)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackKnight)));
        assert_eq!(board_iter.next(), Some(Some(StandardPiece::BlackRook)));

        // end of iterator
        assert_eq!(board_iter.next(), None);
    }
}
