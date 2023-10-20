// error/006.rs
// https://www.runoob.com/rust/rust-basic-syntax.html


fn main() {
  let mut s = "123";
  s = s.len();
  println!("s is {}", s);
}

/* result:
error[E0308]: mismatched types
 --> P:\anqi\Desktop\tech\rust\projects\rust-study\src\com\runoob\003_error6.rs:6:7
  |
5 |   let mut s = "123";
  |               ----- expected due to this value
6 |   s = s.len();
  |       ^^^^^^^ expected `&str`, found `usize`
  |
help: try removing the method call
  |
6 -   s = s.len();
6 +   s = s;
  |

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
*/
