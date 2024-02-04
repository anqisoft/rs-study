// 088.rs
// https://google.github.io/comprehensive-rust/zh-CN/generics/methods.html

#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0  // + 10
    }

    // fn set_x(&mut self, x: T)
}

fn main() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());
}

/* result:
p.x = 5
*/
