// 056.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/copy-clone.html

fn main() {
    let x = 42;
    let y = x;
    println!("x: {x}");
    println!("y: {y}");
}

/* result:
x: 42
y: 42
*/
