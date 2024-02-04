fn main() {
  // let functions = vec![
  //     ("p86a", p86a),
  //     ("p87a", p87a),
  //     ("p88a", p88a),
  // ];
  /*
  error[E0308]: mismatched types
  --> src\main.rs:4:18
    |
  4 |         ("p87a", p87a),
    |                  ^^^^ expected fn item, found a different fn item
    |
    = note: expected fn item `fn() {p86a}`
              found fn item `fn() {p87a}`
    = note: different fn items have unique types, even if their signatures are the same
    = help: consider casting both fn items to fn pointers using `as fn()`

  error[E0308]: mismatched types
  --> src\main.rs:5:18
    |
  5 |         ("p88a", p88a),
    |                  ^^^^ expected fn item, found a different fn item
    |
    = note: expected fn item `fn() {p86a}`
              found fn item `fn() {p88a}`
    = note: different fn items have unique types, even if their signatures are the same
    = help: consider casting both fn items to fn pointers using `as fn()`

  For more information about this error, try `rustc --explain E0308`.
  */
}

fn p86a() {
  println!("\nIn p86a()");

  // {
  //     let s = "hello";
  // }
  // println!("{s}");
}
/*
error[E0425]: cannot find value `s` in this scope
--> src\main.rs:7:16
|
7 |     println!("{s}");
|                ^
|
help: the binding `s` is available in a different scope in the same function
--> src\main.rs:5:13
|
5 |         let s = "hello";
|             ^

For more information about this error, try `rustc --explain E0425`.
*/