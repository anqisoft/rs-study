// 013.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/methods.html

// use std::net::Shutdown::Read; // I don't know where I copied it from.

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(width: u32) -> Rectangle {
        Rectangle { width, height: width }
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {} = {} * {}", rect.area(), rect.width, rect.height);
    rect.inc_width(5);
    println!("new area: {} = {} * {}", rect.area(), rect.width, rect.height);

    println!();
    let mut rect2 = Rectangle::new(20, 10);
    println!("old area of rect2: {} = {} * {}", rect2.area(), rect2.width, rect2.height);
    rect2.inc_width(5);
    println!("new area of rect2: {} = {} * {}", rect2.area(), rect2.width, rect2.height);

    println!();
    let mut rect3 = Rectangle::square(30);
    println!("old area of rect3: {} = {} * {}", rect3.area(), rect3.width, rect3.height);
    rect3.inc_width(5);
    println!("new area of rect3: {} = {} * {}", rect3.area(), rect3.width, rect3.height);
    rect3.height += 5;
    println!("last area of rect3: {} = {} * {}", rect3.area(), rect3.width, rect3.height);
}

/* result:
old area: 50 = 10 * 5
new area: 75 = 15 * 5

old area of rect2: 200 = 20 * 10
new area of rect2: 250 = 25 * 10

old area of rect3: 900 = 30 * 30
new area of rect3: 1050 = 35 * 30
last area of rect3: 1225 = 35 * 35
*/
