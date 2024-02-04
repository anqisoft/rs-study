// 007.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/compound-types.html

fn main() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);

    // println!("t.0: {t.0}"); // Invalid format string
    // println!("t.1: {t.1}"); // Invalid format string

    println!("t: {:?}", t);
    println!("t: {t:?}");

    println!("t: {:#?}", t);
    println!("t: {t:#?}");
}

/* result:
t.0: 7
t.1: true
t: (7, true)
t: (7, true)
t: (
    7,
    true,
)
t: (
    7,
    true,
)
*/
