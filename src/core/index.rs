//! Traits for representing metric spaces composed of chessboard indices.

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
/// ## As a Metric Space
/// Indices on a chessboard form a set of positions equipped
/// with a notion of distance between them, and so they form
/// a [metric space](https://en.wikipedia.org/wiki/Metric_space).
/// On a standard board this is the Euclidean metric, whereas
/// a hypothetical spherical chessboard has a metric given by
/// the lengths of sections of great circles.
pub trait Index {
    /// The type of the distance between two indices.
    ///
    /// The [`PartialOrd`] bound is necessary for a valid distance
    /// metric to make coherent sense; in practice this implies
    /// that this type will almost always be a [`usize`] or [`f64`].
    type MetricTarget: PartialOrd;

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

/// Represents an index with a distinct per-piece
/// notion of distance.
///
/// ## Per-Piece Distance
/// Individual pieces experience the chessboard differently;
/// in some sense they induce different (typically discrete)
/// metrics depending on how they are permitted to move. As an example, the
/// king's movement is described by the [Chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance),
/// whereas the knight's movement has no associated analytic metric.
pub trait PieceMetric: Index {
    /// The set of piece kinds which this index
    /// defines distance metrics for.
    type PieceKind;

    /// The type of the distance between two indices,
    /// as perceived by a [`Piece`](super::Piece).
    ///
    /// The [`Ord`] bound is necessary for a valid distance
    /// metric to make coherent sense; in practice this implies
    /// that this type will almost always be a [`usize`] or [`f64`].
    type PieceMetricTarget: PartialOrd;

    /// Computes the distance between `a` and `b` from the perspective
    /// of a piece of kind `kind`.
    fn distance(kind: Self::PieceKind, a: Self, b: Self) -> Self::PieceMetricTarget
    where
        Self: Sized;
}
