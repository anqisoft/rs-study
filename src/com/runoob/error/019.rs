// error/019.rs
// https://www.runoob.com/rust/rust-project-management.html

mod nation {
    mod government {
        fn govern() {}
    }
    mod congress {
        fn legislate() {}
    }
    mod court {
        fn judicial() {}
    }
}

fn main() {
    nation::government::govern();
    nation::congress::legislate();
    nation::court::judicial();
}

/* result:
error[E0603]: module `government` is private
  --> ..\019.rs:17:13
   |
17 |     nation::government::govern();
   |             ^^^^^^^^^^  ------ function `govern` is not publicly re-exported
   |             |
   |             private module
   |
note: the module `government` is defined here
  --> ..\019.rs:5:5
   |
5  |     mod government {
   |     ^^^^^^^^^^^^^^

error[E0603]: module `congress` is private
  --> ..\019.rs:18:13
   |
18 |     nation::congress::legislate();
   |             ^^^^^^^^  --------- function `legislate` is not publicly re-exported
   |             |
   |             private module
   |
note: the module `congress` is defined here
  --> ..\019.rs:8:5
   |
8  |     mod congress {
   |     ^^^^^^^^^^^^

error[E0603]: module `court` is private
  --> ..\019.rs:19:13
   |
19 |     nation::court::judicial();
   |             ^^^^^  -------- function `judicial` is not publicly re-exported
   |             |
   |             private module
   |
note: the module `court` is defined here
  --> ..\019.rs:11:5
   |
11 |     mod court {
   |     ^^^^^^^^^

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0603`.
*/