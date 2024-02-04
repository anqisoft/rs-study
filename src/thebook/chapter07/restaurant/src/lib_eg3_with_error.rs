mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {
            println!("call add_to_waitlist().");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
/*
error[E0603]: module `hosting` is private
  --> src\lib.rs:11:28
   |
11 |     crate::front_of_house::hosting::add_to_waitlist();
   |                            ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                            |
   |                            private module
   |
note: the module `hosting` is defined here
  --> src\lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src\lib.rs:13:21
   |
13 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^  --------------- function `add_to_waitlist` is not publicly re-exported
   |                     |
   |                     private module
   |
note: the module `hosting` is defined here
  --> src\lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
*/