// 004.rs
// https://www.runoob.com/rust/rust-data-types.html

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余

    println!("x is {}, y is {}, sum is {}, difference is {}, product is {}, quotient is {}, remainder is {}",
        x, y, sum, difference, product, quotient, remainder
    );
}

/* result:
x is 2, y is 3, sum is 15, difference is 91.2, product is 120, quotient is 1.7608695652173911, remainder is 3
*/
