// error\018.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/lifetimes-function-calls.html

#[derive(Debug)]
struct Point(i32, i32);

// fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
fn left_most<'a, 'b>(p1: &'a Point, p2: &'a Point) -> &'b Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point = left_most(&p1, &p2);
    println!("p3: {p3:?}");
}

/* result:
error: lifetime may not live long enough
 --> ..\rs\018.rs:9:22
  |
8 | fn left_most<'a, 'b>(p1: &'a Point, p2: &'a Point) -> &'b Point {
  |              --  -- lifetime `'b` defined here
  |              |
  |              lifetime `'a` defined here
9 |     if p1.0 < p2.0 { p1 } else { p2 }
  |                      ^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
  |
  = help: consider adding the following bound: `'a: 'b`

error: aborting due to previous error
*/
