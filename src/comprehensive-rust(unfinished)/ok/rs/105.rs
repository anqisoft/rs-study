// 105.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/closures.html

fn make_greeter(prefix: String) -> impl Fn(&str) {
    return move |name| println!("{} {}", prefix, name)
}

fn main() {
    let hi = make_greeter("Hi".to_string());
    hi("there");
}

/* result:
Hi there
*/
