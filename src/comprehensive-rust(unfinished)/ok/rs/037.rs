// 037.rs
// https://google.github.io/comprehensive-rust/zh-CN/enums/sizes.html

#[repr(u32)]
enum Bar {
    A,  // 0
    B = 10000,
    C,  // 10001
}

fn main() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
}

/* result:
A: 0
B: 10000
C: 10001
*/
