use crate::core::index::{Index, IndexError};
use std::ops::Deref;

// TODO: make standard index use nonmax::NonMaxU8

/// Represents a specific square on a `StandardBoard`
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct StandardIndex(u8);

impl Index for StandardIndex {
    type MetricTarget = u8;

    fn distance(a: Self, b: Self) -> Self::MetricTarget {
        todo!()
    }
}

impl Deref for StandardIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<u8> for StandardIndex {
    type Error = IndexError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            index @ 0..=63 => Ok(Self(index)),
            index @ _ => Err(IndexError::OutOfBounds(index)),
        }
    }
}

impl TryFrom<usize> for StandardIndex {
    type Error = IndexError<usize>;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let int: u8 = value
            .try_into()
            .map_err(|_err| IndexError::OutOfBounds(value))?;

        match StandardIndex::try_from(int) {
            Ok(index) => Ok(index),
            Err(_) => Err(IndexError::OutOfBounds(int as usize)),
        }
    }
}

impl From<StandardIndex> for usize {
    fn from(value: StandardIndex) -> Self {
        value.0 as usize
    }
}

impl<'a> TryFrom<&'a str> for StandardIndex {
    type Error = IndexError<&'a str>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        // TODO: complete function
        todo!()
    }
}

impl<'a> Into<&'a str> for StandardIndex {
    fn into(self) -> &'a str {
        // TODO: complete function
        todo!()
    }
}

impl StandardIndex {
    /// Attempts to construct a valid [`StandardIndex`]
    /// using the given value, and panics if that fails.
    ///
    /// Consider using `try_from(value: usize)` instead for
    /// safer code.
    ///
    /// This should be treated as a utility function,
    /// to avoid constantly writing `StandardIndex::try_from(val).unwrap()`.
    pub fn new(value: usize) -> Self {
        assert!(value <= 63);
        Self(value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_index_try_from_u8_is_correct() {
        let i = StandardIndex::try_from(0u8);
        let j = StandardIndex::try_from(63u8);
        let k = StandardIndex::try_from(64u8);

        assert!(i.is_ok_and(|index| index == StandardIndex::try_from(0u8).unwrap()));
        assert!(j.is_ok_and(|index| index == StandardIndex::try_from(63u8).unwrap()));
        assert!(k.is_err_and(|err| err == IndexError::OutOfBounds(64u8)));
    }

    #[test]
    fn standard_index_try_from_usize_is_correct() {
        let i = StandardIndex::try_from(0usize);
        let j = StandardIndex::try_from(63usize);
        let k = StandardIndex::try_from(64usize);

        assert!(i.is_ok_and(|index| index == StandardIndex::try_from(0usize).unwrap()));
        assert!(j.is_ok_and(|index| index == StandardIndex::try_from(63usize).unwrap()));
        assert!(k.is_err_and(|err| err == IndexError::OutOfBounds(64usize)));
    }

    // TODO: write tests for algebraic index conversions
}
