// 067.rs
// https://www.runoob.com/rust/rust-collection-string.html

fn main() {
    let v = vec![1, 2, 4, 8];
    println!("{}", v[1]);

    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);
}

/* result:
2
[1, 2, 4, 8, 16, 32, 64]
*/
