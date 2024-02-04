// 087.rs
// https://google.github.io/comprehensive-rust/zh-CN/generics/data-types.html

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
}

/* result:
Point { x: 5, y: 10 } and Point { x: 1.0, y: 4.0 }
*/

/*
warning: fields `x` and `y` are never read
 --> ..\rs\087.rs:6:5
  |
5 | struct Point<T> {
  |        ----- fields in this struct
6 |     x: T,
  |     ^
7 |     y: T,
  |     ^
  |
  = note: `Point` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/