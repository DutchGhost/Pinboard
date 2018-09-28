# pinpoint

This crate provides the `IntoPin` trait. `IntoPin` is powerfull for creating coerced, pinned references.

### Example

```Rust
#![feature(pin)]

extern crate pinpoint;

use std::pin::Pin;
use pinpoint::IntoPin;

fn example<'a, P>(item: P)
where
    P: IntoPin<&'a mut [u8]>
{
    let mut pin: Pin<&mut [u8]> = item.into_pin();
    pin.reverse();
}

let mut v = vec![1, 2, 3, 4];
example(&mut v);
assert_eq!(v, [4, 3, 2, 1]);

let mut b: Box<[u8]> = Box::new([4, 3, 2, 1]);
example(&mut b);
assert_eq!(*b, [1, 2, 3, 4]);
```