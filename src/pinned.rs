use std::borrow::Cow;
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::marker::Unpin;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

/// A trait that wraps any type implementing `Unpin` into a `Pin`.
/// # Examples
/// ```
/// #![feature(pin)]
///
/// extern crate pinpoint;
/// use std::pin::Pin;
/// use pinpoint::IntoPin;
///
/// let v = vec![1, 2, 3, 4, 5];
///
/// let pinned_slice: Pin<&[u32]> = (&v).into_pin();
/// ```
/// An example using generics:
///
/// ```
/// #![feature(pin)]
///
/// extern crate pinpoint;
/// use std::pin::Pin;
/// use pinpoint::IntoPin;
///
/// fn example<'a, P>(item: P)
/// where
///     P: IntoPin<&'a mut [u8]>
/// {
///     let mut pin = item.into_pin();
///
///     pin.reverse();
/// }
///
/// let mut v = vec![1, 2, 3, 4];
/// example(&mut v);
/// assert_eq!(v, [4, 3, 2, 1]);
///
/// let mut b: Box<[u8]> = Box::new([4, 3, 2, 1]);
/// example(&mut b);
/// assert_eq!(*b, [1, 2, 3, 4]);
/// ```
pub trait IntoPin<T: Unpin> {
    /// Performs the wrapping.
    fn into_pin(self) -> Pin<T>;
}

///////////////////////////////////////////////
// Pin<T> IMPL
//
// @NOTE
// &'a &'b T requires 'b: 'a, not 'a: b.
///////////////////////////////////////////////
// Also covers Pin<&'a T> to Pin<&'b T> where 'a: 'b
impl<T: Unpin> IntoPin<T> for Pin<T> {
    #[inline]
    fn into_pin(self) -> Self {
        self
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for Pin<&'a mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::into_ref(self)
    }
}

impl<'short, 'long, T: Unpin> IntoPin<&'short T> for &'short Pin<&'long T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self)
    }
}

// Mutable reference to pin of reference into pin of reference
impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut Pin<&'long T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(&*self)
    }
}

// mutable reference to pin of mutable reference into pin of mutable reference
impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short mut T> for &'short mut Pin<&'long mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short mut T> {
        Pin::new(&mut *self)
    }
}

// mutable reference to pin of mutable reference into pin of reference
impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut Pin<&'long mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(&*self)
    }
}

// reference to pin of mutable reference into pin of reference
impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short Pin<&'long mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// GENERIC IMPL
///////////////////////////////////////////////
impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a T {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self)
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a mut T {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self)
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a mut T> for &'a mut T {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// VEC IMPL
///////////////////////////////////////////////
impl<T: Unpin> IntoPin<Vec<T>> for Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl<T: Unpin> IntoPin<Box<[T]>> for Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<Box<[T]>> {
        Pin::new(self.into_boxed_slice())
    }
}

impl<'a, T: Unpin> IntoPin<&'a [T]> for &'a Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a [T]> {
        Pin::new(self)
    }
}

impl<'a, T: Unpin> IntoPin<&'a [T]> for &'a mut Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a [T]> {
        Pin::new(self)
    }
}

impl<'a, T: Unpin> IntoPin<&'a mut [T]> for &'a mut Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut [T]> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// STRING IMPL
///////////////////////////////////////////////
impl IntoPin<String> for String {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl IntoPin<Box<str>> for String {
    #[inline]
    fn into_pin(self) -> Pin<Box<str>> {
        Pin::new(self.into_boxed_str())
    }
}

impl IntoPin<Vec<u8>> for String {
    #[inline]
    fn into_pin(self) -> Pin<Vec<u8>> {
        Pin::new(self.into_bytes())
    }
}

impl<'a> IntoPin<&'a str> for &'a String {
    #[inline]
    fn into_pin(self) -> Pin<&'a str> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a str> for &'a mut String {
    #[inline]
    fn into_pin(self) -> Pin<&'a str> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a mut str> for &'a mut String {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut str> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a [u8]> for &'a String {
    #[inline]
    fn into_pin(self) -> Pin<&'a [u8]> {
        Pin::new(self.as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// BOX IMPL
///////////////////////////////////////////////
impl<T: Unpin + ?Sized> IntoPin<Box<T>> for Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.as_ref())
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a mut Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.as_mut())
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a mut T> for &'a mut Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(self.as_mut())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// COW IMPL
///////////////////////////////////////////////
impl<'a, T: Clone + Unpin + ?Sized> IntoPin<Cow<'a, T>> for Cow<'a, T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl<'short, 'long, T: Clone + Unpin + ?Sized> IntoPin<&'short T> for &'short Cow<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self.as_ref())
    }
}

