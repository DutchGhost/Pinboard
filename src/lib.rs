#![cfg_attr(feature = "pinned", feature(pin, nll))]

//! This crate provides an `AsPin` trait. AsPin can be used in the same fashion as `AsRef` and `AsMut`.

#[cfg(feature = "pinned")]
pub mod as_pin {
    use std::pin::Pin;
    use std::marker::Unpin;

    /// Converts a non-mutable reference into a non-mutable pinned reference.
    pub trait AsPin<P: ?Sized> {

        /// Performs the conversion.
        fn as_pin(&self) -> Pin<&P>;
    }

    ///////////////////////////////////////////////
    // GENERIC IMPL
    ///////////////////////////////////////////////

    impl <'a, T: AsPin<U>, U> AsPin<U> for &'a T {
        #[inline]
        fn as_pin(&self) -> Pin<&U> {
            (*self).as_pin()
        }
    }

    impl <'a, T: AsPin<U>, U> AsPin<U> for &'a mut T {
        #[inline]
        fn as_pin(&self) -> Pin<&U> {
            <T as AsPin<U>>::as_pin(*self)
        }
    }

    ///////////////////////////////////////////////
    // SLICE IMPL
    ///////////////////////////////////////////////
    impl<T: Unpin> AsPin<[T]> for [T] {
        #[inline]
        fn as_pin(&self) -> Pin<&[T]> {
            Pin::new(self)
        }
    }

    ///////////////////////////////////////////////
    // BOX IMPL
    ///////////////////////////////////////////////
    impl<T: Unpin> AsPin<T> for Box<T> {
        #[inline]
        fn as_pin(&self) -> Pin<&T> {
            Pin::new(self.as_ref())
        }
    }

    impl<T: Unpin> AsPin<Box<T>> for Box<T> {
        #[inline]
        fn as_pin(&self) -> Pin<&Box<T>> {
            Pin::new(self)
        }
    }

    ///////////////////////////////////////////////
    // VEC IMPL
    ///////////////////////////////////////////////
    
    impl<T: Unpin> AsPin<[T]> for Vec<T> {
        #[inline]
        fn as_pin(&self) -> Pin<&[T]> {
            Pin::new(self)
        }
    }

    impl<T: Unpin> AsPin<Vec<T>> for Vec<T> {
        #[inline]
        fn as_pin(&self) -> Pin<&Vec<T>> {
            Pin::new(self)
        }
    }
}

#[cfg(feature = "pinned")]
pub mod as_pin_mut {
    use super::as_pin::AsPin;

    use std::pin::Pin;
    use std::marker::Unpin;

    /// Converts a mutable reference into a mutable pinned reference.
    pub trait AsPinMut<P: ?Sized>: AsPin<P> {
        /// Performs the conversion.
        fn as_pin_mut(&mut self) -> Pin<&mut P>;
    }

    ///////////////////////////////////////////////
    // GENERIC IMPL
    ///////////////////////////////////////////////

    impl <'a, T: AsPinMut<U>, U> AsPinMut<U> for &'a mut T {
        #[inline]
        fn as_pin_mut(&mut self) -> Pin<&mut U> {
            <T as AsPinMut<U>>::as_pin_mut(*self)
        }
    }

    ///////////////////////////////////////////////
    // SLICE IMPL
    ///////////////////////////////////////////////
    impl<T: Unpin> AsPinMut<[T]> for [T] {
        #[inline]
        fn as_pin_mut(&mut self) -> Pin<&mut [T]> {
            Pin::new(self)
        }
    }

    ///////////////////////////////////////////////
    // BOX IMPL
    ///////////////////////////////////////////////
    impl<T: Unpin> AsPinMut<T> for Box<T> {
        #[inline]
        fn as_pin_mut(&mut self) -> Pin<&mut T> {
            Pin::new(self.as_mut())
        }
    }

    impl<T: Unpin> AsPinMut<Box<T>> for Box<T> {
        #[inline]
        fn as_pin_mut(&mut self) -> Pin<&mut Box<T>> {
            Pin::new(self)
        }
    }

    ///////////////////////////////////////////////
    // VEC IMPL
    ///////////////////////////////////////////////
    
    impl<T: Unpin> AsPinMut<[T]> for Vec<T> {
        #[inline]
        fn as_pin_mut(&mut self) -> Pin<&mut [T]> {
            Pin::new(self)
        }
    }

    impl<T: Unpin> AsPinMut<Vec<T>> for Vec<T> {
        #[inline]
        fn as_pin_mut(&mut self) -> Pin<&mut Vec<T>> {
            Pin::new(self)
        }
    }
}

#[cfg(all(test, feature = "pinned"))]
mod tests {
    use std::pin::Pin;
 
    #[test]
    fn box_as_pin() {
        use super::as_pin::AsPin;
        let b = Box::new(1);

        // &Box to Pin<&Box>
        {
            let pin: Pin<&Box<u32>> = b.as_pin();
        }

        // &Box to Pin<&T>
        {
            let pin: Pin<&u32> = (&b).as_pin();
        }
    }

     #[test]
    fn box_as_pin_mut() {
        use super::as_pin_mut::AsPinMut;
        let mut b = Box::new(1);

        // &mut Box to Pin<&mut Box>
        {
            let pin: Pin<&mut Box<u32>> = b.as_pin_mut();
        }

        // &mut Box to Pin<&mut T>
        {
            let pin: Pin<&mut u32> = b.as_pin_mut();
        }
    }
}

#[cfg(feature = "pinned")]
pub use self::as_pin::AsPin;
pub use self::as_pin_mut::AsPinMut;
