//! Traits for representing metric spaces composed of chessboard indices.

use std::{ops::Add, str::FromStr};
use thiserror::Error;

/// The result of the incorrect creation or usage of
/// a particular index.
#[derive(Error, Debug, Eq, PartialEq)]
pub enum IndexError<T> {
    /// The result of using a valid index in an invalid context.
    #[error("Received an out-of-bounds index: {0}")]
    OutOfBounds(T),
    /// The result of attempting to construct an invalid index.
    #[error("Received an index with an invalid format: {0}")]
    InvalidFormat(T),
}

/// Represents an [`Index`] which can be derived from an
/// algebraic notation string. 
///
/// Standard chess implements this with a simple file character 
/// and rank digit, but other variants may have more complex systems.
pub trait Algebraic: Index + FromStr<Err = IndexError<String>> {
    /// The type representing the file component of the [`Index`].
    type File;
    /// The type representing the rank component of the [`Index`].
    type Rank;

    /// Returns the file component of the [`Index`].
    fn file(&self) -> Self::File;
    /// Returns the rank component of the [`Index`].
    fn rank(&self) -> Self::Rank;
}

/// Represents an [`Index`] with an associated notion
/// of color.
pub trait Colored: Index {
    /// The type of the colors which this [`Index`] may be.
    type Color;

    /// Returns the color of the [`Index`].
    fn color(&self) -> Self::Color;
}

/// Represents a particular position on a given board.
pub trait Index {}

/// Represents an [`Index`] equipped with the structure
/// of a metric space.
///
/// ## As a Metric Space
/// Indices on a chessboard form a set of positions equipped
/// with a notion of distance between them, and so they form
/// a [metric space](https://en.wikipedia.org/wiki/Metric_space).
/// On a standard board this is the Euclidean metric, whereas
/// a hypothetical spherical chessboard has a metric given by
/// the lengths of sections of great circles.
pub trait Metric: Index + Sized {
    /// The type of the distance between two indices.
    ///
    /// These trait bounds are derived from the axioms
    /// which govern the distance function on metric spaces.
    type MetricTarget: PartialOrd + Eq + Add<Self, Output = Self::MetricTarget>;

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
    fn distance(a: Self, b: Self) -> Self::MetricTarget;
}
