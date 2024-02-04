// 095.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/impl-trait.html

use std::fmt::Display;

fn get_x(name: impl Display) -> impl Display {
    format!("Hello {name}")
}

fn main() {
    let x = get_x("foo");
    println!("{x}");
}

/* result:
Hello foo
*/
