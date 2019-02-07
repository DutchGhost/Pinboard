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

    fn quazr<'a, P: 'a, T: 'a>(x: P)
    where
        P: IntoPin<&'a mut [T]>,
    {

    }
    let mut v = vec![1, 2, 3, 4];

    let mut pin: Pin<&mut [u32]> = (&mut v).into_pin();

    quazr(&mut pin);
    quazr(&mut pin);
    quazr(&mut pin);
}

#[test]
fn variants() {
    use super::pinned::IntoPin;

    fn to_ref<'a, P, T: 'a>(x: P)
    where
        P: IntoPin<&'a T>,
    {

    }

    fn to_mut<'a, P, T: 'a>(x: P)
    where
        P: IntoPin<&'a mut T>,
    {

    }

    let mut n = 0;
    // PIN<&T> TO PIN<&T>
    let pin: Pin<&u32> = (&n).into_pin();
    to_ref::<_, u32>(pin);

    // PIN<&MUT T> TO PIN<&T>
    let pin: Pin<&mut u32> = (&mut n).into_pin();
    to_ref::<_, u32>(pin);

    // &PIN<&T> TO PIN<&T>
    let pin: Pin<&u32> = (&n).into_pin();
    let pinref: &Pin<&u32> = &pin;
    to_ref::<_, u32>(pinref);

    // &PIN<&MUT T> TO PIN<&T>
    let mut pin: Pin<&mut u32> = (&mut n).into_pin();
    let pinref: &Pin<&mut u32> = &pin;
    to_ref::<_, u32>(pinref);

    // &MUT PIN<&T> TO PIN<&T>
    let mut pin: Pin<&u32> = (&n).into_pin();
    let pinref: &mut Pin<&u32> = &mut pin;
    to_ref::<_, u32>(pinref);

    // &MUT PIN<&MUT> TO PIN<&T>
    let mut pin: Pin<&mut u32> = (&mut n).into_pin();
    let pinref: &mut Pin<&mut u32> = &mut pin;
    to_ref::<_, u32>(pinref);

    // PIN<&MUT T> TO PIN<&MUT T>
    let pin: Pin<&mut u32> = (&mut n).into_pin();
    to_mut::<_, u32>(pin);

    // &MUT PIN<&MUT T> TO PIN<&MUT T>
    let mut pin: Pin<&mut u32> = (&mut n).into_pin();
    let pinref: &mut Pin<&mut u32> = &mut pin;
    to_mut::<_, u32>(pinref);

    let mut p = Pin::new(Box::new(n));
    // &PIN<T> TO PIN<&T>
    to_ref(&p);

    // &MUT PIN<T> TO PIN<&T>
    to_ref(&mut p);

    // &MUT PIN<T> TO PIN<&MUT T>
    to_mut(&mut p);
}

#[test]
fn pinned_str_to_pinned_bytes() {
    use super::pinned::IntoPin;
    use std::borrow::Borrow;

    fn quark<'a, R, P>(x: R)
    where
        R: Borrow<P>,
        P: IntoPin<&'a [u8]>,
    {
        let f: Pin<&_> = (&x.borrow()).into_pin();
    }

    let s = "hello";

    quark::<_, &str>(&s);

    let pinned_str: Pin<&[u8]> = s.into_pin();
    quark(pinned_str);
}

#[test]
fn test_pinning() {
    use super::pinned::IntoPin;

    fn quarck<'a, T>(x: T)
    where
        T: IntoPin<&'a [u8]>,
    {

    }

    let s = "foo";

    quarck(s);

    let b: Box<[u8]> = Box::new([1, 2, 3, 4]);
    let mut b: Pin<Box<[u8]>> = b.into_pin();

    quarck(&mut *b);
}
