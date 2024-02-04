// 039.rs
// https://google.github.io/comprehensive-rust/zh-CN/enums/sizes.html

#![recursion_limit = "1000"]

use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

// Macro to wrap a value in 2^n Some() where n is the number of "@" signs.
// Increasing the recursion limit is required to evaluate this macro.
macro_rules! many_options {
    ($value:expr) => { Some($value) };
    ($value:expr, @) => {
        Some(Some($value))
    };
    ($value:expr, @ $($more:tt)+) => {
        many_options!(many_options!($value, $($more)+), $($more)+)
    };
}

fn main() {
    // TOTALLY UNSAFE. Rust provides no guarantees about the bitwise
    // representation of types.
    unsafe {
        assert_eq!(many_options!(false), Some(false));
        assert_eq!(many_options!(false, @), Some(Some(false)));
        assert_eq!(many_options!(false, @@), Some(Some(Some(Some(false)))));

        println!("Bitwise representation of a chain of 128 Option's.");
        dbg_bits!(many_options!(false, @@@@@@@), u8);
        dbg_bits!(many_options!(true, @@@@@@@), u8);

        println!("Bitwise representation of a chain of 256 Option's.");
        dbg_bits!(many_options!(false, @@@@@@@@), u16);
        dbg_bits!(many_options!(true, @@@@@@@@), u16);

        println!("Bitwise representation of a chain of 257 Option's.");
        dbg_bits!(many_options!(Some(false), @@@@@@@@), u16);
        dbg_bits!(many_options!(Some(true), @@@@@@@@), u16);
        dbg_bits!(many_options!(None::<bool>, @@@@@@@@), u16);
    }
}

/* result:
Bitwise representation of a chain of 128 Option's.
- many_options!(false, @ @ @ @ @ @ @): 0x0
- many_options!(true, @ @ @ @ @ @ @): 0x1
Bitwise representation of a chain of 256 Option's.
- many_options!(false, @ @ @ @ @ @ @ @): 0x1
- many_options!(true, @ @ @ @ @ @ @ @): 0x101
Bitwise representation of a chain of 257 Option's.
- many_options!(Some(false), @ @ @ @ @ @ @ @): 0x1
- many_options!(Some(true), @ @ @ @ @ @ @ @): 0x101
- many_options!(None :: < bool >, @ @ @ @ @ @ @ @): 0x201
*/
