// 083.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-2/iterators-and-ownership.html

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("In test1()");
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    println!("v[0]: {:?}", iter.next());
    println!("v[1]: {:?}", iter.next());
    println!("v[2]: {:?}", iter.next());
    println!("No more items: {:?}", iter.next());
}

fn test2() {
    let v: Vec<i8> = vec![10, 20, 30];
    let mut iter = v.iter();

    // error: invalid const generic expression
    // help: expressions must be enclosed in braces to be used as const generic arguments
    // let v0: Option<..> = iter.next();

    // error[E0747]: constant provided when a type was expected
    // let v0: Option<{ .. }> = iter.next();

    // expected `Option<i8>`, found `Option<&i8>`
    // help: use `Option::copied` to copy the value inside the `Option`
    // let v0: Option<i8> = iter.next();

    let v0: Option<i8> = iter.next().copied();
    println!("v0: {v0:?}");
    println!("v0: {v0:?}");

    let v1: Option<&i8> = iter.next();
    println!("v1: {v1:?}");
    println!("v1: {v1:?}");
}

/* result:
In test1()
v[0]: Some(10)
v[1]: Some(20)
v[2]: Some(30)
No more items: None
v0: Some(10)
v0: Some(10)
v1: Some(20)
v1: Some(20)
*/
