// 030.rs
// https://www.runoob.com/rust/rust-struct.html

#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

/* result:
rect1 is Rectangle { width: 30, height: 50 }
rect1 is Rectangle {
    width: 30,
    height: 50,
}


P:\anqi\Desktop\tech\rust\projects\rust-study\src\com\runoob\ok\bat>(cd ..\debug   && rustc ..\030.rs   && 030  1>..\debug\030.txt  && start "" notepad ..\debug\030.txt )  || pause
warning: fields `width` and `height` are never read
 --> ..\030.rs:7:5
  |
6 | struct Rectangle {
  |        --------- fields in this struct
7 |     width: u32,
  |     ^^^^^
8 |     height: u32,
  |     ^^^^^^
  |
  = note: `Rectangle` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` on by default
*/
