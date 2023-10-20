// error/014.rs
// https://www.runoob.com/rust/rust-struct.html

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

{
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };

    println ! ("{:?}", runoob);
}

/* result:
error: expected item, found `{`
  --> ..\014.rs:11:1
   |
11 | {
   | ^ expected item

error: aborting due to previous error

Press any key to continue . . .
Not enough memory resources are available to process this command.
*/
