use std::slice::ChunksExact;

use super::{
    index::StandardIndex,
    r#move::{IllegalStandardMoveError, LegalStandardMove, StandardMove},
};

use crate::{core::board::Board, standard::piece::StandardPiece};

/// Represents the implicit state of a standard
/// 8x8 chess board, i.e. the information that
/// cannot be derived solely from the current
/// state of the pieces on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StandardBoardState {
    // TODO: replace with better types
    white_turn: bool,
    castling_rights: [bool; 4], // clockwise from the bottom-right on a per-rook basis
    en_passant_square: Option<StandardIndex>,
}

impl Default for StandardBoardState {
    fn default() -> Self {
        Self {
            white_turn: true,
            castling_rights: [true, true, true, true],
            en_passant_square: None,
        }
    }
}

/// Represents a standard 8x8 chess board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandardBoard {
    // pieces
    pieces: [Option<StandardPiece>; 64],

    // essential state
    state: StandardBoardState,
}

impl Board for StandardBoard {
    type IllegalMoveError = IllegalStandardMoveError;
    type Index = StandardIndex;
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
            state: StandardBoardState::default(),
        }
    }
}

impl std::ops::Index<<Self as Board>::Index> for StandardBoard {
    type Output = Option<<Self as Board>::Piece>;

    fn index(&self, index: <Self as Board>::Index) -> &Self::Output {
        &self.pieces[<StandardIndex as Into<usize>>::into(index)]
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

impl<'a> StandardBoard {
    /// Returns an iterator over the ranks of `self`, from white to black.
    pub fn rank_iter(&'a self) -> StandardBoardRankIterator<'a> {
        StandardBoardRankIterator::from(self)
    }
}

/// Linear iterator over the pieces on a `StandardBoard`.
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

/// Linear iterator over the ranks on a `StandardBoard`.
pub struct StandardBoardRankIterator<'a> {
    chunk_iter: ChunksExact<'a, Option<StandardPiece>>,
    index: usize,
}

impl<'a> Iterator for StandardBoardRankIterator<'a> {
    type Item = &'a [Option<StandardPiece>];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.chunk_iter.next();
        self.index += 1;
        result
    }
}

impl<'a> ExactSizeIterator for StandardBoardRankIterator<'a> {
    fn len(&self) -> usize {
        8 - self.index
    }
}

impl<'a> DoubleEndedIterator for StandardBoardRankIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.chunk_iter.next_back()
    }
}

impl<'a> From<&'a StandardBoard> for StandardBoardRankIterator<'a> {
    fn from(value: &'a StandardBoard) -> Self {
        Self {
            chunk_iter: value.pieces.chunks_exact(8),
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

    #[test]
    fn std_ops_index_into_standard_board_is_correct() {
        let board = StandardBoard::default();
        let i = StandardIndex::try_from(0).unwrap();
        let j = StandardIndex::try_from(63).unwrap();
        let k = StandardIndex::try_from(33).unwrap();

        assert_eq!(board[i], Some(StandardPiece::WhiteRook));
        assert_eq!(board[j], Some(StandardPiece::BlackRook));
        assert_eq!(board[k], None);
    }
}
