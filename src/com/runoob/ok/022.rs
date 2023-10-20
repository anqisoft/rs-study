// 022.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    // Error seg:
    // let s1 = String::from("run");
    // let s2 = &s1;
    // println!("{}", s2);
    // s2.push_str("oob");
    // println!("{}", s2);

    let mut s1 = String::from("run");
    let s2 = &mut s1;
    s2.push_str("oob");
    println!("{}", s2);
}

/* result:
runoob
*/

/* Error when using line 6 to 10.
error[E0596]: cannot borrow `*s2` as mutable, as it is behind a `&` reference
 --> ..\022.rs:9:5
  |
9 |     s2.push_str("oob");
  |     ^^ `s2` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 |     let s2 = &mut s1;
  |               +++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
*/
