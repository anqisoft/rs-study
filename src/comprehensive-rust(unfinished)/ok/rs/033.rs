// 033.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/scopes-shadowing.html

fn main() {
    let a = 1;
    let b = &a;
    let a = a + 1;
    println!("a: {a}, b: {b}");
}

/* result:
a: 2, b: 1
*/
