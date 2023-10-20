// error/001.rs
// https://www.runoob.com/rust/rust-basic-syntax.html

let a = 123;
a = "abc";
a = 4.56; 
a = 456;

/* result:
error: expected item, found keyword `let`
 --> 003_error1.rs:3:1
  |
3 | let a = 123;
  | ^^^ consider using `const` or `static` instead of `let` for global variables

error: aborting due to previous error
*/
