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
    fn vec_into_pin() {
        use super::pinned::IntoPin;
        let mut v = vec![1, 2, 3, 4];

        // &mut Vec<T> to Pin<&mut Vec<T>>
        {
            let mut pin: Pin<&mut Vec<u32>> = (&mut v).into_pin();
            pin[0] = 0;
        }

        // &mut Vec<T> to Pin<&mut [T]>
        {
            let mut pin: Pin<&mut [u32]> = (&mut v).into_pin();
            pin[1] = 0;
        }

        // &Vec<T> to Pin<&[T]>
        {
            let pin: Pin<&[u32]> = (&v).into_pin();

            assert_eq!(pin[..2], [0, 0][..]);
        }
        // &mut Box<T> to Pin<&T>
        {
            let pin: Pin<&[u32]> = (&mut v).into_pin();
            assert_eq!(pin[..3], [0, 0, 3][..]);
        }
    }

    #[test]
    fn test_arbitrary_into_pin() {
        use super::pinned::IntoPin;
        let mut n: u64 = 9;

        {
            let mut pin: Pin<&mut u64> = (&mut n).into_pin();
            *pin = 0;
        }

        assert_eq!(n, 0);
    }

    #[test]
    fn pin_into_pin() {
        use super::pinned::IntoPin;

        fn quark<'a, T: IntoPin<&'a mut u32>>(x: T) {
            let mut pin = x.into_pin();
            *pin = 0;
        }

        // test on pinned ref
        {
            let mut n = 9;
            let p: Pin<&mut u32> = (&mut n).into_pin();
            quark(p);
            assert_eq!(n, 0);
        }

        // test on plain ref
        {
            let mut n = 9;
            quark(&mut n);
            assert_eq!(n, 0);
        }
    }
}