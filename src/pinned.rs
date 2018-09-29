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
///////////////////////////////////////////////
// Also covers Pin<&'a T> to Pin<&'b T> where 'a: 'b
impl<T: Unpin> IntoPin<T> for Pin<T> {
    #[inline]
    fn into_pin(self) -> Self {
        self
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for Pin<&'a mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::into_ref(self)
    }
}

impl<'b, 'a: 'b, T: Unpin> IntoPin<&'b T> for &'a Pin<&'b T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

// Mutable reference to pin of reference into pin of reference
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut Pin<&'b T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(&*self)
    }
}

// mutable reference to pin of mutable reference into pin of mutable reference
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b mut T> for &'a mut Pin<&'b mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b mut T> {
        Pin::new(&mut *self)
    }
}

// mutable reference to pin of mutable reference into pin of reference
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut Pin<&'b mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(&*self)
    }
}

// reference to pin of mutable reference into pin of reference
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a Pin<&'b mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// GENERIC IMPL
///////////////////////////////////////////////
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a T {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut T {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b mut T> for &'a mut T {
    #[inline]
    fn into_pin(self) -> Pin<&'b mut T> {
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

impl<'b, 'a: 'b, T: Unpin> IntoPin<&'b [T]> for &'a Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b [T]> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, T: Unpin> IntoPin<&'b [T]> for &'a mut Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b [T]> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, T: Unpin> IntoPin<&'b mut [T]> for &'a mut Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b mut [T]> {
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

impl<'b, 'a: 'b> IntoPin<&'b str> for &'a String {
    #[inline]
    fn into_pin(self) -> Pin<&'b str> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b> IntoPin<&'b str> for &'a mut String {
    #[inline]
    fn into_pin(self) -> Pin<&'b str> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b> IntoPin<&'b mut str> for &'a mut String {
    #[inline]
    fn into_pin(self) -> Pin<&'b mut str> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b> IntoPin<&'b [u8]> for &'a String {
    #[inline]
    fn into_pin(self) -> Pin<&'b [u8]> {
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

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self.as_ref())
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self.as_mut())
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b mut T> for &'a mut Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b mut T> {
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

impl<'b, 'a: 'b, 'c, T: Clone + Unpin + ?Sized> IntoPin<&'b T> for &'a Cow<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self.as_ref())
    }
}

impl<'b, 'a: 'b, 'c, T: Clone + Unpin + ?Sized> IntoPin<&'b T> for &'a mut Cow<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {

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

impl<'b, 'a: 'b> IntoPin<&'b [u8]> for &'a Cow<'a, str> {
    #[inline]
    fn into_pin(self) -> Pin<&'b [u8]> {
        // Asref into &str, then Asref into &[u8].
        Pin::new(self.as_ref().as_ref())
    }
}

impl<'b, 'a: 'b, 'c> IntoPin<&'b [u8]> for &'a mut Cow<'c, str> {
    #[inline]
    fn into_pin(self) -> Pin<&'b [u8]> {
        // Asref into &str, then Asref into &[u8].
        Pin::new(<Cow<'c, str> as AsRef<str>>::as_ref(self).as_ref())
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
impl<'a, 'b: 'a, T: Unpin + Clone + ?Sized> IntoPin<&'a mut T> for &'b mut Arc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(Arc::make_mut(self))
    }
}

impl<'a, 'b: 'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'b mut Arc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(&*self)
    }
}

impl<'a, 'b: 'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'b Arc<T> {
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
impl<'a, 'b: 'a, T: Unpin + Clone + ?Sized> IntoPin<&'a mut T> for &'b mut Rc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(Rc::make_mut(self))
    }
}

impl<'a, 'b: 'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'b mut Rc<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(&*self)
    }
}

impl<'a, 'b: 'a, T: Unpin + ?Sized> IntoPin<&'a T> for &'b Rc<T> {
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
impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<Ref<'b, T>> for &'a RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<Ref<'b, T>> {
        Pin::new(self.borrow())
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<Ref<'b, T>> for &'a mut RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<Ref<'b, T>> {
        Pin::new(self.borrow())
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<RefMut<'b, T>> for &'a RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<RefMut<'b, T>> {
        Pin::new(self.borrow_mut())
    }
}

impl<'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<RefMut<'b, T>> for &'a mut RefCell<T> {
    #[inline]
    fn into_pin(self) -> Pin<RefMut<'b, T>> {
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

impl<'b, 'a: 'b, 'c, T: Unpin + ?Sized> IntoPin<&'b T> for &'a Ref<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, 'c, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut Ref<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
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

impl<'b, 'a: 'b, 'c, T: Unpin + ?Sized> IntoPin<&'b T> for &'a RefMut<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, 'c, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut RefMut<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl<'b, 'a: 'b, 'c, T: Unpin + ?Sized> IntoPin<&'b mut T> for &'a mut RefMut<'c, T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b mut T> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// CELL IMPL
///////////////////////////////////////////////
impl<'b, 'a: 'b, T: Unpin> IntoPin<&'a T> for &'a mut Cell<T> {
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.get_mut())
    }
}

impl<'b, 'a: 'b, T: Unpin> IntoPin<&'a mut T> for &'a mut Cell<T> {
    fn into_pin(self) -> Pin<&'a mut T> {
        Pin::new(self.get_mut())
    }
}

#[cfg(feature = "slice_of_cells")]
impl<'b, 'a: 'b, T: Unpin> IntoPin<&'a [Cell<T>]> for &'a Cell<[T]> {
    fn into_pin(self) -> Pin<&'a [Cell<T>]> {
        Pin::new(self.as_slice_of_cells())
    }
}

#[cfg(feature = "slice_of_cells")]
impl<'b, 'a: 'b, T: Unpin> IntoPin<&'a [Cell<T>]> for &'a mut Cell<[T]> {
    fn into_pin(self) -> Pin<&'a [Cell<T>]> {
        Pin::new(self.as_slice_of_cells())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////
