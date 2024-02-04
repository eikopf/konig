//! Memory-efficient standard board representations.
//!
//! A [quadboard](https://www.chessprogramming.org/Quad-Bitboards) is simply
//! a collection of four [`BitBoard`] values (here called *channels*) used to
//! store information, most commonly about piece positions.
//!
//! Generally, we treat quadboards as though they are given by four horizontal binary
//! channels, each of length 64, and so each vertical "slice" is itself a [`Nibble`].
//!
//!
//! # Typed and Untyped Quadboards
//! Since a quadboard is really just four bitboards, it is effectively an untyped
//! fixed-length buffer; this usage is reflected in the [`RawQuadBoard`] struct,
//! which allows the writing of arbitrary nibbles to arbitrary locations with no
//! concern for their interpretation or validity.
//!
//! But in actual usage, a quadboard is meant to represent a single type, and in
//! that context the manual conversion between a [`Nibble`] and some `T` is just
//! distracting boilerplate. Hence, the [`QuadBoard`] struct wraps a [`RawQuadBoard`]
//! and includes a generic type parameter `T`; the possible interactions with this
//! type are then governed by trait bounds on `T`, and in particular the [`From`],
//! [`Into`], [`TryFrom`], and [`TryInto`] impls where their type parameter is [`Nibble`].
//!
//! # SIMD
//! `TODO`

use crate::bitboard::BitBoard;
pub use halfling::Nibble;
use std::{marker::PhantomData, simd::u64x4};

/// A type whose encoding defines an explicit `EMPTY` value,
/// representing something like an empty space.
pub trait EmptyNibble: Into<Nibble> {
    /// The designated empty value, which **must** be
    /// an element of the codomain of the corresponding
    /// [`NibbleEncode`] implementation on `Self`.
    const EMPTY: halfling::Nibble;
}

/// An unopinionated [quadboard](https://www.chessprogramming.org/Quad-Bitboards)
/// implementation, using Rust's [std::simd] API for accelerated per-nibble operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct QuadBoard<T> {
    inner: RawQuadBoard,
    _data: PhantomData<T>,
}

impl<T> Default for QuadBoard<T> {
    fn default() -> Self {
        Self {
            inner: RawQuadBoard::default(),
            _data: PhantomData,
        }
    }
}

impl<T, E> QuadBoard<T>
where
    T: TryFrom<Nibble, Error = E>,
    E: std::error::Error,
{
    /// Consumes `self` and maps `T::try_from` over
    /// the [`QuadBoard`], returning the result in a fixed
    /// length array.
    pub fn into_array(self) -> [Result<T, E>; 64] {
        todo!()
    }

    /// Reads the [`Nibble`] at the given index and 
    /// attempts a [`TryFrom`] conversion before returning.
    ///
    /// # Panics
    /// Panics if `index >= 64`, i.e. if it is an invalid index
    /// into a [`QuadBoard`].
    pub fn read(&self, index: u8) -> Result<T, E> {
        assert!(index < 64);
        unsafe { self.get_unchecked(index) }
    }

    /// Reads the [`Nibble`] at the given index without bounds checking
    /// and attempts a [`TryFrom`] conversion before returning.
    ///
    /// # Safety
    /// `index` must be less than 64.
    pub unsafe fn get_unchecked(&self, index: u8) -> Result<T, E> {
        let nibble = unsafe { self.inner.get_unchecked(index) };
        T::try_from(nibble)
    }
}

impl<T> QuadBoard<T> {
    /// Returns an empty [`QuadBoard`], where the associated `EMPTY` value
    /// on the [`EmptyNibble`] implementation for `T` has been written to
    /// every index.
    pub fn empty() -> Self
    where
        T: EmptyNibble,
    {
        Self {
            inner: RawQuadBoard::splat(T::EMPTY),
            _data: PhantomData,
        }
    }

    /// Converts `value` into a [`Nibble`] and writes the
    /// resulting `T` value to `index`.
    ///
    /// # Panics
    /// Panics if `index >= 64`, i.e. if the given index is out of
    /// bounds.
    pub fn write(&mut self, value: T, index: u8)
    where
        T: Into<Nibble>,
    {
        todo!()
    }

    /// Converts `value` into a [`Nibble`] and writes the
    /// resulting `T` value to `index` without bounds checking.
    ///
    /// # Safety
    /// `index` must be strictly less than 64.
    pub unsafe fn set_unchecked(&self, value: T, index: u8)
    where
        T: Into<Nibble>,
    {
        todo!()
    }
}

/// An untyped buffer of 64 [`Nibble`] values, stored
/// densely in 4 `u64` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawQuadBoard {
    channels: u64x4,
}

impl Default for RawQuadBoard {
    fn default() -> Self {
        Self {
            channels: Default::default(),
        }
    }
}

impl RawQuadBoard {
    /// Converts the quadboard into an array of its underlying channels.
    fn into_channels(self) -> [BitBoard; 4] {
        self.channels.as_array().map(|ch| BitBoard::from(ch))
    }

    /// Creates a new [`RawQuadBoard`] with each element set to `value`.
    fn splat(value: Nibble) -> Self {
        let mut rqb = RawQuadBoard::default();

        for i in 0..=63u8 {
            unsafe { rqb.set_unchecked(value, i) }
        }

        rqb
    }

    /// Returns the value written to the given index without checking invariants.
    ///
    /// In particular, this function expects that `index` is less than 63.
    pub unsafe fn get_unchecked(&self, index: u8) -> Nibble {
        let mask = u64x4::splat(1 << index);
        let mut masked_board = self.channels & mask;
        masked_board >>= index as u64;
        unsafe {
            Nibble::new_unchecked(
                masked_board
                    .as_array()
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc, (lane, bit)| acc + ((bit << lane) as u8)),
            )
        }
    }

    /// Writes the given value to the given index without checking invariants.
    ///
    /// In particular, this function expects that `value` is less than 16 (i.e.
    /// that it is representable in a nibble) and that `index` is less than 64.
    pub unsafe fn set_unchecked(&mut self, value: Nibble, index: u8) {
        let value: u8 = value.get();

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_quadboard_new_is_all_zero() {
        // the channels will all be zero iff their product is zero
        let qb = RawQuadBoard::default();
        let prod: u64 = qb
            .into_channels()
            .into_iter()
            .map(|board| u64::from(board))
            .product();

        assert!(prod == 0);
    }

    #[test]
    fn raw_quadboard_set_unchecked_is_correct() {
        let mut rqb = RawQuadBoard::default();

        unsafe {
            rqb.set_unchecked(Nibble::try_from(0b1111).unwrap(), 0);
            rqb.set_unchecked(Nibble::try_from(0b1101).unwrap(), 5);
            rqb.set_unchecked(Nibble::try_from(0b1111).unwrap(), 32);
            rqb.set_unchecked(Nibble::try_from(0b0111).unwrap(), 63);
        }

        let lanes = rqb.into_channels().map(|board| u64::from(board));
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
    fn raw_quadboard_get_unchecked_is_correct() {
        let mut rqb = RawQuadBoard::default();

        unsafe {
            rqb.set_unchecked(Nibble::try_from(0b1111).unwrap(), 17);
            rqb.set_unchecked(Nibble::try_from(0b1001).unwrap(), 3);
            rqb.set_unchecked(Nibble::try_from(0b0100).unwrap(), 38);

            assert_eq!(0b1111, rqb.get_unchecked(17).get());
            assert_eq!(0b1001, rqb.get_unchecked(3).get());
            assert_eq!(0b0100, rqb.get_unchecked(38).get());
        }
    }
}
