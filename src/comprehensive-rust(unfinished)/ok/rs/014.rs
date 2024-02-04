// 014.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/functions-interlude.html

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
    println!("coin toss: {}", pick_one("heads", "tails"));
    println!("cash prize: {}", pick_one(500, 1000));
}

/* result:
coin toss: heads
cash prize: 500
*/
