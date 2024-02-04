// 108.rs
// https://google.github.io/comprehensive-rust/zh-CN/error-handling/panics.html

fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}

/* result:
thread 'main' panicked at ..\rs\108.rs:6:29:
index out of bounds: the len is 3 but the index is 100
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
