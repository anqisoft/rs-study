// 026.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/loop-expressions.html

fn main() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        if x == 1 {
            break;
        }
    }
    println!("x: {x}");

    for n in 11..=30 {
        println!();
        fx(n);
    }
}

fn fx(mut x: i32) {
    print!("{x}");

    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        print!(" => {x}");

        if x == 1 {
            break;
        }
    }
}
/* result:
x: 1
*/
