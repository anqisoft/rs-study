/*
 * Copyright (c) 2023 anqisoft@gmail.com
 * chapter19\src\main.rs
 * Reference Links:
 * <en>https://doc.rust-lang.org/book/ch19-00-advanced-features.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch19-00-advanced-features.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch19-00-advanced-features.html</tw>
*/

/* <en>
 * Created on Wed Dec 13 2023 20:16:27
 * Feature: Test the code of Chapter 19 in the Chinese version pdf.
 * The method name starts with p, followed by the PDF page number and the current page sub-number (a to z).
 * For example, p1a represents the first example on the first page of the PDF.
 * </en>
*/

/* <cn>
 * 创建：2023年12月13日 20:16:27
 * 功能：测试中文版pdf中第19章的代码。方法名以p开头，后面加pdf页序号及当页子序号（a到z），如p1a表示pdf第一页第一个例子。
 * </cn>
*/

/* <tw>
 * 創建：2023年12月13日 20:16:27
 *  功能：測試中文版pdf中第19章的程式碼。方法名稱以p開頭，後面加pdf頁序號及當頁子序號（a到z），如p1a表示pdf第一頁第一個例子。
 * </tw>
*/

/*
 * <en>https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch19-01-unsafe-rust.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch19-01-unsafe-rust.html</tw>
*/

#[allow(unused_variables)]
fn f01_eg01() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

#[allow(unused_variables)]
fn f02_eg02() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

#[allow(unused_variables)]
fn f03_eg03() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

fn f04() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn f05_eg04() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

/* fn f06_eg05() { // error[E0499]: cannot borrow `*values` as mutable more than once at a time
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();

        assert!(mid <= len);

        (&mut values[..mid], &mut values[mid..])
    }
} */

fn f07_eg06() {
    use std::slice;

    fn _split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
}

fn f08_eg07() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

fn f09_eg08() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    // }
}

fn f10_eg09() {
    static HELLO_WORLD: &str = "Hello, world!";

    // fn main() {
    println!("name is: {}", HELLO_WORLD);
    // }
}

fn f11_eg10() {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    // fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
    // }
}

fn f12_eg11() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // fn main() {}
}

/* // eg12
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
} */

/* // eg13
pub trait Iterator<T> {
  fn next(&mut self) -> Option<T>;
} */

/*
* <en>https://doc.rust-lang.org/book/ch19-03-advanced-traits.html</en>
* <cn>https://kaisery.github.io/trpl-zh-cn/ch19-03-advanced-traits.html</cn>
* <tw>https://rust-lang.tw/book-tw/ch19-03-advanced-traits.html</tw>
*/

fn f13_eg14() {
    use std::ops::Add;
    // trait Add<Rhs=Self> {
    //   type Output;

    //   fn add(self, rhs: Rhs) -> Self::Output;
    // }

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    // fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    // }
}

fn f14_eg15() {
    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

fn f15_eg16_to_eg18() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // eg17
    // fn main() {
    println!("\nIn eg17:");
    let person = Human;
    person.fly();
    // }

    // eg18
    {
        println!("\nIn eg18:");
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
    }
}

fn f16_eg19() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // eg19
    {
        println!("A baby dog is called a {}", Dog::baby_name());
    }

    // eg20, error
    // {
    //     println!("A baby dog is called a {}", Animal::baby_name());
    // }
    /*
      error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
        --> src\main.rs:307:47
          |
      283 |         fn baby_name() -> String;
          |         ------------------------- `Animal::baby_name` defined here
      ...
      307 |         println!("A baby dog is called a {}", Animal::baby_name());
          |                                               ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
          |
      help: use the fully-qualified path to the only available implementation
          |
      307 |         println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    */

    // eg21
    {
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }
}

