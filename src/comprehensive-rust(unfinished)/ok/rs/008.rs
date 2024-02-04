// 008.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/references.html

fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");

    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    let ref_x2: &mut i32 = &mut x;
    *ref_x2 = 21;
    println!("x: {x}");

    // ref_x = &mut x; // cannot assign twice to immutable variable
    let ref_x = &mut x;
    *ref_x = 22;
    println!("x: {x}");

    println!();

    // let mut ref_x: &i32 = 15; // expected `&i32`, found integer
    let mut ref_x: &i32 = &15;
    println!("x: {x}, ref_x: {ref_x}");
    // ref_x = 25; // expected `&i32`, found integer
    ref_x = &25;
    println!("x: {x}, ref_x: {ref_x}");
}

/* result:
x: 10
x: 20
x: 21
x: 22

x: 22, ref_x: 15
x: 22, ref_x: 25
*/
