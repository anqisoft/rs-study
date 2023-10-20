// 010.rs
// https://www.runoob.com/rust/rust-conditions.html

fn main() {
    let number = 3;
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    }
    else if a < 0 {
        b = -1;
    }
    else {
        b = 0;
    }
    println!("b is {}", b);
}
/* result:
true
b is 1
*/
