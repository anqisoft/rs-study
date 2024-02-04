// 047.rs
// https://google.github.io/comprehensive-rust/zh-CN/pattern-matching/destructuring-arrays.html

#[rustfmt::skip]
fn main() {
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
    println!();

    for triple in [[0, -2, 3], [1, 4, 5], [9, 100, 200]] {
        println!("Tell me about {triple:?}");
        match triple {
            [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
            [1, ..]   => println!("First is 1 and the rest were ignored"),
            _         => println!("All elements were ignored"),
        }

    }
}

/* result:
Tell me about [0, -2, 3]
First is 0, y = -2, and z = 3

Tell me about [0, -2, 3]
First is 0, y = -2, and z = 3
Tell me about [1, 4, 5]
First is 1 and the rest were ignored
Tell me about [9, 100, 200]
All elements were ignored
*/
