#![cfg_attr(feature = "slice_of_cells", feature(as_cell))]
#![cfg_attr(feature = "pinned", feature(pin))]
//! This crate provides an `IntoPin` trait.
//! `IntoPin` can be used to wrap any type in a [`Pin`],
//! but is powerfull in creating  coerced, pinned references.
//! 
//! [`Pin`]: https://doc.rust-lang.org/nightly/std/pin/struct.Pin.html

#[cfg(feature = "pinned")]
pub mod pinned;

#[cfg(all(test, feature = "pinned"))]
mod tests;