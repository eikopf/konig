use packed_struct::prelude::*;
use crate::core::pieces::Piece;
use super::pieces::PieceRepresentationError;

/// Represents a single position in a
/// chess game (aka a board state) as
/// a four-channel tuple composed
/// of `u64`s, which can be thought
/// of like a `[Piece; 64]` or `[u4; 64]`.
///
/// A `u4` in this context can be
/// dealt with as the lower 4 bits
/// of a `u8`. The implementation of
/// `Piece::Into<u8>` describes the
/// canonical mapping for this purpose.
///
/// The channels are ordered such that
/// `Position.n` corresponds to the `n+1`th
/// bit in a `u4`.
///
/// This implementation is significantly
/// more memory-efficient than the naive
/// `[Piece; 64]` implementation, using
/// only 25% the memory to represent
/// an equivalent position.
#[derive(Debug, Eq, PartialEq, PackedStruct)]
#[packed_struct(bit_numbering="msb0", endian="msb")]
pub struct Position {
        ch1: u64,
        ch2: u64,
        ch3: u64,
        ch4: u64
}

pub struct FenOrderedPositionIterator<'a> {
        source: &'a Position,
        index: u8,
        rank_index: u8,
}

impl Position {

    /// Attempts to retrieve the `Piece` at the
    /// given index.
    ///
    /// This operation is a constant time lookup.
    pub fn try_get(&self, index: u8) -> Result<Piece, PieceRepresentationError> {
        let b1 = ((self.ch1 >> index) & 1) as u8;
        let b2 = ((self.ch2 >> index) & 1) as u8;
        let b3 = ((self.ch3 >> index) & 1) as u8;
        let b4 = ((self.ch4 >> index) & 1) as u8;

        let trunc = (b1 + (b2 << 1) + (b3 << 2) + (b4 << 3)) as u8;
        return Piece::try_from(trunc)
    }

    /// Attempts to write the given `Piece` to the
    /// given index.
    ///
    /// This operation will only error for invalid
    /// indices, i.e. indices greater than 63.
    pub fn try_write(&mut self, index: u8, piece: Piece) -> Result<(), ()> {
        let code: u8 = piece.into();

        // check index validity
        if index >= 64 {
            return Err(())
        }

        // compute per-channel bits
        let b1 = ((code >> 0) & 1) as u64;
        let b2 = ((code >> 1) & 1) as u64;
        let b3 = ((code >> 2) & 1) as u64;
        let b4 = ((code >> 3) & 1) as u64;

        // write bits to channels
        let index_mask = !(1 << index); // 1 everywhere except for the index bit
        self.ch1 &= index_mask;
        self.ch1 |= b1 << index;
        self.ch2 &= index_mask;
        self.ch2 |= b2 << index;
        self.ch3 &= index_mask;
        self.ch3 |= b3 << index;
        self.ch4 &= index_mask;
        self.ch4 |= b4 << index;

        return Ok(())
    }

    pub fn empty() -> Position {
        Position {
            ch1: 0,
            ch2: 0,
            ch3: 0,
            ch4: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{pieces::{Piece, PieceColor, PieceType}, positions::Position};

        #[test]
        fn validate_position_try_get_and_try_write() {
                let mut pos = Position { ch1: 0, ch2: 0, ch3: 0, ch4: 0 };
                pos.try_write(4, Piece {
                        color: PieceColor::White,
                        kind: PieceType::Knight,
                }).unwrap();

                let piece = pos.try_get(4).unwrap();
                assert_eq!(piece, Piece{
                        color: PieceColor::White,
                        kind: PieceType::Knight,
                });
        }
}
