// 014.rs
// https://www.runoob.com/rust/rust-loop.html

fn main() {
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!("The index of \'O\' is {}", location);
}

/* result:
The index of 'O' is 3
*/
