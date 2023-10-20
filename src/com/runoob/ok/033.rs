// 033.rs
// https://www.runoob.com/rust/rust-struct.html

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}

/* result:
Rectangle { width: 30, height: 50 }
*/
