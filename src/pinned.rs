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
impl <'a, T: ?Sized, U: ?Sized> AsPin<U> for &'a T
where
    T: AsPin<U>
{
    #[inline]
    fn as_pin(&self) -> Pin<&U> {
        (*self).as_pin()
    }
}

impl <'a, T: ?Sized, U: ?Sized> AsPin<U> for &'a mut T
where
    T: AsPin<U>
{
    #[inline]
    fn as_pin(&self) -> Pin<&U> {
        <T as AsPin<U>>::as_pin(*self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

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
///////////////////////////////////////////////

///////////////////////////////////////////////
// BOX IMPL
///////////////////////////////////////////////
impl<T: ?Sized + Unpin> AsPin<T> for Box<T> {
    #[inline]
    fn as_pin(&self) -> Pin<&T> {
        Pin::new(self.as_ref())
    }
}

impl<T: ?Sized + Unpin> AsPin<Box<T>> for Box<T> {
    #[inline]
    fn as_pin(&self) -> Pin<&Box<T>> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

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
///////////////////////////////////////////////
///////////////////////////////////////////////


/// Converts a mutable reference into a mutable pinned reference.
pub trait AsPinMut<P: ?Sized>: AsPin<P> {
    /// Performs the conversion.
    fn as_pin_mut(&mut self) -> Pin<&mut P>;
}

///////////////////////////////////////////////
// GENERIC IMPL
///////////////////////////////////////////////
impl <'a, T: ?Sized, U: ?Sized> AsPinMut<U> for &'a mut T
where
    T: AsPinMut<U>
{
    #[inline]
    fn as_pin_mut(&mut self) -> Pin<&mut U> {
        <T as AsPinMut<U>>::as_pin_mut(*self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

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
///////////////////////////////////////////////

///////////////////////////////////////////////
// BOX IMPL
///////////////////////////////////////////////
impl<T: ?Sized + Unpin> AsPinMut<T> for Box<T> {
    #[inline]
    fn as_pin_mut(&mut self) -> Pin<&mut T> {
        Pin::new(self.as_mut())
    }
}

impl<T: ?Sized + Unpin> AsPinMut<Box<T>> for Box<T> {
    #[inline]
    fn as_pin_mut(&mut self) -> Pin<&mut Box<T>> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////

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
///////////////////////////////////////////////
///////////////////////////////////////////////

/// Wraps `Self` into a `Pin`.
pub trait IntoPin<T: Unpin> {
    fn into_pin(self) -> Pin<T>;
}

///////////////////////////////////////////////
// GENERIC IMPL
///////////////////////////////////////////////
impl <'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a T
{
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut T
{
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b mut T> for &'a mut T
{
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
impl <T: Unpin> IntoPin<Vec<T>> for Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b, T: Unpin> IntoPin<&'b [T]> for &'a Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b [T]> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b, T: Unpin> IntoPin<&'b [T]> for &'a mut Vec<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b [T]> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b, T: Unpin> IntoPin<&'b mut [T]> for &'a mut Vec<T> {
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

impl <'b, 'a: 'b> IntoPin<&'b str> for &'a String {
    #[inline]
    fn into_pin(self) -> Pin<&'b str> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b> IntoPin<&'b str> for &'a mut String {
    #[inline]
    fn into_pin(self) -> Pin<&'b str> {
        Pin::new(self)
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////


///////////////////////////////////////////////
// BOX IMPL
///////////////////////////////////////////////
impl <T: Unpin + ?Sized> IntoPin<Box<T>> for Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<Self> {
        Pin::new(self)
    }
}

impl <'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self.as_ref())
    }
}

impl <'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b T> for &'a mut Box<T> {
    #[inline]
    fn into_pin(self) -> Pin<&'b T> {
        Pin::new(self.as_mut())
    }
}

impl <'b, 'a: 'b, T: Unpin + ?Sized> IntoPin<&'b mut T> for &'a mut Box<T> {
    fn into_pin(self) -> Pin<&'b mut T> {
        Pin::new(self.as_mut())
    }
}
///////////////////////////////////////////////
///////////////////////////////////////////////
