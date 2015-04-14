#![feature(custom_attribute)]
#![feature(no_std)]
#![feature(core)]
#![no_std]
#![allow(ctypes)]
//extern crate core;
extern crate core;
use core::*;
//use core::prelude::*;

//enum Color {
    static Black : u16      = 0;
    static Blue : u16      = 1;
    static Green : u16      = 2;
    static Cyan : u16       = 3;
    static Red : u16        = 4;
    static Pink : u16       = 5;
    static Brown : u16      = 6;
    static LightGray : u16  = 7;
    static DarkGray : u16   = 8;
    static LightBlue : u16  = 9;
    static LightGreen : u16 = 10;
    static LightCyan : u16  = 11;
    static LightRed  : u16  = 12;
    static LightPink : u16  = 13;
    static Yellow : u16     = 14;
    static White : u16      = 15;
//}

enum Option<T> {
    None,
    Some(T)
}

struct IntRange {
    cur: i32,
    max: i32
}

impl IntRange {
    fn next(&mut self) -> Option<i32> {
        if self.cur < self.max {
            self.cur += 1;
            Option::Some(self.cur - 1)
        } else {
            Option::None
        }
    }
}

fn range(lo: i32, hi: i32) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: u16) {
    //for i in range(0, 80 * 25) {
    for i in 0..80*25 {
        unsafe {
            *((0xb8000 + i * 2) as *mut u16) = background << 12;
        }
    }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    clear_screen(LightRed);
}
