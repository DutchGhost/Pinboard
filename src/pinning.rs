use crate::IntoPin;
use std::pin::Pin;
use std::marker::Unpin;
use std::borrow::{BorrowMut, Borrow};
use std::convert::{AsRef, AsMut};

pub trait Pinning<T> {

    fn pinning(self) -> Pin<T>;
}

// impl <'a, T: Borrow<U> + ?Sized, U: Unpin + ?Sized> Pinning<&'a U> for &'a T {
//     fn pinning(self) -> Pin<&'a U> {
//         Pin::new(self.borrow())
//     }
// }

// impl <'a, T: Borrow<U> + ?Sized, U: Unpin + ?Sized> Pinning<&'a U> for &'a mut T {
//     fn pinning(self) -> Pin<&'a U> {
//         Pin::new((&*self).borrow())
//     }
// }

// impl <'a, T: BorrowMut<U> + ?Sized, U: Unpin + ?Sized> Pinning<&'a mut U> for &'a mut T {
//     fn pinning(self) -> Pin<&'a mut U> {
//         Pin::new(self.borrow_mut())
//     }
// }

use std::ops::{DerefMut, Deref};

impl <'a, T: Deref + ?Sized> Pinning<&'a <T as Deref>::Target> for &'a T
where
    <T as Deref>::Target: Unpin
{
    fn pinning(self) -> Pin<&'a <T as Deref>::Target> {
        Pin::new(self.deref())
    }
}

impl <'a, T: Deref + ?Sized> Pinning<&'a <T as Deref>::Target> for &'a mut T
where
    <T as Deref>::Target: Unpin
{
    fn pinning(self) -> Pin<&'a <T as Deref>::Target> {
        Pin::new((&*self).deref())
    }
}

impl <'a, T: Deref + ?Sized> Pinning<&'a mut <T as Deref>::Target> for &'a mut T
where
    <T as Deref>::Target: Unpin
{
    fn pinning(mut self) -> Pin<&'a mut <T as Deref>::Target> {
        Pin::new()
    }
}

// impl <'a, T: AsRef<U> + ?Sized, U: Unpin + ?Sized> Pinning<&'a U> for &'a mut T {
//     fn pinning(self) -> Pin<&'a U> {
//         Pin::new((&*self).as_ref())
//     }
// }

// impl <'a, T: AsMut<U> + ?Sized, U: Unpin + ?Sized> Pinning<&'a mut U> for &'a mut T {
//     fn pinning(self) -> Pin<&'a mut U> {
//         Pin::new(self.as_mut())
//     }
// }

// impl <T, U: Unpin> Pinning<U> for T
// where
//     T: IntoPin<U>
// {
//     fn pinning(self) -> Pin<U> {
//         self.into_pin()
//     }
// }

// impl <'a, T: AsRef<U>, U> Pinning<U> for Pin<T>
// where
//     T: IntoPin<U>
// {
//     fn pinning(self) -> Pin<U> {
//         Pin::new(self.as_ref())
//     }
// }