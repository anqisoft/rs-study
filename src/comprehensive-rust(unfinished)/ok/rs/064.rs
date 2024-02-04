// 064.rs
// https://google.github.io/comprehensive-rust/zh-CN/structs/tuple-structs.html

struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}

/* result:
(17, 23)
*/
