// 059.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/borrowing.html

#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    let p = Point(p1.0 + p2.0, p1.1 + p2.1);
    println!("&p.0: {:p}", &p.0);
    p
}

pub fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("&p3.0: {:p}", &p3.0);
    println!("{p1:?} + {p2:?} = {p3:?}");
}

/* result:
&p.0: 0xb37d8ff588
&p3.0: 0xb37d8ff648
Point(3, 4) + Point(10, 20) = Point(13, 24)
*/
