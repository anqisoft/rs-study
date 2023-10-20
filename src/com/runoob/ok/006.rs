// 006.rs
// https://www.runoob.com/rust/rust-comments.html

use std::io;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let x = add(1, 2);
///
/// ```
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
   
fn main() {
    println!("{}", add(2, 3));

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guessed: {}", guess);

}

/* result:
5
<en>However, documentation comments cannot be displayed in Visual Studio Code.</en>
<zh_cn>但是在Visual Studio Code中无法显示说明文档注释。</zh_cn>
<zh_tw>但是在Visual Studio Code中無法顯示說明檔案注釋。</zh_tw>
*/
