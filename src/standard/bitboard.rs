use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

/// A `BitBoard` wraps a `u64` to provide
/// a nice API.
pub struct BitBoard(u64);

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        let board = self.0 & rhs.0;
        BitBoard(board)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        let board = self.0 | rhs.0;
        BitBoard(board)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let board = self.0 ^ rhs.0;
        BitBoard(board)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl FromIterator<bool> for BitBoard {
    fn from_iter<T: IntoIterator<Item = bool>>(iter: T) -> Self {
        let mut board = 0;
        let mut iter = iter.into_iter();
        for i in 0..=63 {
            board |= (iter.next().unwrap() as u64) << i;
        }

        // iter should have exactly 64 elements
        assert!(iter.next() == None);

        BitBoard(board)
    }
}

impl<'a> IntoIterator for &'a BitBoard {
    type Item = bool;

    type IntoIter = impl Iterator<Item = bool>;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator {
            board: self,
            mask: 1u64,
        }
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

/// An [`Iterator`] over the bits of a [`BitBoard`].
///
/// Using a mask instead of an index slightly reduces
/// the number of instructions per iteration. In particular,
/// the mask itself only ever has one active bit, whose position
/// corresponds to the value that would be stored by an index.
struct BitBoardIterator<'a> {
    board: &'a BitBoard,
    mask: u64,
}

impl<'a> Iterator for BitBoardIterator<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        // 0 is the edge case to catch the end of the iterator
        if self.mask == 0 {
            return None;
        }

        let result = (self.board.0 | self.mask) == self.board.0;

        // if this was the last bit, set mask to end flag
        if self.mask == 1 << 63 {
            self.mask = 0;
        // otherwise continue
        } else {
            self.mask <<= 1;
        }

        Some(result)
    }
}
