// error/002.rs
// https://www.runoob.com/rust/rust-basic-syntax.html

const a = 123;
a = "abc";
a = 4.56; 
a = 456;

/* result:
error: expected one of `!` or `::`, found `=`
 --> 003_error2.rs:4:3
  |
4 | a = "abc";
  |   ^ expected one of `!` or `::`

error: missing type for `const` item
 --> 003_error2.rs:3:8
  |
3 | const a = 123;
  |        ^ help: provide a type for the item: `: <type>`

error: aborting due to 2 previous errors
*/
