// error/012.rs
// 

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

/* result:
error[E0106]: missing lifetime specifier
 --> ..\012.rs:8:16
  |
8 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
8 | fn dangle() -> &'static String {
  |                 +++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
*/
