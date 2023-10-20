// 075.rs
// https://www.runoob.com/rust/rust-collection-string.html

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, "a");

    if let Some(x) = map.get_mut(&1) {
        *x = "b";
    }
}

/* result: (empty)

*/
