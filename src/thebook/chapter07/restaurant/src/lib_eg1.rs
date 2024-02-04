mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
/*
warning: function `add_to_waitlist` is never used
 --> src\lib.rs:3:12
  |
3 |         fn add_to_waitlist() {}
  |            ^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: function `seat_at_table` is never used
 --> src\lib.rs:4:12
  |
4 |         fn seat_at_table() {}
  |            ^^^^^^^^^^^^^

warning: function `take_order` is never used
 --> src\lib.rs:8:12
  |
8 |         fn take_order() {}
  |            ^^^^^^^^^^

warning: function `serve_order` is never used
 --> src\lib.rs:9:12
  |
9 |         fn serve_order() {}
  |            ^^^^^^^^^^^

warning: function `take_payment` is never used
  --> src\lib.rs:10:12
   |
10 |         fn take_payment() {}
   |            ^^^^^^^^^^^^

warning: `restaurant` (lib) generated 5 warnings
*/