impl<'short, 'long, T: Clone + Unpin + ?Sized> IntoPin<&'short T> for &'short mut Cow<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {

        Pin::new(&*self)
    }
}

impl<'a> IntoPin<Cow<'a, [u8]>> for Cow<'a, str> {
    #[inline]
    fn into_pin(self) -> Pin<Cow<'a, [u8]>> {
        match self {
            Cow::Owned(o) => Pin::new(Cow::Owned(o.into_bytes())),
            Cow::Borrowed(b) => Pin::new(Cow::Borrowed(b.as_bytes())),
        }
    }
}

impl<'a, 'b> IntoPin<&'a [u8]> for &'a Cow<'b, str> {
    #[inline]
    fn into_pin(self) -> Pin<&'a [u8]> {
        // Asref into &str, then Asref into &[u8].
        Pin::new(self.as_ref().as_ref())
    }
}

impl<'a, 'b> IntoPin<&'a [u8]> for &'a mut Cow<'b, str> {
    #[inline]
    fn into_pin(self) -> Pin<&'a [u8]> {
        // Asref into &str, then Asref into &[u8].
        Pin::new((&*self).as_ref().as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// ARC IMPL
///////////////////////////////////////////////
impl<T: Unpin + ?Sized> IntoPin<Arc<T>> for Arc<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

// @NOTE: THIS *CLONES* IF THERE ARE OTHER ARC OR WEAK POINTERS TO THE T
// (https://doc.rust-lang.org/nightly/std/sync/struct.Arc.html#method.make_mut)
impl<'a, T: Unpin + Clone + ?Sized> IntoPin<&'a mut T> for &'a mut Arc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(Arc::make_mut(self))
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a mut Arc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(&*self)
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a Arc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// RC IMPL
///////////////////////////////////////////////
impl<T: Unpin + ?Sized> IntoPin<Rc<T>> for Rc<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

// @NOTE: THIS *CLONES* IF THERE ARE OTHER RC OR WEAK POINTERS TO THE T
// (https://doc.rust-lang.org/nightly/std/rc/struct.Rc.html#method.make_mut)
impl<'a, T: Unpin + Clone + ?Sized> IntoPin<&'a mut T> for &'a mut Rc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(Rc::make_mut(self))
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a mut Rc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(&*self)
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'a Rc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// REFCELL IMPL
///////////////////////////////////////////////
impl<'a, T: Unpin + ?Sized> IntoPin<Ref<'a, T>> for &'a RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<Ref<'a, T>> {
        Pin::new(self.borrow())
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<Ref<'a, T>> for &'a mut RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<Ref<'a, T>> {
        Pin::new(self.borrow())
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<RefMut<'a, T>> for &'a RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<RefMut<'a, T>> {
        Pin::new(self.borrow_mut())
    }
}

impl<'a, T: Unpin + ?Sized> IntoPin<RefMut<'a, T>> for &'a mut RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<RefMut<'a, T>> {
        Pin::new(self.borrow_mut())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// REF IMPL
///////////////////////////////////////////////
impl<'a, T: Unpin + ?Sized> IntoPin<Ref<'a, T>> for Ref<'a, T> {
    #[inline]
    fn into_pin(self) -> Pin<Ref<'a, T>> {
        Pin::new(self)
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short Ref<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self)
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut Ref<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// REFMUT IMPL
///////////////////////////////////////////////
impl<'a, T: Unpin + ?Sized> IntoPin<RefMut<'a, T>> for RefMut<'a, T> {
    #[inline]
    fn into_pin(self) -> Pin<RefMut<'a, T>> {
        Pin::new(self)
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short RefMut<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self)
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut RefMut<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self)
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short mut T> for &'short mut RefMut<'long, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short mut T> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// CELL IMPL
///////////////////////////////////////////////
impl<'a, T: Unpin> IntoPin<&'a T> for &'a mut Cell<T> {
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.get_mut())
    }
}

impl<'a, T: Unpin> IntoPin<&'a mut T> for &'a mut Cell<T> {
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(self.get_mut())
    }
}

#[cfg(feature = "slice_of_cells")]
impl<'a, T: Unpin> IntoPin<&'a [Cell<T>]> for &'a Cell<[T]> {
    fn into_pin(self) -> Pin<&'a [Cell<T>]> {
        Pin::new(self.as_slice_of_cells())
    }
}

#[cfg(feature = "slice_of_cells")]
impl<'a, T: Unpin> IntoPin<&'a [Cell<T>]> for &'a mut Cell<[T]> {
    fn into_pin(self) -> Pin<&'a [Cell<T>]> {
        Pin::new(self.as_slice_of_cells())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////