/* fn f17_eg22_with_error() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}
} */
/*
error[E0277]: `f17_eg22_with_error::Point` doesn't implement `std::fmt::Display`
   --> src\main.rs:336:27
    |
336 |     impl OutlinePrint for Point {}
    |                           ^^^^^ `f17_eg22_with_error::Point` cannot be formatted with the default formatter
    |
    = help: the trait `std::fmt::Display` is not implemented for `f17_eg22_with_error::Point`
    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `f17_eg22_with_error::OutlinePrint`
   --> src\main.rs:319:25
    |
319 |     trait OutlinePrint: fmt::Display {
    |                         ^^^^^^^^^^^^ required by this bound in `OutlinePrint`
*/

fn f17_eg22() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

fn f18_eg23() {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

/*
 * <en>https://doc.rust-lang.org/book/ch19-04-advanced-types.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch19-04-advanced-types.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch19-04-advanced-types.html</tw>
*/
fn f19() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

fn f20() {
    // eg24
    {
        let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

        fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
            f();
        }

        fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
            // --snip--
            Box::new(|| println!("hello"))
        }

        takes_long_type(f);
        takes_long_type(returns_long_type());
    }

    // eg25
    {
        type Thunk = Box<dyn Fn() + Send + 'static>;

        let f: Thunk = Box::new(|| println!("hi"));

        fn takes_long_type(f: Thunk) {
            f();
        }

        fn returns_long_type() -> Thunk {
            // --snip--
            Box::new(|| println!("hello"))
        }

        takes_long_type(f);
        takes_long_type(returns_long_type());
    }
}

fn f21() {
    use std::fmt;

    {
        use std::io::Error;
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
            fn flush(&mut self) -> Result<(), Error>;

            fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
        }
    }

    {
        use std::io::Result;
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;

            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }
}

fn f22() {
    fn _bar() -> ! {
        // --snip--
        panic!("bar()");

        // // couldn't use:
        // !
    }
}

fn f23_eg26() {
    // ok, in eg2-5
    // let guess: u32 = match guess.trim().parse() {
    //     Ok(num) => num,
    //     Err(_) => continue,
    // };
}

/* fn f24() {
    let guess = String::from(" test ");

    let guess = match guess.trim().parse() {
        Ok(_) => 5,
        Err(_) => "hello",
    };
    println!("{guess}");
} */
/*
error[E0308]: `match` arms have incompatible types
   --> src\main.rs:513:19
    |
511 |       let guess = match guess.trim().parse() {
    |  _________________-
512 | |         Ok(_) => 5,
    | |                  - this is found to be of type `{integer}`
513 | |         Err(_) => "hello",
    | |                   ^^^^^^^ expected integer, found `&str`
514 | |     };
    | |_____- `match` arms have incompatible types
*/

// impl<T> Option<T> {
//   pub fn unwrap(self) -> T {
//       match self {
//           Some(val) => val,
//           None => panic!("called `Option::unwrap()` on a `None` value"),
//       }
//   }
// }

/* {
  print!("forever ");

  loop {
      print!("and ever ");
  }
} // get: ! */

/* fn f25_with_error() {
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
} */
/*
error[E0308]: mismatched types
   --> src\main.rs:549:19
    |
549 |     let s1: str = "Hello there!";
    |             ---   ^^^^^^^^^^^^^^ expected `str`, found `&str`
    |             |
    |             expected due to this

error[E0308]: mismatched types
   --> src\main.rs:550:19
    |
550 |     let s2: str = "How's it going?";
    |             ---   ^^^^^^^^^^^^^^^^^ expected `str`, found `&str`
    |             |
    |             expected due to this

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src\main.rs:549:9
    |
549 |     let s1: str = "Hello there!";
    |         ^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature
help: consider borrowing here
    |
549 |     let s1: &str = "Hello there!";
    |             +

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src\main.rs:550:9
    |
550 |     let s2: str = "How's it going?";
    |         ^^ doesn't have a size known at compile-time
    |
    = help: the trait `Sized` is not implemented for `str`
    = note: all local variables must have a statically known size
    = help: unsized locals are gated as an unstable feature
help: consider borrowing here
    |
550 |     let s2: &str = "How's it going?";
    |             +
*/

