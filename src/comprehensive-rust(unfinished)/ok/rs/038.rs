// 038.rs
// https://google.github.io/comprehensive-rust/zh-CN/enums/sizes.html

use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

fn main() {
    unsafe {
        println!("bool:");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Option<bool>:");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Option<Option<bool>>:");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Option<&i32>:");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
    }
}

/* result:
bool:
- false: 0x0
- true: 0x1
Option<bool>:
- None::<bool>: 0x2
- Some(false): 0x0
- Some(true): 0x1
Option<Option<bool>>:
- Some(Some(false)): 0x0
- Some(Some(true)): 0x1
- Some(None::<bool>): 0x2
- None::<Option<bool>>: 0x3
Option<&i32>:
- None::<&i32>: 0x0
- Some(&0i32): 0x7ff6e025e6c0
*/
