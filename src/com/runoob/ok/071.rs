// 071.rs
// https://www.runoob.com/rust/rust-collection-string.html

fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}

/* result:
[150, 82, 107]
*/
