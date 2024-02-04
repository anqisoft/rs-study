fn test1() {
    println!("In test1()");

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
/*
In test1()
condition was true
*/

fn test2() {
    println!("\nIn test2()");

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
/*
In test2()
condition was false
*/

// P77
// fn error1() {
//     println!("\nIn error1()");

//     let number = 3;

//     if number {
//         println!("condition was true");
//     }
// }
/*
error[E0308]: mismatched types
  --> src\main.rs:39:8
   |
39 |     if number {
   |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
*/

// P77
fn test3() {
    println!("\nIn test3()");

    let number = 7;

    if number != 0 {
        println!("number was something other than zero");
    }
}
/*
In test3()
number was something other than zero
*/

// P77
fn test4() {
    println!("\nIn test4()");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
/*
In test4()
number is divisible by 3
*/

// P78
fn let_and_if() {
    println!("\nIn let_and_if()");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
/*
In let_and_if()
The value of number is: 5
*/

// fn let_and_if_error() {
//     let condition = true;
//     let number = if condition { 5 } else { "six" };
//     println!("The value of number is: {number}");
// }
/*
error[E0308]: `if` and `else` have incompatible types
   --> src\main.rs:103:44
    |
103 |     let number = if condition { 5 } else { "six" };
    |                                 -          ^^^^^ expected integer, found `&str`
    |                                 |
    |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
*/

fn main() {
    test1();
    test2();

    // error1();
    test3();
    test4();

    let_and_if();
    // let_and_if_error();
}
