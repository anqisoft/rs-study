// 005.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/scalar-types.html

fn main() {
    let a = 1_000;
    let b= 1000;
    let c=10_00;
    println!("{a} == {b} is {}, {b} == {c} is {}.", a == b, b == c);
    println!("{0} == {1} is {3}, {1} == {2} is {4}.", a, b, c, a == b, b == c);

    let d = 123_i64;
    let e = 123i64;
    println!("{d} == {e} is {}.", d == e);
}

/* result:
1000 == 1000 is true, 1000 == 1000 is true.
1000 == 1000 is true, 1000 == 1000 is true.
123 == 123 is true.
*/
