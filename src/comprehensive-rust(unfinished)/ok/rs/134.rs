// 134.rs
// https://google.github.io/comprehensive-rust/zh-CN/concurrency/shared_state/example.html

// use std::thread;

// fn test1() {
//     let v = vec![10, 20, 30];
//     let handle = thread::spawn(|| {
//         v.push(10);
//     });
//     v.push(1000);

//     handle.join().unwrap();
//     println!("v: {v:?}");
// }

// fn test2() {
//     let mut v = vec![10, 20, 30];
//     let handle = thread::spawn(move || {
//         v.push(10);
//     });
//     v.push(1000);

//     handle.join().unwrap();
//     println!("v: {v:?}");
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let v = Arc::new(Mutex::new(vec![10, 20, 30]));

    let v2 = Arc::clone(&v);
    let handle = thread::spawn(move || {
        let mut v2 = v2.lock().unwrap();
        v2.push(10);
    });

    {
        let mut v = v.lock().unwrap();
        v.push(1000);
    }

    handle.join().unwrap();

    println!("v: {v:?}");
}

/* result:
v: Mutex { data: [10, 20, 30, 1000, 10], poisoned: false, .. }
*/

/* test1
error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
  --> ..\rs\134.rs:10:9
   |
8  |     let v = vec![10, 20, 30];
   |         - help: consider changing this to be mutable: `mut v`
9  |     let handle = thread::spawn(|| {
10 |         v.push(10);
   |         ^ cannot borrow as mutable

error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
  --> ..\rs\134.rs:9:32
   |
9  |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `v`
10 |         v.push(10);
   |         - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> ..\rs\134.rs:9:18
   |
9  |       let handle = thread::spawn(|| {
   |  __________________^
10 | |         v.push(10);
11 | |     });
   | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
9  |     let handle = thread::spawn(move || {
   |                                ++++

error[E0501]: cannot borrow `v` as mutable because previous closure requires unique access
  --> ..\rs\134.rs:12:5
   |
9  |       let handle = thread::spawn(|| {
   |                    -             -- closure construction occurs here
   |  __________________|
   | |
10 | |         v.push(10);
   | |         - first borrow occurs due to use of `v` in closure
11 | |     });
   | |______- argument requires that `v` is borrowed for `'static`
12 |       v.push(1000);
   |       ^ second borrow occurs here

error[E0596]: cannot borrow `v` as mutable, as it is not declared as mutable
  --> ..\rs\134.rs:12:5
   |
12 |     v.push(1000);
   |     ^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
8  |     let mut v = vec![10, 20, 30];
   |         +++

error[E0501]: cannot borrow `v` as immutable because previous closure requires unique access
  --> ..\rs\134.rs:15:18
   |
9  |       let handle = thread::spawn(|| {
   |                    -             -- closure construction occurs here
   |  __________________|
   | |
10 | |         v.push(10);
   | |         - first borrow occurs due to use of `v` in closure
11 | |     });
   | |______- argument requires that `v` is borrowed for `'static`
...
15 |       println!("v: {v:?}");
   |                    ^^^^^ second borrow occurs here
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0373, E0501, E0596.
For more information about an error, try `rustc --explain E0373`.
*/

/* test2
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
  --> ..\rs\134.rs:20:32
   |
20 |     let handle = thread::spawn(|| {
   |                                ^^ may outlive borrowed value `v`
21 |         v.push(10);
   |         - `v` is borrowed here
   |
note: function requires argument type to outlive `'static`
  --> ..\rs\134.rs:20:18
   |
20 |       let handle = thread::spawn(|| {
   |  __________________^
21 | |         v.push(10);
22 | |     });
   | |______^
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
   |
20 |     let handle = thread::spawn(move || {
   |                                ++++

error[E0499]: cannot borrow `v` as mutable more than once at a time
  --> ..\rs\134.rs:23:5
   |
20 |       let handle = thread::spawn(|| {
   |                    -             -- first mutable borrow occurs here
   |  __________________|
   | |
21 | |         v.push(10);
   | |         - first borrow occurs due to use of `v` in closure
22 | |     });
   | |______- argument requires that `v` is borrowed for `'static`
23 |       v.push(1000);
   |       ^ second mutable borrow occurs here

error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable
  --> ..\rs\134.rs:26:18
   |
20 |       let handle = thread::spawn(|| {
   |                    -             -- mutable borrow occurs here
   |  __________________|
   | |
21 | |         v.push(10);
   | |         - first borrow occurs due to use of `v` in closure
22 | |     });
   | |______- argument requires that `v` is borrowed for `'static`
...
26 |       println!("v: {v:?}");
   |                    ^^^^^ immutable borrow occurs here
   |
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0373, E0499, E0502.
For more information about an error, try `rustc --explain E0373`.
*/