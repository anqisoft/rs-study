// error\009.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/for-expressions.html

fn main() {
    let v = vec![10, 20, 30];
    v[1] = 21;

    for x in v {
        println!("x: {x}");
    }

    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}

/* result:
error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
 --> ..\rs\009.rs:6:5
  |
6 |     v[1] = 21;
  |     ^ cannot borrow as mutable
  |
help: consider changing this to be mutable
  |
5 |     let mut v = vec![10, 20, 30];
  |         +++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
*/
