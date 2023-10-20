// 032.rs
// https://www.runoob.com/rust/rust-struct.html

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("{}", rect1.wider(&rect2));
}

/* result:
false
warning: field `height` is never read
 --> ..\032.rs:6:5
  |
4 | struct Rectangle {
  |        --------- field in this struct
5 |     width: u32,
6 |     height: u32,
  |     ^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: method `area` is never used
  --> ..\032.rs:10:8
   |
9  | impl Rectangle {
   | -------------- method in this implementation
10 |     fn area(&self) -> u32 {
   |        ^^^^

warning: 2 warnings emitted
*/
