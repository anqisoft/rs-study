// 025.rs
// https://www.runoob.com/rust/rust-slice.html

fn main() {
    let mut s = String::from("runoob");
    let slice = &s[0..3];
    // s.push_str("yes!"); // Error
    println!("slice = {}", slice);

    let s1 = String::from("hello");
    let s2 = &s1[..];
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);
}

/* result:
slice = run
s1 = hello
s2 = hello

warning: variable does not need to be mutable
 --> ..\025.rs:5:9
  |
5 |     let mut s = String::from("runoob");
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
*/
