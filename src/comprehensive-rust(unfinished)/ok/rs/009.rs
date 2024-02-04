// 009.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/slices.html

fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");

    println!("{:?}: a[0..a.len()]", &a[0..a.len()]);
    println!("{:?}: a[..a.len()]", &a[..a.len()]);
    println!("{:?}: a[0..]", &a[0..]);

    let _destory = s;
    a[3] = 41;
    println!("a: {a:?}");
    // println!("s: {s:?}");

    // test1();
    // test2();
    // test3();
}

// fn test1() {
//     let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     let s: &[i32] = &a[2..4];
//
//     a[3] = 41;
//     println!("a: {a:?}");
//     println!("s: {s:?}");
// }

// fn test2() {
//     let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     let s: &[i32] = &a[2..4];
//
//     let _destory = s;
//     a[3] = 41;
//     println!("a: {a:?}");
//     println!("s: {s:?}");
// }

// fn test3() {
//     let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     let s: &[i32] = &a[2..4];
//
//     let _destory = s;
//     a[3] = 41;
//     println!("a: {a:?}");
//     println!("s: {s:?}");
// }

/* result:
a: [10, 20, 30, 40, 50, 60]
s: [30, 40]
[10, 20, 30, 40, 50, 60]: a[0..a.len()]
[10, 20, 30, 40, 50, 60]: a[..a.len()]
[10, 20, 30, 40, 50, 60]: a[0..]
a: [10, 20, 30, 41, 50, 60]
*/

/* test1
error[E0506]: cannot assign to `a[_]` because it is borrowed
  --> ..\rs\009.rs:29:5
   |
27 |     let s: &[i32] = &a[2..4];
   |                      - `a[_]` is borrowed here
28 |
29 |     a[3] = 41;
   |     ^^^^^^^^^ `a[_]` is assigned to here but it was already borrowed
30 |     println!("a: {a:?}");
31 |     println!("s: {s:?}");
   |                  ----- borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
*/

/* test2
error[E0506]: cannot assign to `a[_]` because it is borrowed
  --> ..\rs\009.rs:39:5
   |
36 |     let s: &[i32] = &a[2..4];
   |                      - `a[_]` is borrowed here
...
39 |     a[3] = 41;
   |     ^^^^^^^^^ `a[_]` is assigned to here but it was already borrowed
40 |     println!("a: {a:?}");
41 |     println!("s: {s:?}");
   |                  ----- borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
*/

/* test3
error[E0506]: cannot assign to `a[_]` because it is borrowed
  --> ..\rs\009.rs:49:5
   |
46 |     let s: &[i32] = &a[2..4];
   |                      - `a[_]` is borrowed here
...
49 |     a[3] = 41;
   |     ^^^^^^^^^ `a[_]` is assigned to here but it was already borrowed
50 |     println!("a: {a:?}");
51 |     println!("s: {s:?}");
   |                  ----- borrow later used here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0506`.
 */