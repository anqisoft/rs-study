// error\010.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/match-expressions.html

#[rustfmt::skip]
fn main() {
    match std::env::args().next() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}

/* result:
error[E0308]: mismatched types
 --> ..\rs\010.rs:7:14
  |
6 |     match std::env::args().next() {
  |           ----------------------- this expression has type `Option<String>`
7 |         Some("cat") => println!("Will do cat things"),
  |              ^^^^^ expected `String`, found `&str`

error[E0308]: mismatched types
 --> ..\rs\010.rs:8:14
  |
6 |     match std::env::args().next() {
  |           ----------------------- this expression has type `Option<String>`
7 |         Some("cat") => println!("Will do cat things"),
8 |         Some("ls")  => println!("Will ls some files"),
  |              ^^^^ expected `String`, found `&str`

error[E0308]: mismatched types
 --> ..\rs\010.rs:9:14
  |
6 |     match std::env::args().next() {
  |           ----------------------- this expression has type `Option<String>`
...
9 |         Some("mv")  => println!("Let's move some files"),
  |              ^^^^ expected `String`, found `&str`

error[E0308]: mismatched types
  --> ..\rs\010.rs:10:14
   |
6  |     match std::env::args().next() {
   |           ----------------------- this expression has type `Option<String>`
...
10 |         Some("rm")  => println!("Uh, dangerous!"),
   |              ^^^^ expected `String`, found `&str`

error: aborting due to 4 previous errors
*/
