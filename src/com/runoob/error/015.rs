// error/015.rs
// https://www.runoob.com/rust/rust-struct.html

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

let runoob = Site {
    domain: String::from("www.runoob.com"),
    name: String::from("RUNOOB"),
    nation: String::from("China"),
    found: 2013
};

println ! ("{:?}", runoob);

/* result:
error: expected item, found keyword `let`
  --> ..\015.rs:11:1
   |
11 | let runoob = Site {
   | ^^^ consider using `const` or `static` instead of `let` for global variables

error: aborting due to previous error
*/
