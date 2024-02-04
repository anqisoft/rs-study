// 076.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/box.html

fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
    println!("five: {}", five);
}

/* result:
five: 5
five: 5
*/
