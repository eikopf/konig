use crate::core;
use crate::core::index::IndexError;
use nom::{character::complete::one_of, combinator::eof, sequence::Tuple, Finish};
use nonmax::NonMaxU8;

/// Represents a specific square on a `StandardBoard`
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Square(NonMaxU8);

impl core::index::Index for Square {
    type MetricTarget = u8;

    fn distance(a: Self, b: Self) -> Self::MetricTarget {
        todo!()
    }
}

impl TryFrom<u8> for Square {
    type Error = IndexError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            index @ 0..=63 => Ok(Self(index.try_into().unwrap())),
            index @ _ => Err(IndexError::OutOfBounds(index)),
        }
    }
}

impl TryFrom<usize> for Square {
    type Error = IndexError<usize>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let int: u8 = value
            .try_into()
            .map_err(|_err| IndexError::OutOfBounds(value))?;

        match Square::try_from(int) {
            Ok(index) => Ok(index),
            Err(_) => Err(IndexError::OutOfBounds(int as usize)),
        }
    }
}

impl From<Square> for usize {
    fn from(value: Square) -> Self {
        u8::from(value.0) as usize
    }
}

impl<'a> TryFrom<&'a str> for Square {
    type Error = IndexError<&'a str>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser = (
            one_of("abcdefgh"), // file
            one_of("12345678"), // rank
            eof,
        );

        parser
            .parse(value)
            .finish()
            .map(|(_, (file, rank, _))| {
                let rank_offset = ((rank as u8) - 49) * 8;
                let file_offset = (file as u8) - 97;
                unsafe { Square::new_unchecked(rank_offset + file_offset) }
            })
            .map_err(|_: nom::error::Error<&'a str>| IndexError::InvalidFormat(value))
    }
}

impl Into<String> for Square {
    fn into(self) -> String {
        let rank = ((self.0.get() / 8) + 49) as char;
        let file = ((self.0.get() % 8) + 97) as char;

        [file, rank].iter().collect()
    }
}

impl Square {
    /// Attempts to construct a valid [`StandardIndex`]
    /// using the given value, and panics if that fails.
    ///
    /// Consider using `try_from(value: usize)` instead for
    /// safer code.
    ///
    /// This should be treated as a utility function,
    /// to avoid constantly writing `StandardIndex::try_from(val).unwrap()`.
    pub(crate) fn new(value: u8) -> Self {
        assert!(value <= 63);
        unsafe { Self(NonMaxU8::new_unchecked(value)) }
    }

    /// Constructs a [`StandardIndex`] without performing
    /// safety checks. The caller must ensure that the
    /// value is less than 64.
    pub(crate) unsafe fn new_unchecked(value: u8) -> Self {
        Self(NonMaxU8::new_unchecked(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_index_try_from_u8_is_correct() {
        let i = Square::try_from(0u8);
        let j = Square::try_from(63u8);
        let k = Square::try_from(64u8);

        assert!(i.is_ok_and(|index| index == Square::try_from(0u8).unwrap()));
        assert!(j.is_ok_and(|index| index == Square::try_from(63u8).unwrap()));
        assert!(k.is_err_and(|err| err == IndexError::OutOfBounds(64u8)));
    }

    #[test]
    fn standard_index_try_from_usize_is_correct() {
        let i = Square::try_from(0usize);
        let j = Square::try_from(63usize);
        let k = Square::try_from(64usize);

        assert!(i.is_ok_and(|index| index == Square::try_from(0usize).unwrap()));
        assert!(j.is_ok_and(|index| index == Square::try_from(63usize).unwrap()));
        assert!(k.is_err_and(|err| err == IndexError::OutOfBounds(64usize)));
    }

    #[test]
    fn standard_index_try_from_string_slice_is_correct() {
        let i = Square::try_from("a3").unwrap();
        let j = Square::try_from("d6").unwrap();
        let k = Square::try_from("h7").unwrap();

        assert_eq!(i, Square::new(16));
        assert_eq!(j, Square::new(43));
        assert_eq!(k, Square::new(55));
    }

    #[test]
    fn standard_index_into_string_is_correct() {
        let a3: String = Square::new(16).into();
        let d6: String = Square::new(43).into();
        let h7: String = Square::new(55).into();

        assert_eq!(a3, String::from("a3"));
        assert_eq!(d6, String::from("d6"));
        assert_eq!(h7, String::from("h7"));
    }
}
