// 041.rs
// https://www.runoob.com/rust/rust-project-management.html

mod nation {
    pub mod government {
        pub fn govern() {
            println!("crate::nation::government::govern");
        }
    }
    pub fn govern() {
        println!("crate::nation::govern as nation_govern");
    }
}

use crate::nation::government::govern;
use crate::nation::govern as nation_govern;

fn main() {
    nation_govern();
    govern();
}

/* result:
crate::nation::govern as nation_govern
crate::nation::government::govern
*/