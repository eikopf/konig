use std::slice::ChunksExact;

use super::{
    piece::Color,
    r#move::{IllegalMoveError, LegalMove, Move},
    Square,
};

use crate::{core, core::Position, io::fen::Fen, standard::piece::StandardPiece};

/// Represents the possible castling permissions described by a FEN string.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingPermissions {
    /// Whether or not castling on the bottom-right is allowed.
    pub white_king_side: bool,
    /// Whether or not castling on the bottom-left is allowed.
    pub white_queen_side: bool,
    /// Whether or not castling on the top-right is allowed.
    pub black_king_side: bool,
    /// Whether or not castling on the top-left is allowed.
    pub black_queen_side: bool,
}

impl CastlingPermissions {
    /// Convienience function for the empty set of castling permissions.
    pub fn none() -> CastlingPermissions {
        CastlingPermissions {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }
}

impl Default for CastlingPermissions {
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
struct BoardState {
    side_to_move: Color,
    castling_rights: CastlingPermissions,
    en_passant_square: Option<Square>,
}

impl Default for BoardState {
    fn default() -> Self {
        Self {
            side_to_move: Color::White,
            castling_rights: CastlingPermissions::default(),
            en_passant_square: None,
        }
    }
}

/// Represents a standard 8x8 chess board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pieces: [Option<StandardPiece>; 64],
    state: BoardState,
}

impl core::Position for Board {
    type Index = Square;
    type Piece = StandardPiece;

    fn get_piece_at(&self, index: Self::Index) -> Option<&Self::Piece> {
        self.pieces[usize::from(index)].as_ref()
    }
}

impl core::Standard for Board {
    type Color = Color;

    type CastlingPermissions = CastlingPermissions;

    fn side_to_move(&self) -> Self::Color {
        self.state.side_to_move
    }

    fn castling_permissions(&self) -> Self::CastlingPermissions {
        self.state.castling_rights
    }

    fn en_passant_target_square(&self) -> Option<Self::Index> {
        self.state.en_passant_square
    }
}

impl core::Validate for Board {
    type LegalMove = LegalMove;
    type Move = Move;
    type ValidationError = IllegalMoveError;

    fn validate(&self, candidate: Self::Move) -> Result<Self::LegalMove, Self::ValidationError> {
        todo!()
    }
}

impl core::Process for Board {
    fn process(&self, candidate: Self::LegalMove) -> Self {
        todo!()
    }
}

impl Default for Board {
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
            state: BoardState::default(),
        }
    }
}

impl std::ops::Index<Square> for Board {
    type Output = Option<StandardPiece>;

    fn index(&self, index: Square) -> &Self::Output {
        &self.pieces[<Square as Into<usize>>::into(index)]
    }
}

impl From<Fen> for Board {
    fn from(value: Fen) -> Self {
        let mut pieces = [None; 64];
        let board = value.into_board();
        for i in 0..=63 {
            let index = unsafe { Square::new_unchecked(i) };
            let piece: Option<StandardPiece> = board.get_piece_at(index.into()).map(|&p| p.into());
            pieces[i as usize] = piece;
        }

        let state = BoardState {
            side_to_move: value.side_to_move(),
            castling_rights: CastlingPermissions {
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

impl<'a> IntoIterator for &'a Board {
    type Item = Option<<Board as Position>::Piece>;
    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIterator {
            board: &self,
            index: 0,
        }
    }
}

impl<'a> Board {
    /// Returns an iterator over the ranks of `self`, from white to black.
    pub fn rank_iter(&'a self) -> impl Iterator<Item = &'a [Option<StandardPiece>]> {
        BoardRankIterator::from(self)
    }
}

struct BoardIterator<'a> {
    board: &'a Board,
    index: usize, // alignment makes u8 and usize take the same space
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = Option<<Board as Position>::Piece>;

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

impl<'a> ExactSizeIterator for BoardIterator<'a> {
    fn len(&self) -> usize {
        64 - self.index
    }
}

struct BoardRankIterator<'a> {
    chunk_iter: ChunksExact<'a, Option<StandardPiece>>,
    index: usize,
}

impl<'a> Iterator for BoardRankIterator<'a> {
    type Item = &'a [Option<StandardPiece>];

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.chunk_iter.next();
        self.index += 1;
        result
    }
}

impl<'a> ExactSizeIterator for BoardRankIterator<'a> {
    fn len(&self) -> usize {
        8 - self.index
    }
}

impl<'a> DoubleEndedIterator for BoardRankIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.chunk_iter.next_back()
    }
}

impl<'a> From<&'a Board> for BoardRankIterator<'a> {
    fn from(value: &'a Board) -> Self {
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
        let board = Board::default();
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
        let board = Board::default();
        let i = Square::try_from(0u8).unwrap();
        let j = Square::try_from(63u8).unwrap();
        let k = Square::try_from(33u8).unwrap();

        assert_eq!(board[i], Some(StandardPiece::WhiteRook));
        assert_eq!(board[j], Some(StandardPiece::BlackRook));
        assert_eq!(board[k], None);
    }
}
