#![feature(pin)]
extern crate pinboard;

use pinboard::AsPin;

use std::marker::Unpin;
use std::pin::PinMut;


fn main() {
    let mut v = vec![1u32, 2, 3, 4, 5];

    // Turn a vec into PinMut<Vec>
    {
        let pin: PinMut<Vec<u32>> = v.as_pin();
    }
    
    // Turn vec into PinMut<[]>
    {
        let pin: PinMut<[u32]> = v.as_pin();
    }
    
    // slice into PinMut<[]>
    {
        let mut array = [100, 200, 300, 400];
        let pin: PinMut<[u32]> = array.as_mut().as_pin();
    }

    // Box into PinMut
    {
        let mut sliced_box: Box<[u32]> = Box::new([1, 2, 3, 4, 5]);

        let pin = sliced_box.as_pin();
    }

}