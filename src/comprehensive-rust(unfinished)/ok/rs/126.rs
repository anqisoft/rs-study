// 126.rs
// https://google.github.io/comprehensive-rust/zh-CN/unsafe/unsafe-traits.html

use std::mem::size_of_val;
use std::slice;

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self))
        }
    }
}

// Safe because u32 has a defined representation and no padding.
unsafe impl AsBytes for u32 {}

fn main() {
    println!("18 as bytes is {:?}", 18.as_bytes());
}

/* result:
18 as bytes is [18, 0, 0, 0]
*/
