//! An abstract `Index` trait.

use super::board::StaticBoard;
use thiserror::Error;

/// The result of the incorrect creation or usage of
/// a particular index.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum IndexError<T> {
    /// The result of using a valid index in an invalid context.
    #[error("Received an out-of-bounds index: {0}")]
    OutOfBounds(T),
    /// The result of attempting to construct an invalid index.
    #[error("Received an index with invalid formatting: {0}")]
    InvalidFormat(T),
}

// TODO: are these trait bounds necessary?
/// Represents a particular place on the associated [`Board`]
pub trait Index: Into<usize> + TryFrom<usize> {}
