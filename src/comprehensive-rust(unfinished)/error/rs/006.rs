// error\006.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/for-loops.html

fn pretty_print(matrix: &[&[i32]]) {
    for i in matrix {
        print!("[");
        let max_index = i.len() - 1;
        for j in 0..=max_index {
            print!("{}{}", i[j], if j == max_index { "" } else { ", "});
        }
        println!("]");
    }
}

fn main() {
    let matrix2 = &[
        &[101 as i32, 102 as i32], // <-- 这个注释会让 rustfmt 添加一个新行
        &[201 as i32, 202 as i32],
    ];
    println!("matrix2:");
    pretty_print(matrix2);
    println!();
}

/* result:
error[E0308]: mismatched types
  --> ..\rs\006.rs:21:18
   |
21 |     pretty_print(matrix2);
   |     ------------ ^^^^^^^ expected `&[&[i32]]`, found `&[&[i32; 2]; 2]`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected reference `&[&[i32]]`
              found reference `&[&[i32; 2]; 2]`
note: function defined here
  --> ..\rs\006.rs:4:4
   |
4  | fn pretty_print(matrix: &[&[i32]]) {
   |    ^^^^^^^^^^^^ -----------------

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
*/
