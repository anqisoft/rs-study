// 020.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/blocks.html

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("In test1()");
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4
            };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}

fn test2() {
    println!("\nIn test2()");
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4;
                3 + 4
            };
            println!("w: {w}");
            y * w;
            y * w
        };
        println!("z: {z}");
        z - y;
        z - y
    };
    println!("x: {x}");
}

/* result:
In test1()
y: 10
w: 7
z: 70
x: 60

In test2()
y: 10
w: 7
z: 70
x: 60
*/


/* warnings
warning: unused arithmetic operation that must be used
  --> ..\rs\020.rs:34:17
   |
34 |                 3 + 4;
   |                 ^^^^^ the arithmetic operation produces a value
   |
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
34 |                 let _ = 3 + 4;
   |                 +++++++

warning: unused arithmetic operation that must be used
  --> ..\rs\020.rs:38:13
   |
38 |             y * w;
   |             ^^^^^ the arithmetic operation produces a value
   |
help: use `let _ = ...` to ignore the resulting value
   |
38 |             let _ = y * w;
   |             +++++++

warning: unused arithmetic operation that must be used
  --> ..\rs\020.rs:42:9
   |
42 |         z - y;
   |         ^^^^^ the arithmetic operation produces a value
   |
help: use `let _ = ...` to ignore the resulting value
   |
42 |         let _ = z - y;
   |         +++++++

warning: 3 warnings emitted
*/