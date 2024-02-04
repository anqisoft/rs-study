// 125.rs
// https://google.github.io/comprehensive-rust/zh-CN/unsafe/extern-functions.html

extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

/* result:
Absolute value of -3 according to C: 3
*/
