// 019.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/for-loops.html
// TODO fix the bugs.

// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     let mut result: [[i32; 3]; 3] = [[0; 3]; 3];
//     for i in 0..3 {
//         for j in 0 ..3 {
//             result[i][j] = matrix[j][i];
//         }
//     }
//
//     result
// }

use std::fmt::Display;

fn pretty_print<T>(matrix: &[&[T]]) -> () where T: Display {
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
    //
    // let matrix3 = [
    //     [101, 102, 103], // <-- 这个注释会让 rustfmt 添加一个新行
    //     [201, 202, 203],
    //     [301, 302, 303],
    // ];
    // println!("matrix3:");
    // pretty_print(&matrix3);
    // println!();
    //
    // let matrix4 = [
    //     [101, 102, 103, 104], // <-- 这个注释会让 rustfmt 添加一个新行
    //     [201, 202, 203, 204],
    //     [301, 302, 303, 304],
    //     [401, 402, 403, 404],
    // ];
    // println!("matrix4:");
    // pretty_print(&matrix4);
    // println!();

    // let transposed = transpose(matrix);
    // println!("transposed:");
    // pretty_print(&transposed);
}

/* result:

*/
