#![cfg_attr(feature = "slice_of_cells", feature(as_cell))]
#![cfg_attr(feature = "pinned", feature(pin))]
//! This crate provides the `IntoPin` trait.
//! `IntoPin` can be used to wrap any type in a [`Pin`],
//! but is powerfull in creating  coerced, pinned references.
//!
//! [`Pin`]: https://doc.rust-lang.org/nightly/std/pin/struct.Pin.html
//! # Examples
//! ```
//! #![feature(pin)]
//!
//! extern crate pinpoint;
//! use std::pin::Pin;
//! use pinpoint::IntoPin;
//!
//! let v = vec![1, 2, 3, 4, 5];
//!
//! let pinned_slice: Pin<&[u32]> = (&v).into_pin();
//! ```
//!
//! An example using generics:
//! ```
//! #![feature(pin)]
//!
//! extern crate pinpoint;
//! use std::pin::Pin;
//! use pinpoint::IntoPin;
//!
//! fn example<'a, P>(item: P)
//! where
//!     P: IntoPin<&'a mut [u8]>
//! {
//!     let mut pin = item.into_pin();
//!
//!     pin.reverse();
//! }
//!
//! let mut v = vec![1, 2, 3, 4];
//! example(&mut v);
//! assert_eq!(v, [4, 3, 2, 1]);
//!
//! let mut b: Box<[u8]> = Box::new([4, 3, 2, 1]);
//! example(&mut b);
//! assert_eq!(*b, [1, 2, 3, 4]);
//! ```
//! # Features
//! 
//! In order to use the `IntoPin` trait, this crate should be used with the feature `pinned` of this crate turned on.
//! In order to create a pinned slice containg Cell types from a Cell containing a slice, use the `slice_of_cells` feature of this crate.

#[cfg(feature = "pinned")]
pub mod pinned;

#[cfg(feature = "pinned")]
pub use self::pinned::IntoPin;

#[cfg(all(test, feature = "pinned"))]
mod tests;
