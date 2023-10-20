// error/011.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

/* result:
error[E0499]: cannot borrow `s` as mutable more than once at a time
  --> ..\011.rs:8:14
   |
7  |     let r1 = &mut s;
   |              ------ first mutable borrow occurs here
8  |     let r2 = &mut s;
   |              ^^^^^^ second mutable borrow occurs here
9  |
10 |     println!("{}, {}", r1, r2);
   |                        -- first borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0499`.
*/
