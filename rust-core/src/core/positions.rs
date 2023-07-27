use crate::core::pieces::Piece;

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
#[derive(Debug, Eq, PartialEq)]
pub struct Position(u64, u64, u64, u64);

impl Position {
    fn try_get(&self, index: u8) -> Result<Piece, &'static str> {
        let b1 = ((self.0 >> index) & 1) as u8;
        let b2 = ((self.1 >> index) & 1) as u8;
        let b3 = ((self.2 >> index) & 1) as u8;
        let b4 = ((self.3 >> index) & 1) as u8;

        let trunc = (b1 + (b2 << 1) + (b3 << 2) + (b4 << 3)) as u8;
        return Piece::try_from(trunc)
    }

    fn try_write(&mut self, index: u8, piece: Piece) -> Result<(), &'static str> {
        let code: u8 = piece.into();

        // check index validity
        if index >= 64 {
            return Err("Invalid index into a position: it must be in [0, 63]")
        }

        // compute per-channel bits
        let b1 = ((code >> 0) & 1) as u64;
        let b2 = ((code >> 1) & 1) as u64;
        let b3 = ((code >> 2) & 1) as u64;
        let b4 = ((code >> 3) & 1) as u64;

        // write bits to channels
        let index_mask = !(1 << index); // 1 everywhere except for the index bit
        self.0 |= index_mask;
        self.0 &= b1 << index;
        self.1 |= index_mask;
        self.1 &= b2 << index;
        self.2 |= index_mask;
        self.2 &= b3 << index;
        self.3 |= index_mask;
        self.3 &= b4 << index;


        return Ok(())
    }
}
