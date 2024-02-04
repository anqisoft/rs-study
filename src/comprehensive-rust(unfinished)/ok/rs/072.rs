// 072.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/option-result.html

fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    println!("first: {first:?}");

    // no method named `try_into` found for struct `Vec<i8>` in the current scope
    // let arr: Result<[i8; 3], Vec<i8>> = numbers.try_into();
    
    // let arr: Result<[i8; 3], Vec<i8>> = Err(numbers);
    // println!("arr: {arr:?}");

    let arr: Result<[i8; 3], Vec<i8>> = Err(numbers.clone());
    println!("arr: {arr:?}");

    use std::convert::TryInto;
    let arr: Result<[i8; 3], Vec<_>> = numbers.try_into();
    println!("arr: {arr:?}");
}

/* result:
first: Some(10)
arr: Err([10, 20, 30])
arr: Ok([10, 20, 30])
*/
