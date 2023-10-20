// 013.rs
// https://www.runoob.com/rust/rust-loop.html

fn main() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }
}

/* result:
'R'
'U'
'N'
*/
