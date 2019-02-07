use std::borrow::Cow;
use std::cell::{Cell, Ref, RefMut};
use std::ffi::{OsStr, OsString};
use std::marker::Unpin;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

/// Used for pinning pointer/reference types.
/// This can also be used to coerce from one pointer type to the pinned version of the other, for example `&str` to `Pin<&[u8]>`.
/// # Examples
/// ```
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

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short Pin<&'long T> {
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
// VALUE TO SMARTPTR
///////////////////////////////////////////////
impl<T> IntoPin<Box<T>> for T {
    #[inline]
    fn into_pin(self) -> Pin<Box<T>> {
        Box::pin(self)
    }
}

impl<T> IntoPin<Arc<T>> for T {
    #[inline]
    fn into_pin(self) -> Pin<Arc<T>> {
        Arc::pin(self)
    }
}

impl<T> IntoPin<Rc<T>> for T {
    #[inline]
    fn into_pin(self) -> Pin<Rc<T>> {
        Rc::pin(self)
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
// STR IMPL
///////////////////////////////////////////////
impl<'a> IntoPin<&'a [u8]> for &'a str {
    fn into_pin(self) -> Pin<&'a [u8]> {
        Pin::new(self.as_bytes())
    }
}

impl<'a> IntoPin<&'a [u8]> for &'a mut str {
    fn into_pin(self) -> Pin<&'a [u8]> {
        Pin::new(self.as_bytes())
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a str {
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self.as_ref())
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a mut str {
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new((&*self).as_ref())
    }
}

impl<'a> IntoPin<&'a Path> for &'a str {
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new(self.as_ref())
    }
}

impl<'a> IntoPin<&'a Path> for &'a mut str {
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new((&*self).as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// PATHBUF IMPL
///////////////////////////////////////////////
impl IntoPin<PathBuf> for PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl IntoPin<Box<Path>> for PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<Box<Path>> {
        Pin::new(self.into_boxed_path())
    }
}

impl IntoPin<OsString> for PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<OsString> {
        Pin::new(self.into_os_string())
    }
}

impl<'a> IntoPin<&'a Path> for &'a PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a Path> for &'a mut PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self.as_os_str())
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a mut PathBuf {
    #[inline]
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self.as_os_str())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// PATH IMPL
///////////////////////////////////////////////
impl<'a> IntoPin<&'a OsStr> for &'a Path {
    #[inline]
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self.as_os_str())
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a mut Path {
    #[inline]
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self.as_os_str())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// OSSTRING IMPL
///////////////////////////////////////////////
impl IntoPin<OsString> for OsString {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl IntoPin<Box<OsStr>> for OsString {
    #[inline]
    fn into_pin(self) -> Pin<Box<OsStr>> {
        Pin::new(self.into_boxed_os_str())
    }
}

//@NOTE: I think this does not alloc?
impl IntoPin<PathBuf> for OsString {
    #[inline]
    fn into_pin(self) -> Pin<PathBuf> {
        Pin::new(self.into())
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a OsString {
    #[inline]
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a OsStr> for &'a mut OsString {
    #[inline]
    fn into_pin(self) -> Pin<&'a OsStr> {
        Pin::new(self)
    }
}

impl<'a> IntoPin<&'a Path> for &'a OsString {
    #[inline]
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new(self.as_ref())
    }
}

impl<'a> IntoPin<&'a Path> for &'a mut OsString {
    #[inline]
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new((&*self).as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// OSSTR IMPL
///////////////////////////////////////////////
impl<'a> IntoPin<&'a Path> for &'a OsStr {
    #[inline]
    fn into_pin(self) -> Pin<&'a Path> {
        Pin::new(self.as_ref())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

///////////////////////////////////////////////
// BOX IMPL
///////////////////////////////////////////////
impl<T: ?Sized> IntoPin<Box<T>> for Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        self.into()
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

impl IntoPin<Box<[u8]>> for Box<str> {
    #[inline]
    fn into_pin(self) -> Pin<Box<[u8]>> {
        Pin::new(str::into_boxed_bytes(self))
    }
}

impl<T: Unpin> IntoPin<Vec<T>> for Box<[T]> {
    #[inline]
    fn into_pin(self) -> Pin<Vec<T>> {
        Pin::new(<[T]>::into_vec(self))
    }
}

impl IntoPin<OsString> for Box<OsStr> {
    #[inline]
    fn into_pin(self) -> Pin<OsString> {
        Pin::new(OsStr::into_os_string(self))
    }
}

impl IntoPin<PathBuf> for Box<Path> {
    #[inline]
    fn into_pin(self) -> Pin<PathBuf> {
        Pin::new(Path::into_path_buf(self))
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

impl<'short, 'long> IntoPin<&'short [u8]> for &'short Cow<'long, str> {
    #[inline]
    fn into_pin(self) -> Pin<&'short [u8]> {
        // Asref into &str, then Asref into &[u8].
        Pin::new(self.as_ref().as_ref())
    }
}

impl<'short, 'long> IntoPin<&'short [u8]> for &'short mut Cow<'long, str> {
    #[inline]
    fn into_pin(self) -> Pin<&'short [u8]> {
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

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short Arc<&'long T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self.as_ref())
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short Arc<&'long mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self.as_ref())
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut Arc<&'long mut T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(&**self)
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

impl<'a, T: Unpin + ?Sized> IntoPin<&'a T> for Rc<&'a T> {
    #[inline]
    fn into_pin(self) -> Pin<&'a T> {
        Pin::new(self.as_ref())
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short Rc<&'long T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self.as_ref())
    }
}

impl<'short, 'long, T: Unpin + ?Sized> IntoPin<&'short T> for &'short mut Rc<&'long T> {
    #[inline]
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self.as_ref())
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

impl<'short, 'long, T: Unpin> IntoPin<&'short T> for &'short mut Cell<&'long T> {
    fn into_pin(self) -> Pin<&'short T> {
        Pin::new(self.get_mut())
    }
}

impl<'short, 'long, T: Unpin> IntoPin<&'short mut T> for &'short mut Cell<&'long mut T> {
    fn into_pin(self) -> Pin<&'short mut T> {
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

macro_rules! impl_array {
    ($size:expr $(,$sizes:expr)*) => (
        impl <'a, T: Unpin> IntoPin<&'a [T]> for &'a [T; $size] {
            #[inline]
            fn into_pin(self) -> Pin<&'a [T]> {
                Pin::new(self)
            }
        }

        impl <'a, T: Unpin> IntoPin<&'a [T]> for &'a mut [T; $size] {
            #[inline]
            fn into_pin(self) -> Pin<&'a [T]> {
                Pin::new(self)
            }
        }

        impl <'a, T: Unpin> IntoPin<&'a mut [T]> for &'a mut [T; $size] {
            #[inline]
            fn into_pin(self) -> Pin<&'a mut [T]> {
                Pin::new(self)
            }
        }

        impl_array!($($sizes),*);
    );

    () => {}
}

impl_array!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);
