// 026.rs
// https://www.runoob.com/rust/rust-slice.html

fn main() {
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}

/* result:
1
3
5
*/
