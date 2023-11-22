use std::slice::ChunksExact;

use super::{
    index::StandardIndex,
    piece::StandardColor,
    r#move::{IllegalStandardMoveError, LegalStandardMove, StandardMove},
};

use crate::{
    core::board::{Board, Process, Validate},
    io::fen::Fen,
    standard::piece::StandardPiece,
};

/// Represents the possible castling permissions described by a FEN string.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct StandardCastlingPermissions {
    /// Whether or not castling on the bottom-right is allowed.
    pub white_king_side: bool,
    /// Whether or not castling on the bottom-left is allowed.
    pub white_queen_side: bool,
    /// Whether or not castling on the top-right is allowed.
    pub black_king_side: bool,
    /// Whether or not castling on the top-left is allowed.
    pub black_queen_side: bool,
}

impl StandardCastlingPermissions {
    /// Convienience function for the empty set of castling permissions.
    pub fn none() -> StandardCastlingPermissions {
        StandardCastlingPermissions {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }
}

impl Default for StandardCastlingPermissions {
    fn default() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

/// Represents the implicit state of a standard
/// 8x8 chess board, i.e. the information that
/// cannot be derived solely from the current
/// state of the pieces on the board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StandardBoardState {
    side_to_move: StandardColor,
    castling_rights: StandardCastlingPermissions,
    en_passant_square: Option<StandardIndex>,
}

impl Default for StandardBoardState {
    fn default() -> Self {
        Self {
            side_to_move: StandardColor::White,
            castling_rights: StandardCastlingPermissions::default(),
            en_passant_square: None,
        }
    }
}

/// Represents a standard 8x8 chess board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandardBoard {
    pieces: [Option<StandardPiece>; 64],
    state: StandardBoardState,
}

impl Board for StandardBoard {
    type Index = StandardIndex;
    type Piece = StandardPiece;

    fn get_piece_at(&self, index: Self::Index) -> Option<&Self::Piece> {
        self.pieces[usize::from(index)].as_ref()
    }
}

impl Validate for StandardBoard {
    type LegalMove = LegalStandardMove;
    type Move = StandardMove;
    type ValidationError = IllegalStandardMoveError;

    fn validate(&self, candidate: Self::Move) -> Result<Self::LegalMove, Self::ValidationError> {
        todo!()
    }
}

impl Process for StandardBoard {
    fn process(&self, candidate: Self::LegalMove) -> Self {
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

impl std::ops::Index<StandardIndex> for StandardBoard {
    type Output = Option<StandardPiece>;

    fn index(&self, index: StandardIndex) -> &Self::Output {
        &self.pieces[<StandardIndex as Into<usize>>::into(index)]
    }
}

impl From<Fen> for StandardBoard {
    fn from(value: Fen) -> Self {
        let mut pieces = [None; 64];
        let board = value.into_board();
        for i in 0..=63 {
            let index = unsafe { StandardIndex::new_unchecked(i) };
            let piece: Option<StandardPiece> = board.get_piece_at(index.into()).map(|&p| p.into());
            pieces[i as usize] = piece;
        }

        let state = StandardBoardState {
            side_to_move: value.side_to_move(),
            castling_rights: StandardCastlingPermissions {
                white_king_side: value.castling_permissions().white_king_side,
                white_queen_side: value.castling_permissions().white_queen_side,
                black_king_side: value.castling_permissions().black_king_side,
                black_queen_side: value.castling_permissions().black_queen_side,
            },
            en_passant_square: value.en_passant_square().map(Into::into),
        };

        Self { pieces, state }
    }
}

impl<'a> IntoIterator for &'a StandardBoard {
    type Item = Option<<StandardBoard as Board>::Piece>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        StandardBoardIterator {
            board: &self,
            index: 0,
        }
    }
}

impl<'a> StandardBoard {
    /// Returns an iterator over the ranks of `self`, from white to black.
    pub fn rank_iter(&'a self) -> impl Iterator<Item = &'a [Option<StandardPiece>]> {
        StandardBoardRankIterator::from(self)
    }
}

impl StandardBoard {
    /// Returns the side whose move is next as a [`StandardColor`].
    pub fn side_to_move(&self) -> StandardColor {
        self.state.side_to_move
    }
}

/// Linear iterator over the pieces on a `StandardBoard`.
struct StandardBoardIterator<'a> {
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
struct StandardBoardRankIterator<'a> {
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
        let i = StandardIndex::try_from(0u8).unwrap();
        let j = StandardIndex::try_from(63u8).unwrap();
        let k = StandardIndex::try_from(33u8).unwrap();

        assert_eq!(board[i], Some(StandardPiece::WhiteRook));
        assert_eq!(board[j], Some(StandardPiece::BlackRook));
        assert_eq!(board[k], None);
    }
}
