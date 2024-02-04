// 048.rs
// https://google.github.io/comprehensive-rust/zh-CN/pattern-matching/destructuring-arrays.html

fn main() {
    inspect(&[0, -2, 3]);
    inspect(&[0, -2, 3, 4]);
}

#[rustfmt::skip]
fn inspect(slice: &[i32]) {
    println!("Tell me about {slice:?}");
    match slice {
        &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        &[1, ..]   => println!("First is 1 and the rest were ignored"),
        _          => println!("All elements were ignored"),
    }
}

/* result:
Tell me about [0, -2, 3]
First is 0, y = -2, and z = 3
Tell me about [0, -2, 3, 4]
All elements were ignored
*/
