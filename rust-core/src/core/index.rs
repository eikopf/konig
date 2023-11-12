//! An abstract `Index` trait.

use super::board::Board;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexError<T> {
    #[error("Received an out-of-bounds index: {0}")]
    OutOfBounds(T),
    #[error("Received an index with invalid formatting: {0}")]
    InvalidFormat(T),
}

pub trait Index: Into<usize> {
    type Board: Board;
}
