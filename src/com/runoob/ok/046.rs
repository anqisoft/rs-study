// 046.rs
// https://www.runoob.com/rust/rust-error-handle.html

fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

fn main() {
    let r = f(10000);
    if let Ok(v) = r {
        println!("Ok: f(-1) = {}", v);
    } else {
        println!("Err");
    }
}

/* result:
Ok: f(-1) = 10000
*/
