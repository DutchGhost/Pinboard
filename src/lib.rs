#![cfg_attr(feature = "pinned", feature(pin))]

//! This crate provides an `AsPin` trait. AsPin can be used in the same fashion as `AsRef` and `AsMut`.

#[cfg(feature = "pinned")]
pub mod implement {
    use std::marker::Unpin;
    use std::pin::PinMut;

    /// Converts a mutable reference into a Pinned reference. See the [`pin` module].
    ///
    /// [`pin` module]: https://doc.rust-lang.org/nightly/std/pin/struct.PinMut.html
    pub trait AsPin<T: ?Sized + Unpin> {
        /// Performs the conversion.
        fn as_pin(&mut self) -> PinMut<T>;
    }

    ///////////////////////////////////////////////
    // GENERIC IMPL
    ///////////////////////////////////////////////

    impl<'a, T: ?Sized, U: ?Sized + Unpin> AsPin<U> for &'a mut T
    where
        T: AsPin<U>,
    {
        #[inline]
        fn as_pin(&mut self) -> PinMut<U> {
            (*self).as_pin()
        }
    }

    ///////////////////////////////////////////////
    // SLICE IMPLS
    ///////////////////////////////////////////////
    impl<T: Unpin> AsPin<[T]> for [T] {
        #[inline]
        fn as_pin(&mut self) -> PinMut<Self> {
            PinMut::new(self)
        }
    }

    impl AsPin<str> for str {
        #[inline]
        fn as_pin(&mut self) -> PinMut<Self> {
            PinMut::new(self)
        }
    }

    ///////////////////////////////////////////////
    // OTHER IMPLS
    ///////////////////////////////////////////////
    impl<T: Unpin + ?Sized> AsPin<T> for Box<T> {
        #[inline]
        fn as_pin(&mut self) -> PinMut<T> {
            PinMut::new(self)
        }
    }

    impl<T: Unpin> AsPin<[T]> for Vec<T> {
        #[inline]
        fn as_pin(&mut self) -> PinMut<[T]> {
            PinMut::new(self)
        }
    }

    impl<T: Unpin> AsPin<Vec<T>> for Vec<T> {
        #[inline]
        fn as_pin(&mut self) -> PinMut<Vec<T>> {
            PinMut::new(self)
        }
    }

    impl AsPin<String> for String {
        #[inline]
        fn as_pin(&mut self) -> PinMut<String> {
            PinMut::new(self)
        }
    }
}


#[cfg(feature = "pinned")]
pub use self::implement::AsPin;
