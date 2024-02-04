//! This crate provides traits and concrete implementations
//! for chess, both the standard game and some related variants.

// lints
#![warn(missing_docs)]

// nightly features
#![feature(never_type)]
#![feature(associated_type_bounds)]
#![feature(impl_trait_in_assoc_type)]
#![feature(portable_simd)]
#![feature(slice_as_chunks)]
#![feature(slice_flatten)]

pub mod bitboard;
pub mod core;
pub mod io;
pub mod quadboard;
pub mod standard;
