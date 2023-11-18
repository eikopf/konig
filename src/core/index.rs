//! An abstract `Index` trait.

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

/// Represents a particular position on a given board.
///
/// Indices on a chessboard form a set of positions equipped
/// with a notion of distance between them, and so they form
/// a metric space. The obvious example of this is the Chebyshev
/// distance, which describes the distance between squares on a
/// standard chessboard (from the perspective of the king).
pub trait Index {
    /// The type of the distance between two indices.
    ///
    /// The [`Ord`] bound is necessary for a valid distance
    /// metric to make coherent sense; in practice this implies
    /// that this type will almost always be a [`usize`] or [`f64`].
    type MetricTarget: Ord;

    /// Computes the distance between `a` and `b`.
    ///
    /// In particular, the following must hold:
    /// - `distance(a, b)` == `distance(b, a)`;
    /// - `distance(a, a)` == `0`;
    /// - `a != b` ==> `distance(a, b) > 0`;
    /// - `distance(a, c)` <= `distance(a, b) + distance(b, c)`
    ///
    /// These are the axioms which a metric space must uphold,
    /// though in general most intutitive notions of distance will
    /// already fulfil these requirements.
    fn distance(a: Self, b: Self) -> Self::MetricTarget
    where
        Self: Sized;
}
