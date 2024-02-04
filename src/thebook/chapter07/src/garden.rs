pub mod vegetables;

// Attention: must use "pub mod vegetables;". If use "mod vegetables;", will make mistake:
/*
error[E0603]: module `vegetables` is private
  --> src\main.rs:74:20
   |
74 | use crate::garden::vegetables::Asparagus;
   |                    ^^^^^^^^^^ private module
   |
note: the module `vegetables` is defined here
  --> src\garden.rs:2:1
   |
2  | mod vegetables;
   | ^^^^^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
*/
