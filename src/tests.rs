use std::pin::Pin;

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

    let pinbox: Pin<Box<u32>> = b.into_pin();
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

#[test]
fn pinned_ref_to_pinned_ref() {
    use super::pinned::IntoPin;
    use std::borrow::BorrowMut;
    use std::borrow::Borrow;
    
    fn quazr<'a, P, T: 'a>(x: P)
    where
        P: IntoPin<&'a [T]>
    {
        let pin: Pin<&'a [T]> = x.into_pin();
    }

    let mut v = vec![1, 2, 3, 4];

    let mut pin: Pin<&mut [u32]> = (&mut v).into_pin();

    {
        quazr(<&mut Pin<&mut [u32]> as IntoPin<&[u32]>>::into_pin(&mut pin));
    }
    {
        quazr(<&mut Pin<&mut [u32]> as IntoPin<&[u32]>>::into_pin(&mut pin));
    }
}