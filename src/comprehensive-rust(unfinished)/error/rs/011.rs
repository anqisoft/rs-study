// error\011.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership.html

struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    println!("y: {}", p.1);
}

/* result:
error[E0425]: cannot find value `p` in this scope
  --> ..\rs\011.rs:11:23
   |
11 |     println!("y: {}", p.1);
   |                       ^ not found in this scope

error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
*/
