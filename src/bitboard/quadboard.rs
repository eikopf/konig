use super::BitBoard;
use std::{marker::PhantomData, simd::u64x4};
use thiserror::Error;

/// An unopinionated [quadboard](https://www.chessprogramming.org/Quad-Bitboards)
/// implementation, using Rust's [std::simd] API for accelerated per-nibble operations.
///
/// The details of piece encodings are delegated to the [`NibbleEncode`] and [`NibbleDecode`]
/// traits, allowing downstream consumers of this struct to define custom encodings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuadBoard<T> {
    channels: u64x4,
    _data: PhantomData<T>,
}

impl<T> QuadBoard<T> {
    /// Converts the quadboard into an array of its underlying channels.
    pub fn into_channels(self) -> [BitBoard; 4] {
        self.channels.as_array().map(|ch| BitBoard::from(ch))
    }

    pub fn into_array(self) -> [T; 64] where T: NibbleDecode {
        self.into()
    }

    /// Returns a new [`QuadBoard`] with all channels set to 0.
    pub fn empty() -> Self {
        Self {
            channels: u64x4::default(),
            _data: PhantomData,
        }
    }

    /// Returns the value written to the given index without checking invariants.
    ///
    /// In particular, this function expects that `index` is less than 63.
    pub unsafe fn get_unchecked(&self, index: u8) -> u8 {
        let mask = u64x4::splat(1 << index);
        let mut masked_board = self.channels & mask;
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
        self.channels &= mask;
        self.channels |= u64x4::from_array([channel1, channel2, channel3, channel4]);
    }

    /// Writes the given value to the given index, using the encoding defined
    /// by the implementation of [`NibbleEncode`] on `T`.
    pub fn write(&mut self, value: T, index: u8)
    where
        T: NibbleEncode,
    {
        let nibble = T::encode(value);
        assert!(nibble < 16);
        assert!(index < 64);

        unsafe {
            self.set_unchecked(nibble, index);
        };
    }

    /// Reads a value from the given index and tries to decode it using the
    /// encoding defined by the implementation of [`NibbleDecode`] on `T`.
    pub fn read(&self, index: u8) -> Result<T, NibbleDecodingError>
    where
        T: NibbleDecode,
    {
        assert!(index < 64);
        let nibble = unsafe { self.get_unchecked(index) };
        T::decode(nibble)
    }
}

impl<T> From<[u64; 4]> for QuadBoard<T> {
    fn from(value: [u64; 4]) -> Self {
        Self {
            channels: value.into(),
            _data: PhantomData,
        }
    }
}

impl<T> From<[BitBoard; 4]> for QuadBoard<T> {
    fn from(value: [BitBoard; 4]) -> Self {
        Self {
            channels: value
                .into_iter()
                .map(|board| u64::from(board))
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap(),
            _data: PhantomData,
        }
    }
}

impl<T: NibbleDecode> From<QuadBoard<T>> for [T; 64] {
    fn from(value: QuadBoard<T>) -> Self {
        match value.into_iter().collect::<Vec<T>>().try_into() {
            Ok(array) => array,
            Err(_) => unreachable!(),
        }
    }
}

impl<T: NibbleDecode> IntoIterator for QuadBoard<T> {
    type Item = T;

    type IntoIter = impl Iterator<Item = T>;

    fn into_iter(self) -> Self::IntoIter {
        QuadBoardIterator {
            quadboard: self,
            index: 0,
        }
    }
}

struct QuadBoardIterator<T: NibbleDecode> {
    quadboard: QuadBoard<T>,
    index: u8,
}

impl<T: NibbleDecode> Iterator for QuadBoardIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 64 {
            None
        } else {
            let nibble = unsafe { self.quadboard.get_unchecked(self.index) };
            let value = T::decode(nibble).expect("nibble has a defined encoding");
            self.index += 1;
            Some(value)
        }
    }
}

/// The error returned when attempting to decode a nibble.
#[derive(Debug, Error)]
pub enum NibbleDecodingError {
    /// Returned when trying to decode a nibble with no defined encoding.
    #[error("The nibble {0:04b} is undefined in this encoding.")]
    Undefined(u8),
    /// Returned when trying to decode a non-nibble integer.
    #[error("The integer {0:02x} is too large to be a nibble.")]
    TooLarge(u8),
}

/// A type which can be decoded from a nibble.
pub trait NibbleDecode: Sized {
    /// Decodes a nibble into this type, or
    /// returns the associated error if that fails.
    fn decode(value: u8) -> Result<Self, NibbleDecodingError>;
}

/// A type which can be encoded into a nibble.
pub trait NibbleEncode {
    /// Encodes the given value into a nibble.
    fn encode(self) -> u8;
}

impl NibbleDecode for u8 {
    fn decode(value: u8) -> Result<Self, NibbleDecodingError> {
        if value > 16 {
            Err(NibbleDecodingError::TooLarge(value))
        } else {
            Ok(value)
        }
    }
}

impl NibbleEncode for u8 {
    fn encode(self) -> u8 {
        if self >= 16 {
            panic!("The given value cannot be stored in a nibble.")
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadboard_new_is_all_zero() {
        // the channels will all be zero iff their product is zero
        let qb = QuadBoard::<u8>::empty();
        let prod: u64 = qb
            .into_channels()
            .into_iter()
            .map(|board| u64::from(board))
            .product();

        assert!(prod == 0);
    }

    #[test]
    fn quadboard_set_unchecked_is_correct() {
        let mut qb = QuadBoard::<u8>::empty();

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
        let mut qb = QuadBoard::<u8>::empty();

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
