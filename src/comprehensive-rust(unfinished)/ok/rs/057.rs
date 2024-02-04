// 057.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/copy-clone.html

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");

    let p3 = p1.clone();
    println!("p3: {p3:?}");
}

/* result:
p1: Point(3, 4)
p2: Point(3, 4)
p3: Point(3, 4)
*/
