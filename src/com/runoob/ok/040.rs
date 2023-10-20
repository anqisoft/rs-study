// 040.rs
// https://www.runoob.com/rust/rust-project-management.html

mod nation {
    pub mod government {
        pub fn govern() {
            println!("crate::nation::government::govern");
        }
    }
}

use crate::nation::government::govern;

fn main() {
    govern();
}

/* result:
crate::nation::government::govern
*/