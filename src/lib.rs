#![cfg_attr(feature = "pinned", feature(pin, nll))]

//! This crate provides an `AsPin` trait. AsPin can be used in the same fashion as `AsRef` and `AsMut`.

#[cfg(feature = "pinned")]
pub mod implement {
    use std::pin::Pin;
    use std::ops::Deref;
    use std::marker::Unpin;


    /// Converts a mutable reference into a pinned reference. See the [`pin` module].
    ///
    /// [`pin` module]: https://doc.rust-lang.org/nightly/std/pin/struct.Pin.html
    ///
    /// # Examples
    /// ```
    /// #![feature(pin)]
    ///
    /// extern crate pinpoint;
    /// use pinpoint::AsPin;
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
    pub trait AsPin<P: Deref>
    where
        <P as Deref>::Target: Unpin
    {
        /// Performs the conversion.
        fn as_pin(&mut self) -> Pin<P>;
    }

    ///////////////////////////////////////////////
    // GENERIC IMPL
    ///////////////////////////////////////////////

    impl<'a, P: ?Sized, U> AsPin<U> for &'a mut P
    where
        P: AsPin<U>,
        U: Deref,
        <U as Deref>::Target: Unpin
    {
        #[inline]
        fn as_pin(&mut self) -> Pin<U> {
            (*self).as_pin()
        }
    }

    ///////////////////////////////////////////////
    // SLICE IMPLS
    ///////////////////////////////////////////////
    impl<'a, T: Unpin> AsPin<&'a [T]> for &'a [T] {
        #[inline]
        fn as_pin(&mut self) -> Pin<&'a [T]> {
            Pin::new(self)
        }
    }

    impl<'a, T: Unpin> AsPin<&'a mut [T]> for &'a mut [T]
    {
        #[inline]
        fn as_pin(&mut self) -> Pin<&'a mut [T]> {
            Pin::new(self)
        }
    }

    // ///////////////////////////////////////////////
    // // OTHER IMPLS
    // ///////////////////////////////////////////////
    // impl<T: Unpin + ?Sized> AsPin<T> for Box<T> {
    //     #[inline]
    //     fn as_pin(&mut self) -> Pin<T> {
    //         Pin::new(self)
    //     }
    // }

    // impl<T: Unpin> AsPin<[T]> for Vec<T> {
    //     #[inline]
    //     fn as_pin(&mut self) -> Pin<[T]> {
    //         Pin::new(self)
    //     }
    // }

    // impl<T: Unpin> AsPin<Vec<T>> for Vec<T> {
    //     #[inline]
    //     fn as_pin(&mut self) -> Pin<Vec<T>> {
    //         Pin::new(self)
    //     }
    // }

    // impl AsPin<String> for String {
    //     #[inline]
    //     fn as_pin(&mut self) -> Pin<String> {
    //         Pin::new(self)
    //     }
    // }
}

#[cfg(feature = "pinned")]
pub use self::implement::AsPin;
