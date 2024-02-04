// 016.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/for-loops.html

fn main() {
    let array = [10, 20, 30];
    println!("{:?}", array);
    println!("{array:?}");
    println!("{:#?}", array);
    println!("{array:#?}");
    println!();

    print!("Iterating over array:");
    for n in &array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}

/* result:
[10, 20, 30]
[10, 20, 30]
[
    10,
    20,
    30,
]
[
    10,
    20,
    30,
]

Iterating over array: 10 20 30
Iterating over range: 10 20 30
*/
