fn test1() {
    #[derive(PartialEq)]
    struct AlwaysEqual;

    let a = AlwaysEqual;
    let b = AlwaysEqual;
    println!("Is a same to b? {}.", if a == b { "yes" } else { "no" });
    // => Is a same to b? yes.

    // // error
    // #[derive(PartialEq)]
    // struct AlwaysEqualNew;
    //
    // #[allow(unused_variables)]
    // let c = AlwaysEqualNew;
    // println!("Is a same to c? {}.", if a == c { "yes" } else { "no" });
    /*
      error[E0308]: mismatched types
        --> src\main.rs:16:45
        |
      16 |     println!("Is a same to c? {}.", if a == c { "yes" } else { "no" });
        |                                        -    ^ expected `AlwaysEqual`, found `AlwaysEqualNew`
        |                                        |
        |                                        expected because this is `AlwaysEqual`

      For more information about this error, try `rustc --explain E0308`.
    */
}

fn test2() {
    println!("\n\nIn test2()");
    //
    // struct AlwaysEqual;
    // impl PartialEq for AlwaysEqual{
    //     fn eq(&self, other: &Self) -> bool {
    //         todo!()
    //     }
    //
    //     fn ne(&self, other: &Self) -> bool {
    //         todo!()
    //     }
    // }
    //
    // struct AlwaysEqualNew;
    // impl PartialEq for AlwaysEqualNew{
    //     fn eq(&self, other: &Self) -> bool {
    //         todo!()
    //     }
    // }
    //
    // let a = AlwaysEqual;
    // let c = AlwaysEqualNew;
    // println!("Is a same to c? {}.", if a == c { "yes" } else { "no" });
}

fn test3() {
    println!("\n\nIn test3()");

    // #[derive(PartialEq)]
    // struct AlwaysEqual;
    // #[derive(PartialEq)]
    // struct AlwaysEqualNew;
    //
    // let a = AlwaysEqual;
    // let c = AlwaysEqualNew;
    // println!(
    //     "Is a same to c? {}.",
    //     if a == (c as AlwaysEqual) { "yes" } else { "no" }
    // );
}
/*
error[E0605]: non-primitive cast: `AlwaysEqualNew` as `test3::AlwaysEqual`
  --> src\main.rs:68:17
   |
68 |         if a == (c as AlwaysEqual) { "yes" } else { "no" }
   |                 ^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object

For more information about this error, try `rustc --explain E0605`.
*/

fn test4() {
    println!("\n\nIn test4()");

    // https://juejin.cn/post/6919003661280477198

    // #[derive(PartialEq)]
    // struct AlwaysEqual;
    // #[derive(PartialEq)]
    // struct AlwaysEqualNew;
    //
    // let a = AlwaysEqual;
    // let c = AlwaysEqualNew;
    // println!(
    //     "Is a same to c? {}.",
    //     if a == (c as AlwaysEqual) { "yes" } else { "no" }
    // );
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}
