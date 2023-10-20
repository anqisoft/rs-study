// 070.rs
// https://www.runoob.com/rust/rust-collection-string.html

fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

/* result:
100
32
57
*/
