// 042.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/while-let-expressions.html

fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}

/* result:
x: 10
x: 20
x: 30
*/
