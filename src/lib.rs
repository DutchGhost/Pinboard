#![cfg_attr(feature = "pinned", feature(pin, nll))]

//! This crate provides an `AsPin` trait. AsPin can be used in the same fashion as `AsRef` and `AsMut`.

#[cfg(feature = "pinned")]
pub mod pinned;

#[cfg(all(test, feature = "pinned"))]
mod tests {
    use std::pin::Pin;
 
    #[test]
    fn box_as_pin() {
        use super::pinned::AsPin;
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
        use super::pinned::AsPinMut;
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

    #[test]
    fn box_into_pin() {
        use super::pinned::IntoPin;
        let mut b = Box::new(1);

        // &mut Box<T> to Pin<&mut Box<T>>
        {
            let pin: Pin<&mut Box<u32>> = (&mut b).into_pin();
        }

        // &mut Box<T> to Pin<&mut T>
        {
            let pin: Pin<&mut u32> = (&mut b).into_pin();
        }

        // &Box<T> to Pin<&T>
        {
            let pin: Pin<&u32> = (&b).into_pin();
        }
        // &mut Box<T> to Pin<&T>
        {
            let pin: Pin<&u32> = (&mut b).into_pin();
        }
    }

    #[test]
    fn test_arbitrary_into_pin() {
        use super::pinned::IntoPin;
        let mut n: u64 = 9;

        {
            let pin: Pin<&mut u64> = (&mut n).into_pin();
        }
    }
}