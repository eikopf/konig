use super::{
    piece::Color,
    r#move::{IllegalMoveError, LegalMove, Move},
    Square,
};
use crate::{
    quadboard::QuadBoard,
    core,
    core::Position,
    io::Fen,
    standard::piece::Piece,
};

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

/// Newtype wrapper around an `[Option<Piece>]`
/// to define the relevant encoding in a [`QuadBoard`].
pub(crate) struct BoardPiece(Option<Piece>);

impl From<Option<Piece>> for BoardPiece {
    fn from(value: Option<Piece>) -> Self {
        Self(value)
    }
}

impl From<BoardPiece> for Option<Piece> {
    fn from(value: BoardPiece) -> Self {
        value.0
    }
}

/// Represents a standard 8x8 chess board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    side_to_move: Color,
    pieces: QuadBoard<Option<Piece>>,
    castling_rights: CastlingPermissions,
    en_passant_square: Option<Square>,
}

impl core::Position for Board {
    type Index = Square;
    type Piece = Piece;

    fn get_piece_at(&self, index: Self::Index) -> Option<Self::Piece> {
        todo!()
    }
}

impl core::Standard for Board {
    type Color = Color;

    type CastlingPermissions = CastlingPermissions;

    fn side_to_move(&self) -> Self::Color {
        self.side_to_move
    }

    fn castling_permissions(&self) -> Self::CastlingPermissions {
        self.castling_rights
    }

    fn en_passant_target_square(&self) -> Option<Self::Index> {
        self.en_passant_square
    }
}

impl core::Validate for Board {
    type Move = Move;
    type LegalMove = LegalMove;
    type ValidationError = IllegalMoveError;

    fn validate(&self, candidate: Self::Move) -> Result<Self::LegalMove, Self::ValidationError> {
        todo!()
    }

    fn validate_san(
        &self,
        candidate: crate::io::San,
    ) -> Result<Self::LegalMove, Self::ValidationError>
    where
        Self: core::Standard + Sized,
    {
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
        todo!()
    }
}

impl From<Fen> for Board {
    fn from(value: Fen) -> Self {
        let mut pieces = [None; 64];
        let board = value.into_position();
        for i in 0..=63 {
            let index = unsafe { Square::new_unchecked(i) };
            let piece: Option<Piece> = board.get_piece_at(index.into()).map(|p| p.into());
            pieces[i as usize] = piece;
        }

        let side_to_move = value.side_to_move();
        let en_passant_square = value.en_passant_square().map(Into::into);
        let castling_rights = CastlingPermissions {
            white_king_side: value.castling_permissions().white_king_side,
            white_queen_side: value.castling_permissions().white_queen_side,
            black_king_side: value.castling_permissions().black_king_side,
            black_queen_side: value.castling_permissions().black_queen_side,
        };

        Self {
            side_to_move,
            pieces: pieces.into(),
            castling_rights,
            en_passant_square,
        }
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

impl Board {
    /// Returns an iterator over the ranks of `self`, from white to black.
    pub fn rank_iter(self) -> impl Iterator<Item = [Option<Piece>; 8]> {
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
        todo!()
    }
}

impl<'a> ExactSizeIterator for BoardIterator<'a> {
    fn len(&self) -> usize {
        64 - self.index
    }
}

struct BoardRankIterator {
    board: [Option<Piece>; 64],
    index: usize,
}

impl<'a> Iterator for BoardRankIterator {
    type Item = [Option<Piece>; 8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 64 { return None; };
        let result = &self.board[self.index..(self.index + 8)];
        self.index += 8;
        match result.try_into() {
            Ok(rank) => Some(rank),
            Err(_) => unreachable!(),
        }
    }
}

impl ExactSizeIterator for BoardRankIterator {
    fn len(&self) -> usize {
        8 - (self.index / 8)
    }
}

impl DoubleEndedIterator for BoardRankIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index == 0 { return None; };
        let result = &self.board[(self.index - 8)..self.index];
        self.index -= 8;
        match result.try_into() {
            Ok(rank) => Some(rank),
            Err(_) => unreachable!(),
        }
    }
}

impl From<Board> for BoardRankIterator {
    fn from(value: Board) -> Self {
        //Self {
        //    board: value.pieces.into_array(),
        //    index: 0,
        //}
        todo!()
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
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteRook)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteKnight)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteBishop)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteQueen)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteKing)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteBishop)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteKnight)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhiteRook)));

        // second rank
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::WhitePawn)));

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
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackPawn)));

        // eighth rank
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackRook)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackKnight)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackBishop)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackQueen)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackKing)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackBishop)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackKnight)));
        assert_eq!(board_iter.next(), Some(Some(Piece::BlackRook)));

        // end of iterator
        assert_eq!(board_iter.next(), None);
    }

    #[test]
    fn std_ops_index_into_standard_board_is_correct() {
        let board = Board::default();
        let i = Square::try_from(0u8).unwrap();
        let j = Square::try_from(63u8).unwrap();
        let k = Square::try_from(33u8).unwrap();

        assert_eq!(board.get_piece_at(i), Some(Piece::WhiteRook));
        assert_eq!(board.get_piece_at(j), Some(Piece::BlackRook));
        assert_eq!(board.get_piece_at(k), None);
    }
}
