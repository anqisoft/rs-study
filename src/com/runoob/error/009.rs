// error/009.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = s1;
    println!("{}", s2);
}

/* result:
warning: unused variable: `s3`
 --> ..\009.rs:7:9
  |
7 |     let s3 = s1;
  |         ^^ help: if this is intentional, prefix it with an underscore: `_s3`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0505]: cannot move out of `s1` because it is borrowed
 --> ..\009.rs:7:14
  |
5 |     let s1 = String::from("hello");
  |         -- binding `s1` declared here
6 |     let s2 = &s1;
  |              --- borrow of `s1` occurs here
7 |     let s3 = s1;
  |              ^^ move out of `s1` occurs here
8 |     println!("{}", s2);
  |                    -- borrow later used here

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0505`.
*/
