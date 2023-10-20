// 005.rs
// https://www.runoob.com/rust/rust-data-types.html

fn main() {
    println!("Hello, Rustaceans! Say by 005.rs.");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup.0 is {}, tup.1 is {}, tup.2 is {}, x is {}, y is {}, z is {}",
        tup.0, tup.1, tup.2, x, y, z
    );

    let a = [1, 2, 3, 4, 5];
    let b = ["January", "February", "March"];
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    let d = [3; 5];
    let first = a[0];
    let second = a[1];
    // a[0] = 123;

    let mut e = [1, 2, 3];
    e[0] = 4;
    let third = e[0];

    println!("a is {:?}, b is {:?}, c is {:?}, d is {:?}, e is {:?}, first is {}, second is {}, third is {}",
        a, b, c, d, e, first, second, third
    );
}

/* result:
error[E0594]: cannot assign to `a[_]`, as `a` is not declared as mutable
  --> ..\005.rs:18:5
   |
18 |     a[0] = 123;
   |     ^^^^^^^^^^ cannot assign
   |
help: consider changing this to be mutable
   |
12 |     let mut a = [1, 2, 3, 4, 5];
   |         +++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0594`.
*/


/* result(after commenting line 18):
Hello, Rustaceans! Say by 005.rs.
tup.0 is 500, tup.1 is 6.4, tup.2 is 1, x is 500, y is 6.4, z is 1
a is [1, 2, 3, 4, 5], b is ["January", "February", "March"], c is [1, 2, 3, 4, 5], d is [3, 3, 3, 3, 3], e is [4, 2, 3], first is 1, second is 2, third is 4
*/