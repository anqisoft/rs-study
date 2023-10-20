// 069.rs
// https://www.runoob.com/rust/rust-collection-string.html

fn main() {
    let mut v = vec![1, 2, 4, 8];
    println!("{}", match v.get(0) {
        Some(value) => value.to_string(),
        None => "None".to_string()
    });
}

/* result:
1
*/

/* warnings:
warning: variable does not need to be mutable
 --> ..\069.rs:5:9
  |
5 |     let mut v = vec![1, 2, 4, 8];
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

warning: 1 warning emitted
*/
