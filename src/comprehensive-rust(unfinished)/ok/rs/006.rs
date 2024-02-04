// 006.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/compound-types.html

fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    // println!("a: {}", a); // `[i8; 10]` does not implement `Display` (required by `{}`) [E0277]
    println!("a: {:?}", a);
    println!("a: {a:?}");
    println!("a: {:#?}", a);
    println!("a: {a:#?}");
}

/* result:
a: [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]
a: [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]
a: [
    42,
    42,
    42,
    42,
    42,
    0,
    42,
    42,
    42,
    42,
]
a: [
    42,
    42,
    42,
    42,
    42,
    0,
    42,
    42,
    42,
    42,
]
*/
