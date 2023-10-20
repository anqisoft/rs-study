// 051.rs
// https://www.runoob.com/rust/rust-generics.html

/*
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn main() {
    a();
    b();
    c();
    d();
}

fn a() {
    println!("In a()");

    fn max1(array: &[i32]) -> i32 {
        let mut max_index = 0;
        let mut i = 1;
        while i < array.len() {
            if array[i] > array[max_index] {
                max_index = i;
            }
            i += 1;
        }
        array[max_index]
    }

    let a = [2, 4, 6, 3, 1];
    println!("max1 = {}", max1(&a));
}

fn b() {
    println!("\nIn b()");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T
    }

    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 1.0, y: 2.0};

    // let p = Point {x: 1, y: 2.0}; // Error
    println!("{:?}, {:?}", p1, p2);
}

fn c() {
    println!("\nIn c()");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f64> {
        fn x_multiply_2(&self) -> f64 {
            self.x * 2.0
        }
    }

    let p = Point { x: 1, y: 2 };
    println!("p.x = {}", p.x());
    // error[E0599]: no method named `x_multiply_2` found for struct `c::Point<{integer}>` in the current scope
    // println!("p.x_multiply_2 = {}", p.x_multiply_2());

    let p1 = Point { x: 1.0, y: 2.0 };
    println!("p1.x_multiply_2 = {}", p1.x_multiply_2());
}

fn d() {
    println!("\nIn d()");

    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point{ x: 1, y: 2 };
    let p2 = Point{ x: 11, y: 12 };
    let p3 = p1.mixup(p2);
    // println!("p1: {:?}", p1); // error[E0382]: borrow of moved value: `p1`
    // println!("p2: {:?}", p2); // error[E0382]: borrow of moved value: `p2`
    println!("p3: {:?}", p3);
}

/* result:
In a()
max1 = 6

In b()
Point { x: 1, y: 2 }, Point { x: 1.0, y: 2.0 }

In c()
p.x = 1
p1.x_multiply_2 = 2

In d()
p3: Point { x: 1, y: 12 }
*/

/* warnings
warning: fields `x` and `y` are never read
  --> ..\051.rs:47:9
   |
46 |     struct Point<T> {
   |            ----- fields in this struct
47 |         x: T,
   |         ^
48 |         y: T
   |         ^
   |
   = note: `Point` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: field `y` is never read
  --> ..\051.rs:64:9
   |
62 |     struct Point<T> {
   |            ----- field in this struct
63 |         x: T,
64 |         y: T,
   |         ^
   |
   = note: `Point` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: 2 warnings emitted
*/