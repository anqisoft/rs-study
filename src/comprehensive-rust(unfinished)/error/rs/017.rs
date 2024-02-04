// error\017.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/lifetimes-function-calls.html

#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p3: &Point;
    {
        let p2: Point = Point(20, 20);
        p3 = left_most(&p1, &p2);
    }
    println!("p3: {p3:?}");
}

/* result:
error[E0597]: `p2` does not live long enough
  --> ..\rs\017.rs:16:29
   |
15 |         let p2: Point = Point(20, 20);
   |             -- binding `p2` declared here
16 |         p3 = left_most(&p1, &p2);
   |                             ^^^ borrowed value does not live long enough
17 |     }
   |     - `p2` dropped here while still borrowed
18 |     println!("p3: {p3:?}");
   |                   ------ borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
*/
