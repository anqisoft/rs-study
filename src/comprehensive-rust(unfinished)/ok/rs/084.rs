// 084.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-2/iterators-and-ownership.html

fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];
    let mut iter = v.into_iter();

    // error: invalid const generic expression --> ..\rs\084.rs:8:20
    // help: expressions must be enclosed in braces to be used as const generic arguments
    // let v0: Option<{ .. }> = iter.next();
    // error[E0747]: constant provided when a type was expected --> ..\rs\084.rs:8:20
    // let v0: Option<..> = iter.next();

    // error[E0747]: constant provided when a type was expected --> ..\rs\084.rs:14:20
    // let v0: Option<{ .. }> = iter.next();

    let v0: Option<String> = iter.next();
    println!("v0: {v0:?}");
    
    // expected `Option<&str>`, found `Option<String>`
    // help: try using `.as_deref()` to convert `Option<String>` to `Option<&str>`
    // let v1: Option<&str> = iter.next().as_deref();
    // let v1: Option<&str> = iter.next();

    // error[E0716]: temporary value dropped while borrowed --> ..\rs\084.rs:25:28
    // - temporary value is freed at the end of this statement
    // creates a temporary value which is freed while still in use
    // ------ borrow later used here
    // help: consider using a `let` binding to create a longer lived value
    // let v1: Option<&str> = iter.next().as_deref();
   
    let binding = iter.next();
    let v1: Option<&str> = binding.as_deref();
    println!("v1: {v1:?}");
}

/* result:
v0: Some("foo")
v1: Some("bar")
*/
