// 029.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/type-inference.html

fn main() {
    let mut v = Vec::new();
    v.push((10, false));
    v.push((20, true));
    println!("v: {v:?}");

    let vv = v.iter().collect::<std::collections::HashSet<_>>();
    println!("vv: {vv:?}");
}

/* result:
v: [(10, false), (20, true)]
vv: {(20, true), (10, false)}
*/
