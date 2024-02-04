// 101.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/drop.html

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }

    // Ok
    drop(a);

    // rror[E0040]: explicit use of destructor method --> ..\rs\101.rs:26:7
    // explicit destructor calls not allowed
    // help: consider using `drop` function: `drop(a)`
    // a.drop();

    println!("Exiting main");
}

/* result:
Exiting block B
Dropping d
Dropping c
Exiting block A
Dropping b
Dropping a
Exiting main
*/

/*
warning: unused variable: `b`
  --> ..\rs\101.rs:17:13
   |
17 |         let b = Droppable { name: "b" };
   |             ^ help: if this is intentional, prefix it with an underscore: `_b`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `c`
  --> ..\rs\101.rs:19:17
   |
19 |             let c = Droppable { name: "c" };
   |                 ^ help: if this is intentional, prefix it with an underscore: `_c`

warning: unused variable: `d`
  --> ..\rs\101.rs:20:17
   |
20 |             let d = Droppable { name: "d" };
   |                 ^ help: if this is intentional, prefix it with an underscore: `_d`

warning: 3 warnings emitted
*/