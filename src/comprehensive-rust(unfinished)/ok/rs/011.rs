// 011.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/functions.html

fn main() {
    print_fizzbuzz_to(20);
}

fn is_divisible(n: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    n % divisor == 0
}

fn fizzbuzz(n: u32) -> String {
    let fizz = if is_divisible(n, 3) { "fizz" } else { "" };
    let buzz = if is_divisible(n, 5) { "buzz" } else { "" };
    if fizz.is_empty() && buzz.is_empty() {
        return format!("{n}");
    }
    format!("{fizz}{buzz}")
}

fn print_fizzbuzz_to(n: u32) {
    for i in 1..=n {
        println!("{}", fizzbuzz(i));
    }

    println!();
    for i in 1..=n {
        println!("{i} => {}", fizzbuzz(i));
    }

    println!();
    for i in 1..n { // 1..<n
        println!("{i} => {}", fizzbuzz(i));
    }

    // println!();
    // for i in 1..<n { // expected one of 7 possible tokens
    //     println!("i => {}", fizzbuzz(i));
    // }

    // println!();
    // for i in 1..>n { // expected `{`
    //     println!("i => {}", fizzbuzz(i));
    // }
}

/* result:
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14
fizzbuzz
16
17
fizz
19
buzz

1 => 1
2 => 2
3 => fizz
4 => 4
5 => buzz
6 => fizz
7 => 7
8 => 8
9 => fizz
10 => buzz
11 => 11
12 => fizz
13 => 13
14 => 14
15 => fizzbuzz
16 => 16
17 => 17
18 => fizz
19 => 19
20 => buzz

1 => 1
2 => 2
3 => fizz
4 => 4
5 => buzz
6 => fizz
7 => 7
8 => 8
9 => fizz
10 => buzz
11 => 11
12 => fizz
13 => 13
14 => 14
15 => fizzbuzz
16 => 16
17 => 17
18 => fizz
19 => 19
*/
