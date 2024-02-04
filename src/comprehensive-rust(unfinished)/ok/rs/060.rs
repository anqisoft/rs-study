// 060.rs
// https://google.github.io/comprehensive-rust/zh-CN/ownership/shared-unique-borrows.html
// See: error\rs\016.rs

fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    println!("b: {b}");

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
}

/* result:
b: 10
a: 20
*/