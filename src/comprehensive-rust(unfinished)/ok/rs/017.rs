// 017.rs. This is just the original code, no implementation details, please see 018.rs.
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/for-loops.html

#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    unimplemented!()
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    unimplemented!()
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- 这个注释会让 rustfmt 添加一个新行
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}

/* result:
thread 'main' panicked at ..\rs\017.rs:11:5:
not implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
