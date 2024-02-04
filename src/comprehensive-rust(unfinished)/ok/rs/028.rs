// 028.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/type-inference.html

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);

    // expected `u32`, found `i8`
    // arguments to this function are incorrect
    // help: you can convert an `i8` to a `u32` and panic if the converted value doesn't fit
    // takes_u32(y);

    use std::convert::TryInto;
    takes_u32(y.try_into().unwrap());
}

/* result:
u32: 10
i8: 20
u32: 20
*/
