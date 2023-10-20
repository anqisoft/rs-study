// 047.rs
// https://www.runoob.com/rust/rust-error-handle.html

fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

fn g(i: i32) -> Result<i32, bool> {
    let t = f(i);
    return match t {
        Ok(i) => Ok(i),
        Err(b) => Err(b)
    };
}

fn main() {
    let r = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}

/* result:
Ok: f(-1) = 10000
*/

/* warnings:
warning: function `g` is never used
 --> ..\047.rs:9:4
  |
9 | fn g(i: i32) -> Result<i32, bool> {
  |    ^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/
