// 031.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/static-and-const.html

static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
    println!("{BANNER}");
}

/* result:
Welcome to RustOS 3.14
Welcome to RustOS 3.14
*/
