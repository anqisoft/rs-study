// 122.rs
// https://google.github.io/comprehensive-rust/zh-CN/unsafe/unions.html

#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn main() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b });  // Undefined behavior!
}

/* result:
int: 42
bool: true
*/
