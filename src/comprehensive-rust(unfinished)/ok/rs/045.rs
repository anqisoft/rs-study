// 045.rs
// https://google.github.io/comprehensive-rust/zh-CN/pattern-matching/destructuring-enums.html

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

// fn main() {
//     let n = 100;
//     match divide_in_two(n) {
//         Result::Ok(half) => println!("{n} divided in two is {half}"),
//         Result::Err(msg) => println!("sorry, an error happened: {msg}"),
//     }
// }

fn main() {
    for n in 100..=101 {
        match divide_in_two(n) {
            Result::Ok(half) => println!("{n} divided in two is {half}"),
            Result::Err(msg) => println!("sorry, an error happened: {msg}"),
        }
    }
}

/* result 1:
100 divided in two is 50
*/

/* result 2:
100 divided in two is 50
sorry, an error happened: cannot divide 101 into two equal parts
*/
