// error/017.rs
// https://www.runoob.com/rust/rust-struct.html

fn main() {
    #[derive(Debug)]
    struct UnitStruct;
    let unit_struct = UnitStruct { test: 1 };
    println!("{:?}", unit_struct);
}

/* result:
error[E0560]: struct `UnitStruct` has no field named `test`
 --> ..\017.rs:7:36
  |
7 |     let unit_struct = UnitStruct { test: 1 };
  |                                    ^^^^ `UnitStruct` does not have this field
  |
  = note: all struct fields are already assigned

error: aborting due to previous error

For more information about this error, try `rustc --explain E0560`.
*/
