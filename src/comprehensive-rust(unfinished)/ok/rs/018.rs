// 018.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/for-loops.html

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0 ..3 {
            result[i][j] = matrix[j][i];
        }
    }

    result
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        print!("[");
        let array = matrix[i];
        for j in 0..3 {
            print!("{}{}", array[j], if j == 2 { "" } else { ", "});
        }
        println!("]");
    }
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
matrix:
[101, 102, 103]
[201, 202, 203]
[301, 302, 303]
transposed:
[101, 201, 301]
[102, 202, 302]
[103, 203, 303]
*/