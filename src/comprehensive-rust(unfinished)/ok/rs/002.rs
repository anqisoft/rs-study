// 002.rs
// https://google.github.io/comprehensive-rust/zh-CN/hello-world/small-example.html

fn main() {              // 程序入口
    let mut x: i32 = 6;  // 可变变量绑定
    print!("{x}");       // 与 printf 类似的输出宏
    while x != 1 {       // 表达式周围没有括号
        if x % 2 == 0 {  // 与其他语言类似的数值计算
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}

/* result:
6 -> 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
*/

/* Line5 let mut x: i32 = 6; => let x: i32 = 6;
error[E0384]: cannot assign twice to immutable variable `x`
 --> ..\rs\002.rs:9:13
  |
5 |     let x: i32 = 6;  // 可变变量绑定
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
...
9 |             x = x / 2;
  |             ^^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `x`
  --> ..\rs\002.rs:11:13
   |
5  |     let x: i32 = 6;  // 可变变量绑定
   |         -
   |         |
   |         first assignment to `x`
   |         help: consider making this binding mutable: `mut x`
...
11 |             x = 3 * x + 1;
   |             ^^^^^^^^^^^^^ cannot assign twice to immutable variable

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0384`.
 */

/* Line6 print!("{x}");  => print!("{x} {}");
error: 1 positional argument in format string, but no arguments were given
 --> ..\rs\002.rs:6:17
  |
6 |     print!("{x} {}");       // 与 printf 类似的输出宏
  |                 ^^

error: aborting due to previous error
 */