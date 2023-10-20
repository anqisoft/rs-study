// error/007.rs
// https://www.runoob.com/rust/rust-function.html

fn main() {
    let a = (let b = 2);
}

/* result:
error: expected expression, found `let` statement
 --> ..\007.rs:5:14
  |
5 |     let a = (let b = 2);
  |              ^^^

error: expected expression, found statement (`let`)
 --> ..\007.rs:5:14
  |
5 |     let a = (let b = 2);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> ..\007.rs:5:14
  |
5 |     let a = (let b = 2);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> ..\007.rs:5:13
  |
5 |     let a = (let b = 2);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
5 -     let a = (let b = 2);
5 +     let a = let b = 2;
  |

error: aborting due to 3 previous errors; 1 warning emitted
*/
