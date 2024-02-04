// 015.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-1/implicit-conversions.html

use std::convert::TryInto;

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();

    test6();
    test7();
    test8();

    test9();
    test10();
}

fn test1() {
    println!("In test1()");

    let x: i8 = 15;
    let y: i16 = 1000;

    // println!("{x} * {y} = {}", multiply(x, y)); // expected `i16`, found `i8`. help: you can convert an `i8` to an `i16`
    println!("{x} * {y} = {}", multiply(x.into(), y));
}

fn test2() {
    println!("\nIn test2()");

    let x: f32 = 15.0;      // let x: f32 = 15; expected `f32`, found integer. help: use a float literal: `15.0`
    let y: f64 = 1000.0;    // let y: f64 = 1000; expected `f64`, found integer. help: use a float literal: `1000.0`

    // expected `i16`, found `f32`; expected `i16`, found `f64`
    // println!("{x} * {y} = {}", multiply(x, y));

    // the trait `From<f32>` is not implemented for `i16`
    // the trait `From<f64>` is not implemented for `i16`
    // println!("{x} * {y} = {}", multiply(x.into(), y.into()));

    println!("{x} * {y} = {}", multiply(x as i16, y as i16));
}

fn test3() {
    println!("\nIn test3()");

    let x: i32 = 15;
    let y: i64 = 1000;

    // expected `i16`, found `i32`; expected `i16`, found `i64`.
    // help: you can convert an `i32` to an `i16` and panic if the converted value doesn't fit
    // println!("{x} * {y} = {}", multiply(x, y));
    println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y.try_into().unwrap()));
}

fn test4() {
    println!("\nIn test4()");

    let x: i128 = 15;
    let y: isize = 1000;

    // expected `i16`, found `i128`; expected `i16`, found `isize`
    // help: you can convert an `i128` to an `i16` and panic if the converted value doesn't fit
    // help: you can convert an `isize` to an `i16` and panic if the converted value doesn't fit
    // println!("{x} * {y} = {}", multiply(x, y));
    println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y.try_into().unwrap()));
}

fn test5() {
    println!("\nIn test5()");

    let x: bool = true;
    let y: bool = false;

    // expected `i16`, found `bool`; expected `i16`, found `bool`
    // println!("{x} * {y} = {}", multiply(x, y));

    // println!("{x} * {y} = {}", multiply(if x { 1 } else { 0 }, if y { 1 } else { 0 }));
    println!("{x} * {y} = {}", multiply(x.into(), y.into()));
}

fn test6() {
    println!("In test6()");

    let x: u8 = 15;
    let y: u16 = 1000;

    // expected `i16`, found `u8`; expected `i16`, found `u16`
    // help: you can convert a `u8` to an `i16`
    // help: you can convert a `u16` to an `i16` and panic if the converted value doesn't fit
    // println!("{x} * {y} = {}", multiply(x, y));
    println!("{x} * {y} = {}", multiply(x.into(), y.try_into().unwrap()));
}

fn test7() {
    println!("\nIn test7()");

    let x: u32 = 15;
    let y: u64 = 1000;

    // expected `i16`, found `u32`; expected `i16`, found `u64`
    // help: you can convert a `u32` to an `i16` and panic if the converted value doesn't fit
    // help: you can convert a `u64` to an `i16` and panic if the converted value doesn't fit
    // println!("{x} * {y} = {}", multiply(x, y));
    println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y.try_into().unwrap()));
}

fn test8() {
    println!("\nIn test8()");

    let x: u128 = 15;
    let y: usize = 1000;

    // expected `i16`, found `u128`; expected `i16`, found `usize`
    // help: you can convert a `u128` to an `i16` and panic if the converted value doesn't fit
    // help: you can convert a `usize` to an `i16` and panic if the converted value doesn't fit
    // println!("{x} * {y} = {}", multiply(x, y));

    println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y.try_into().unwrap()));
}

fn test9() {
    println!("\nIn test9()");

    let x: String = String::from("15");
    let y: String = String::from("1000");

    // expected `i16`, found `String`. expected `i16`, found `String`
    // println!("{x} * {y} = {}", multiply(x, y));

    // the trait `From<String>` is not implemented for `i16`
    // println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y.try_into().unwrap()));

    // error[E0605]: non-primitive cast: `String` as `i16`
    // println!("{x} * {y} = {}", multiply(x as i16, y as i16));

    // error[E0282]: type annotations needed
    // help: try using a fully qualified path to specify the expected types
    // println!("{x} * {y} = {}", multiply(*x.try_into().unwrap(), *y.try_into().unwrap()));

    // error[E0277]: the trait bound `i16: From<String>` is not satisfied
    // = help: the following other types implement trait `From<T>`:
    //               <i16 as From<bool>>
    //               <i16 as From<i8>>
    //               <i16 as From<u8>>
    //               <i16 as From<NonZeroI16>>
    //     = note: required for `String` to implement `Into<i16>`
    //     = note: required for `i16` to implement `TryFrom<String>`
    //
    // error[E0614]: type `i16` cannot be dereferenced
    // println!("{x} * {y} = {}", multiply(*<String as TryInto<i16>>::try_into(x).unwrap(),
    //                                     *<String as TryInto<i16>>::try_into(y).unwrap()
    // ));

    println!("{x} * {y} = {}", multiply(x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()));
}

fn test10() {
    println!("\nIn test10()");

    let x: &str = "15";
    let y: &str = "1000";

    // expected `i16`, found `&str`. expected `i16`, found `&str`
    // println!("{x} * {y} = {}", multiply(x, y));

    // the trait `From<&str>` is not implemented for `i16`
    // println!("{x} * {y} = {}", multiply(x.try_into().unwrap(), y.try_into().unwrap()));

    // help: cast through a raw pointer first
    // error[E0606]: casting `&str` as `i16` is invalid
    // println!("{x} * {y} = {}", multiply(x as i16, y as i16));

    // help: try using a fully qualified path to specify the expected types
    // *<&str as TryInto<T>>::try_into(x).unwrap()
    // println!("{x} * {y} = {}", multiply(*x.try_into().unwrap(), *y.try_into().unwrap()));

    // error[E0277]: the trait bound `i16: From<&str>` is not satisfied
    //     = help: the following other types implement trait `From<T>`:
    //               <i16 as From<bool>>
    //               <i16 as From<i8>>
    //               <i16 as From<u8>>
    //               <i16 as From<NonZeroI16>>
    //     = note: required for `&str` to implement `Into<i16>`
    //     = note: required for `i16` to implement `TryFrom<&str>`
    //
    // error[E0614]: type `i16` cannot be dereferenced
    // println!("{x} * {y} = {}", multiply(*<&str as TryInto<i16>>::try_into(x).unwrap(),
    //                                     *<&str as TryInto<i16>>::try_into(y).unwrap())
    // );

    println!("{x} * {y} = {}", multiply(x.parse::<i16>().unwrap(), y.parse::<i16>().unwrap()));
}

/* result:
In test1()
15 * 1000 = 15000

In test2()
15 * 1000 = 15000

In test3()
15 * 1000 = 15000

In test4()
15 * 1000 = 15000

In test5()
true * false = 0
In test6()
15 * 1000 = 15000

In test7()
15 * 1000 = 15000

In test8()
15 * 1000 = 15000

In test9()
15 * 1000 = 15000

In test10()
15 * 1000 = 15000
*/
