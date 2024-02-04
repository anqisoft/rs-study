// 021.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/blocks.html

fn double(x: i32) -> i32 {
    return x + x;
    x + x;
    x + x
}

fn main() {
    println!("double: {}", double(7));
    println!("double2: {}", double2(7));
}

fn double2(x: i32) -> i32 {
    x + x + 1;
    x + x
}

/* result:
double: 14
double2: 14
*/

/* warnings
warning: unreachable statement
 --> ..\rs\021.rs:6:5
  |
5 |     return x + x;
  |     ------------ any code following this expression is unreachable
6 |     x + x;
  |     ^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default

warning: unused arithmetic operation that must be used
 --> ..\rs\021.rs:6:5
  |
6 |     x + x;
  |     ^^^^^ the arithmetic operation produces a value
  |
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
6 |     let _ = x + x;
  |     +++++++

warning: unused arithmetic operation that must be used
  --> ..\rs\021.rs:16:5
   |
16 |     x + x + 1;
   |     ^^^^^^^^^ the arithmetic operation produces a value
   |
help: use `let _ = ...` to ignore the resulting value
   |
16 |     let _ = x + x + 1;
   |     +++++++

warning: 3 warnings emitted
 */