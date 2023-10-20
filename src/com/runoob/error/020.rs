// error/020.rs
// https://www.runoob.com/rust/rust-project-management.html

mod nation {
    pub mod government {
        pub fn govern() {
            println!("nation::government::govern()");
        }
    }
    pub use government::govern;
}

fn main() {
    nation::govern();
}

/* result:
error[E0432]: unresolved import `government`
  --> ..\020.rs:10:13
   |
10 |     pub use government::govern;
   |             ^^^^^^^^^^ help: a similar path exists: `self::government`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0432`.
*/
