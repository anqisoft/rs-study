// 022.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/if-expressions.html

fn main() {
    {
        let mut x = 10;
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        println!("x is {x}");
    }

    {
        let mut x = 10;
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        println!("x is {x}");
    }

    println!("fx(10) is {}", fx(10));
    println!("fx(9) is {}", fx(9));
}

/// <en>-> i32 cannot be omitted, otherwise errors will occur on lines 13 and 14.</en><br>
/// <cn>不可以省略“-> i32”，否则第13、14行会出错。</cn><br>
/// <tw>不可以省略”-> i32“，否則第13、14行會出錯。</tw><br>
fn fx(x: i32) -> i32 { //
    if x % 2 == 0 {
        x / 2
    } else {
        3 * x + 1
    }
}

/* result:
x is 5
x is 5
fx(10) is 5
fx(9) is 28
*/