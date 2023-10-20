// error/023.rs
// https://www.runoob.com/rust/rust-lifetime.html

fn longer(s1: &str, s2: &str) -> &str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}


fn main() {
    let r;
    {
        let s1 = "rust";
        let s2 = "ecmascript";
        r = longer(s1, s2);
    }
    println!("{} is longer", r);
}

/* result:
error[E0106]: missing lifetime specifier
 --> ..\023.rs:4:34
  |
4 | fn longer(s1: &str, s2: &str) -> &str {
  |               ----      ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
help: consider introducing a named lifetime parameter
  |
4 | fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  |          ++++      ++           ++          ++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0106`.
*/
