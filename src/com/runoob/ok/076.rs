// 076.rs
// 

mod second_076;
use second_076::ClassName;

fn main() {
    let object = ClassName::new(1024);
    object.public_method();
}

/* result:
from public method
from private method
*/

/* warnings
warning: field `field` is never read
 --> ..\second_076.rs:5:5
  |
4 | pub struct ClassName {
  |            --------- field in this struct
5 |     field: i32,
  |     ^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/
