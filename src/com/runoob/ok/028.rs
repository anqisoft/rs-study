// 028.rs
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

    // println!("runoob is {:?}", runoob); // Error
    println!("site is {:?}", site);
}

/* result:
site is Site { domain: "www.runoob.com", name: "RUNOOB", nation: "China", found: 2013 }


P:\anqi\Desktop\tech\rust\projects\rust-study\src\com\runoob\ok\bat>(cd ..\debug   && rustc ..\028.rs   && 028  1>..\debug\028.txt  && start "" notepad ..\debug\028.txt )  || pause
warning: fields `domain`, `name`, `nation`, and `found` are never read
  --> ..\028.rs:7:5
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
