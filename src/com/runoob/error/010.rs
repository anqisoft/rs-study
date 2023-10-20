// error/010.rs
// 
fn main() {
    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    s2.push_str("oob");
    println!("{}", s2);
}

/* result:
error[E0596]: cannot borrow `*s2` as mutable, as it is behind a `&` reference
 --> ..\010.rs:7:5
  |
7 |     s2.push_str("oob");
  |     ^^ `s2` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
5 |     let s2 = &mut s1;
  |               +++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
*/
