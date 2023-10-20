// error/021.rs
// https://www.runoob.com/rust/rust-generics.html

fn max<T>(array: &[T]) -> T {
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

fn main() {
    let a = [2, 4, 6, 3, 1];
    println!("max = {}", max(&a));
}

/* result:
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> ..\021.rs:8:21
  |
8 |         if array[i] > array[max_index] {
  |            -------- ^ ---------------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
4 | fn max<T: std::cmp::PartialOrd>(array: &[T]) -> T {
  |         ++++++++++++++++++++++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0369`.
*/