/*
 * <en>https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch19-05-advanced-functions-and-closures.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch19-05-advanced-functions-and-closures.html</tw>
*/

fn f26_eg27() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    // fn main()
    {
        let answer = do_twice(add_one, 5);

        println!("The answer is: {}", answer);
    }
}

fn f27() {
    {
        let list_of_numbers = vec![1, 2, 3];
        let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    }

    {
        let list_of_numbers = vec![1, 2, 3];
        let _list_of_strings: Vec<String> =
            list_of_numbers.iter().map(ToString::to_string).collect();
    }

    {
        enum Status {
            Value(u32),
            // Stop, // warning: variant `Stop` is never constructed
        }

        let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }

    // error
    // {
    //     fn returns_closure() -> dyn Fn(i32) -> i32 {
    //         |x| x + 1
    //     }
    // }
    /*
     error[E0746]: return type cannot have an unboxed trait object
       --> src\main.rs:644:33
         |
     644 |         fn returns_closure() -> dyn Fn(i32) -> i32 {
         |                                 ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
         |
     help: return an `impl Trait` instead of a `dyn Trait`, if all returned values are the same type
         |
     644 |         fn returns_closure() -> impl Fn(i32) -> i32 {
         |                                 ~~~~
     help: box the return type, and wrap all of the returned values in `Box::new`
         |
     644 ~         fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
     645 ~             Box::new(|x| x + 1)
    */

    // ok
    {
        fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}
/*
 * <en>https://doc.rust-lang.org/book/ch19-06-macros.html</en>
 * <cn>https://kaisery.github.io/trpl-zh-cn/ch19-06-macros.html</cn>
 * <tw>https://rust-lang.tw/book-tw/ch19-06-macros.html</tw>
*/

fn f28_eg28() {
    let _v: Vec<u32> = vec_new![1, 2, 3];

    {
        #[macro_export]
        macro_rules! vec_new {
          ( $( $x:expr ),* ) => {
              {
                  let mut temp_vec = Vec::new();
                  $(
                      temp_vec.push($x);
                  )*
                  temp_vec
              }
          };
      }
    }

    // get
    fn _expand_to_code() -> Vec<i32> {
        let mut temp_vec = Vec::new();
        temp_vec.push(1);
        temp_vec.push(2);
        temp_vec.push(3);
        temp_vec
    }
}

fn f29_eg29() {
    // use proc_macro;

    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }
}

/* fn f30_eg30() {
    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    // error: can't use a procedural macro from the same crate that defines it
    #[derive(HelloMacro)]
    struct Pancakes;

    impl HelloMacro for Pancakes {
        fn hello_macro() {
            println!("Hello, Macro! My name is Pancakes!");
        }
    }

    // fn main()
    {
        Pancakes::hello_macro();
    }

    mod hello_macro {
        pub trait HelloMacro {
            fn hello_macro();
        }
    }

    // eg31
    mod hello_macro_derive {
        // error[E0432]: unresolved import `proc_macro`
        use proc_macro::TokenStream; //error[E0432]: unresolved import `quote`
        use quote::quote; //error[E0432]: unresolved import `syn`
        use syn;

        // error: the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
        #[proc_macro_derive(HelloMacro)]
        pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
            // Construct a representation of Rust code as a syntax tree
            // that we can manipulate
            let ast = syn::parse(input).unwrap();

            // Build the trait implementation
            impl_hello_macro(&ast)
        }

        // eg33
        fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
            let name = &ast.ident; // error: cannot determine resolution for the macro `quote`
            let gen = quote! {
                impl HelloMacro for #name {
                    fn hello_macro() {
                        println!("Hello, Macro! My name is {}!", stringify!(#name));
                    }
                }
            };
            gen.into()
        }
    }
} */

