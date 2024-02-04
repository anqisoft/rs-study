// error\002.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/references-dangling.html

fn main() {
    let ref_x: &i32;
    {
        let x: i32 = 10;
        ref_x = &x;
    }
    println!("ref_x: {ref_x}");
}

/* result:
error[E0597]: `x` does not live long enough
  --> ..\rs\002.rs:8:17
   |
7  |         let x: i32 = 10;
   |             - binding `x` declared here
8  |         ref_x = &x;
   |                 ^^ borrowed value does not live long enough
9  |     }
   |     - `x` dropped here while still borrowed
10 |     println!("ref_x: {ref_x}");
   |                      ------- borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
*/
