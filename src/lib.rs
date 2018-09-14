#![cfg_attr(feature = "pinned", feature(pin))]

//! This crate provides an `AsPin` trait. AsPin can be used in the same fashion as `AsRef` and `AsMut`.

#[cfg(feature = "pinned")]
pub mod implement {
    use std::marker::Unpin;
    use std::pin::PinMut;

    /// Converts a mutable reference into a pinned reference. See the [`pin` module].
    ///
    /// [`pin` module]: https://doc.rust-lang.org/nightly/std/pin/struct.PinMut.html
    /// 
    /// # Examples
    /// ```
    /// #![feature(pin)]
    /// 
    /// extern crate pinboard;
    /// use pinboard::AsPin;
    /// 
    /// use std::pin::PinMut;
    /// 
    /// let mut v = vec![5, 4, 3, 2, 1];
    /// let mut pin: PinMut<[u32]> = v.as_pin();
    /// 
    /// pin.sort();
    /// 
    /// assert_eq!(pin.as_ref(), [1, 2, 3, 4, 5]);
    /// 
    /// ```
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