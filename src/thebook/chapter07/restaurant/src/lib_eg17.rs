mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// error
// mod Customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }
/*
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src\lib.rs:16:9
   |
16 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
15 +     use crate::hosting;
   |

For more information about this error, try `rustc --explain E0433`.
*/

mod customer {
    use crate::hosting;

    pub fn _eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
