use crate::core::index::{Index, IndexError};
use crate::standard::board::StandardBoard;
use std::ops::Deref;

/// Represents a specific square on a `StandardBoard`
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct StandardIndex(u8);

impl Index for StandardIndex {
    type Board = StandardBoard;
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

impl Into<usize> for StandardIndex {
    fn into(self) -> usize {
        self.0 as usize
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
