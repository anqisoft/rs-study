// 027.rs
// https://www.runoob.com/rust/rust-struct.html

#[derive(Debug)]

struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

fn main() {
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };

    println!("{:?}", runoob);
}

/* result:
Site { domain: "www.runoob.com", name: "RUNOOB", nation: "China", found: 2013 }


P:\anqi\Desktop\tech\rust\projects\rust-study\src\com\runoob\ok\bat>(cd ..\debug   && rustc ..\027.rs   && 027  1>..\debug\027.txt  && start "" notepad ..\debug\027.txt )  || pause
warning: fields `domain`, `name`, `nation`, and `found` are never read
  --> ..\027.rs:7:5
   |
6  | struct Site {
   |        ---- fields in this struct
7  |     domain: String,
   |     ^^^^^^
8  |     name: String,
   |     ^^^^
9  |     nation: String,
   |     ^^^^^^
10 |     found: u32
   |     ^^^^^
   |
   = note: `Site` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default
*/
