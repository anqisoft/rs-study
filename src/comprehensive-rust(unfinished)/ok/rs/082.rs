// 082.rs
// https://google.github.io/comprehensive-rust/zh-CN/modules/visibility.html

mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    // mod inner { =>
    pub mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn main() {
    outer::public();

    outer::inner::public();

    // error[E0603]: module `inner` is private --> ..\rs\082.rs:28:12
    // function `public` is not publicly re-exported
    // private module
    // note: the module `inner` is defined here --> ..\rs\082.rs:13:5
    // outer::inner::public();

    // error[E0603]: function `private` is private --> ..\rs\082.rs:30:12
    // private function
    // note: the function `private` is defined here --> ..\rs\082.rs:5:5
    // outer::private();

    // error[E0603]: module `inner` is private --> ..\rs\082.rs:31:12
    // function `private` is not publicly re-exported
    // note: the module `inner` is defined here --> ..\rs\082.rs:13:5
    // outer::inner::private();
}

/* result:
outer::public
outer::inner::public
outer::private
*/


/*
warning: function `private` is never used
  --> ..\rs\082.rs:15:12
   |
15 |         fn private() {
   |            ^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/