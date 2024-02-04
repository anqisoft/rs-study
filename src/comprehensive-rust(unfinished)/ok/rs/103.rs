// 103.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/operators.html

#[derive(Debug, Copy, Clone)]
struct Point { x: i32, y: i32 }

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}

/* result:
Point { x: 10, y: 20 } + Point { x: 100, y: 200 } = Point { x: 110, y: 220 }
*/
