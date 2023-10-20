// 008.rs
// https://www.runoob.com/rust/rust-function.html

fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x: {}", x);
    println!("y: {}", y);
}

/* result:
x: 5
y: 4
*/
