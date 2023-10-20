// 079.rs
// https://www.runoob.com/rust/rust-concurrency.html

fn main() {
    closure1();
    closure2();
}

fn closure1() {
    let inc = |num: i32| -> i32 {
        num + 1
    };
    println!("inc(5) = {}", inc(5));
}

fn closure2() {
    let inc = |num| {
        num + 1
    };
    // println!("inc(5) = {}", inc(5)); // `()` does not implement `Display` (required by `{}`) [E0277]
    println!("inc(5) = {:?}", inc(5));
}

/* result:
inc(5) = 6
inc(5) = 6
*/