/* fn f31() {
    // error[E0432]: unresolved import `proc_macro`
    use proc_macro::TokenStream;
    // error: can't use a procedural macro from the same crate that defines it
    #[route(GET, "/")]
    fn index() {}

    // error: the `#[proc_macro_attribute]` attribute is only usable with crates of the `proc-macro` crate type
    #[proc_macro_attribute]
    pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
} */

/* fn f32() {
    // error[E0432]: unresolved import `proc_macro`
    use proc_macro::TokenStream;

    // error: can't use a procedural macro from the same crate that defines it
    let sql = sql!(SELECT * FROM posts WHERE id=1);
    //error: the `#[proc_macro]` attribute is only usable with crates of the `proc-macro` crate type
    #[proc_macro]
    pub fn sql(input: TokenStream) -> TokenStream {}
} */

fn f30_eg30() {}
fn f31() {}
fn f32() {}

/* // eg32
DeriveInput {
  // --snip--

  ident: Ident {
      ident: "Pancakes",
      span: #0 bytes(95..103)
  },
  data: Struct(
      DataStruct {
          struct_token: Struct,
          fields: Unit,
          semi_token: Some(
              Semi
          )
      }
  )
} */

fn main() {
    // f01_eg01();
    // f02_eg02();
    // println!("\nIn eg03();");
    // f03_eg03();

    // f04();
    // f05_eg04();
    // // f06_eg05();
    // f07_eg06();
    // f08_eg07();
    // println!("\nIn eg08();");
    // f09_eg08();
    // println!("\nIn eg09();");
    // f10_eg09();
    // println!("\nIn eg10();");
    // f11_eg10();
    // f12_eg11();
    // f13_eg14();
    // f14_eg15();
    // f15_eg16_to_eg18();
    // println!("\nIn eg19();");
    // f16_eg19();
    // f17_eg22();
    // println!("\nIn eg23();");
    // f18_eg23();
    // println!("\nIn f19();");
    // f19();
    // println!("\nIn f20();");
    // f20();
    // f21();
    // f22();
    // f23_eg26();
    // println!("\nIn eg27();");
    // f26_eg27();
    // f27();
    // f28_eg28();
    // f29_eg29();
    // f30_eg30();
    // f31();
    // f32();
    call_functions();
}

fn call_functions() {
    for (name, f) in vec![
        ("f01_eg01", f01_eg01 as fn()),
        ("f02_eg02", f02_eg02 as fn()),
        ("f03_eg03", f03_eg03 as fn()),
        ("f04", f04 as fn()),
        ("f05_eg04", f05_eg04 as fn()),
        // ("f06_eg05", f06_eg05 as fn()),
        ("f07_eg06", f07_eg06 as fn()),
        ("f08_eg07", f08_eg07 as fn()),
        ("f09_eg08", f09_eg08 as fn()),
        ("f10_eg09", f10_eg09 as fn()),
        ("f11_eg10", f11_eg10 as fn()),
        ("f12_eg11", f12_eg11 as fn()),
        ("f13_eg14", f13_eg14 as fn()),
        ("f14_eg15", f14_eg15 as fn()),
        ("f15_eg16_to_eg18", f15_eg16_to_eg18 as fn()),
        ("f16_eg19", f16_eg19 as fn()),
        ("f17_eg22", f17_eg22 as fn()),
        ("f18_eg23", f18_eg23 as fn()),
        ("f19", f19 as fn()),
        ("f20", f20 as fn()),
        ("f21", f21 as fn()),
        ("f22", f22 as fn()),
        ("f23_eg26", f23_eg26 as fn()),
        ("f26_eg27", f26_eg27 as fn()),
        ("f27", f27 as fn()),
        ("f28_eg28", f28_eg28 as fn()),
        ("f29_eg29", f29_eg29 as fn()),
        ("f30_eg30", f30_eg30 as fn()),
        ("f31", f31 as fn()),
        ("f32", f32 as fn()),
    ] {
        println!("\nIn {}():", name);
        f();
    }
}