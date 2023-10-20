// error/016.rs
// https://www.runoob.com/rust/rust-struct.html

#[derive(Debug)]

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

fn main() {
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        // traffic: 2013, // The original code is wrong.
        found: 2013,
    };

    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };

    println!("runoob is {:?}", runoob);
    println!("site is {:?}", site);
}

/* result:
error[E0382]: borrow of partially moved value: `runoob`
  --> ..\016.rs:30:32
   |
24 |       let site = Site {
   |  ________________-
25 | |         domain: String::from("www.runoob.com"),
26 | |         name: String::from("RUNOOB"),
27 | |         ..runoob
28 | |     };
   | |_____- value partially moved here
29 |
30 |       println!("runoob is {:?}", runoob);
   |                                  ^^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `runoob.nation` has type `String`, which does not implement the `Copy` trait
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
*/