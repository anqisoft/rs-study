// 074.rs
// https://www.runoob.com/rust/rust-collection-string.html

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert("color", "red");
    map.insert("size", "10 m^2");

    for p in map.iter() {
        println!("{:?}", p);
    }
}

/* result:
("size", "10 m^2")
("color", "red")
*/
