mod front_of_house {
    pub mod hosting {
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
error[E0603]: function `add_to_waitlist` is private
  --> src\lib.rs:11:37
   |
11 |     crate::front_of_house::hosting::add_to_waitlist();
   |                                     ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src\lib.rs:3:9
   |
3  |         fn add_to_waitlist() {
   |         ^^^^^^^^^^^^^^^^^^^^

error[E0603]: function `add_to_waitlist` is private
  --> src\lib.rs:13:30
   |
13 |     front_of_house::hosting::add_to_waitlist();
   |                              ^^^^^^^^^^^^^^^ private function
   |
note: the function `add_to_waitlist` is defined here
  --> src\lib.rs:3:9
   |
3  |         fn add_to_waitlist() {
   |         ^^^^^^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
*/