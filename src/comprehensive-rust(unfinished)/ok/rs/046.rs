// 046.rs
// https://google.github.io/comprehensive-rust/zh-CN/pattern-matching/destructuring-structs.html

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn test1() {
    println!("\nIn test1()");
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

#[rustfmt::skip]
fn test2() {
    println!("\nIn test2()");
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: _j, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

#[rustfmt::skip]
fn test3() {
    println!("\nIn test3()");
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 3, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}

fn main() {
    test1();
    test2();
    test3();
}

/* result:

In test1()
x.0 = 1, b = 2, y = 3

In test2()
x.0 = 1, b = 2, y = 3

In test3()
x.0 = 1, b = 2, y = 3

*/

/* warning
warning: unreachable pattern
  --> ..\rs\046.rs:27:9
   |
26 |         Foo { y: _j, x: i }   => println!("y = 2, x = {i:?}"),
   |         ------------------- matches any value
27 |         Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
   |         ^^^^^^^^^^^^^ unreachable pattern
   |
   = note: `#[warn(unreachable_patterns)]` on by default

warning: 1 warning emitted
*/