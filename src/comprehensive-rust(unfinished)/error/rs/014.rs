// error\014.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/copy-clone.html

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32, String);

fn ok() {
    let p1 = Point(3, 4, "test");
    let p2 = p1.clone();
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

fn error() {
    let p1 = Point(3, 4, "test");
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}

fn main() {
    ok();
    error();
}

/* result:
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> ..\rs\014.rs:6:10
  |
6 | #[derive(Copy, Clone, Debug)]
  |          ^^^^
7 | struct Point(i32, i32, String);
  |                        ------ this field does not implement `Copy`
  |
  = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0204`.
*/
