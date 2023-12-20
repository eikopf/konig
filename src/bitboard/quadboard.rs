use super::BitBoard;
use std::simd::u64x4;

/// An unopinionated [quadboard](https://www.chessprogramming.org/Quad-Bitboards) 
/// implementation, using Rust's [std::simd] API for accelerated per-nibble operations.
///
/// Actual piece encodings and nicer APIs should typically be handled by a wrapper type,
/// which can check its own invariants before handing off storage concerns to this struct.
/// No safe equivalents to [`get_unchecked`] and [`set_unchecked`] are provided for this
/// reason.
///
/// As a general rule of thumb, this struct should never appear in the public API of a
/// type that uses it; it is almost the prototypical example of an implementation detail.
pub struct QuadBoard(u64x4);

impl QuadBoard {
    /// Converts the quadboard into an array of its underlying channels.
    pub fn into_channels(self) -> [BitBoard; 4] {
        self.0.as_array().map(|ch| BitBoard::from(ch))
    }

    /// Returns a new [`QuadBoard`] with all channels set to 0.
    pub fn empty() -> Self {
        Self(u64x4::default())
    }

    /// Returns the value written to the given index without checking invariants.
    ///
    /// In particular, this function expects that `index` is less than 63.
    pub unsafe fn get_unchecked(&self, index: u8) -> u8 {
        let mask = u64x4::splat(1 << index);
        let mut masked_board = self.0 & mask;
        masked_board >>= index as u64;
        masked_board
            .as_array()
            .into_iter()
            .enumerate()
            .fold(0, |acc, (lane, bit)| acc + ((bit << lane) as u8))
    }

    /// Writes the given value to the given index without checking invariants.
    ///
    /// In particular, this function expects that `value` is less than 16 (i.e.
    /// that it is representable in a nibble) and that `index` is less than 64.
    pub unsafe fn set_unchecked(&mut self, value: u8, index: u8) {
        // extract the individual bits from the given value
        let bit1: u64 = (value & 0b0001).into();
        let bit2: u64 = ((value & 0b0010) >> 1).into();
        let bit3: u64 = ((value & 0b0100) >> 2).into();
        let bit4: u64 = (value >> 3).into();

        // shift the bits to the indexed location
        let channel1 = bit1 << index;
        let channel2 = bit2 << index;
        let channel3 = bit3 << index;
        let channel4 = bit4 << index;

        // create mask vector with all bits set and clear the bits at the indexed location
        let mut mask = u64x4::from_array([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
        let clear_mask = !(1 << index);
        mask &= u64x4::splat(clear_mask);

        // mask off existing value and write new value
        self.0 &= mask;
        self.0 |= u64x4::from_array([channel1, channel2, channel3, channel4]);
    }
}

impl From<[u64; 4]> for QuadBoard {
    fn from(value: [u64; 4]) -> Self {
        Self(value.into())
    }
}

impl From<[BitBoard; 4]> for QuadBoard {
    fn from(value: [BitBoard; 4]) -> Self {
        Self(
            value
                .into_iter()
                .map(|board| u64::from(board))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadboard_new_is_all_zero() {
        // the channels will all be zero iff their product is zero
        let qb = QuadBoard::empty();
        let prod: u64 = qb
            .into_channels()
            .into_iter()
            .map(|board| u64::from(board))
            .product();

        assert!(prod == 0);
    }

    #[test]
    fn quadboard_set_unchecked_is_correct() {
        let mut qb = QuadBoard::empty();

        unsafe {
            qb.set_unchecked(0b1111, 0);
            qb.set_unchecked(0b1101, 5);
            qb.set_unchecked(0b1111, 32);
            qb.set_unchecked(0b0111, 63);
        }

        let lanes = qb.into_channels().map(|board| u64::from(board));
        for (i, lane) in lanes.iter().enumerate() {
            eprintln!("channel {}: 0x{:016x}", i, lane);
        }

        // these values were chosen to match with the particular
        // values set above; changes to either will break the test
        assert_eq!(lanes[0], 0x8000000100000021);
        assert_eq!(lanes[1], 0x8000000100000001);
        assert_eq!(lanes[2], 0x8000000100000021);
        assert_eq!(lanes[3], 0x0000000100000021);
    }

    #[test]
    fn quadboard_get_unchecked_is_correct() {
        let mut qb = QuadBoard::empty();

        unsafe {
            qb.set_unchecked(0b1111, 17);
            qb.set_unchecked(0b1001, 3);
            qb.set_unchecked(0b0100, 38);

            assert_eq!(0b1111, qb.get_unchecked(17));
            assert_eq!(0b1001, qb.get_unchecked(3));
            assert_eq!(0b0100, qb.get_unchecked(38));
        }
    }
}
