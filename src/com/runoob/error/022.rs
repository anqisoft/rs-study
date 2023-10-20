// error/022.rs
// https://www.runoob.com/rust/rust-lifetime.html

fn main() {
    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
}

/* result:
error[E0597]: `x` does not live long enough
  --> ..\022.rs:10:17
   |
9  |             let x = 5;
   |                 - binding `x` declared here
10 |             r = &x;
   |                 ^^ borrowed value does not live long enough
11 |         }
   |         - `x` dropped here while still borrowed
12 |
13 |         println!("r: {}", r);
   |                           - borrow later used here

error: aborting due to previous error
*/

