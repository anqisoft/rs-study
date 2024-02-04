// 055.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/moves-function-calls.html

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn main() {
    let name = String::from("Alice");
    say_hello(name);
    // say_hello(name);
}

/* result:
Hello Alice
*/
