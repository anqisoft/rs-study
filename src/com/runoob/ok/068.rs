// 068.rs
// https://www.runoob.com/rust/rust-collection-string.html

fn main() {
    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);
}

/* result:
[1, 2, 4, 8, 16, 32, 64]
*/
