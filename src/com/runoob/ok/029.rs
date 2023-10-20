// 029.rs
// https://www.runoob.com/rust/rust-struct.html

fn main() {
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}

/* result:
black = (0, 0, 0)
origin = (0, 0)
*/
