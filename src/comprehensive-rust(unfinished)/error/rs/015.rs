// error\015.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/copy-clone.html

#[derive(Clone, Debug)]
struct Point(i32, i32);

fn ok() {
    let p1 = Point(3, 4);
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

fn error() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

fn main() {
    ok();
    error();
}

/* result:
error[E0382]: borrow of moved value: `p1`
  --> ..\rs\015.rs:17:19
   |
15 |     let p1 = Point(3, 4);
   |         -- move occurs because `p1` has type `Point`, which does not implement the `Copy` trait
16 |     let p2 = p1;
   |              -- value moved here
17 |     println!("p1: {p1:?}");
   |                   ^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
   |
16 |     let p2 = p1.clone();
   |                ++++++++

error: aborting due to previous error
*/
