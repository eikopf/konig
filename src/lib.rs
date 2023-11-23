//! This crate provides traits and concrete implementations
//! for chess, both the standard game and some related variants.

// lints
#![warn(missing_docs)]
// features
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_flatten)]

pub mod core;
pub mod io;
pub mod standard;
