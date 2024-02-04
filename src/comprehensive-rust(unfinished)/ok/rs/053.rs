// 053.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership.html

struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
        println!("y: {}", p.1);
    }

    // not found in this scope
    // println!("y: {}", p.1);
}

/* result:
x: 3
y: 4
*/
