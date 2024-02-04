// 032.rs
// https://google.github.io/comprehensive-rust/zh-CN/basic-syntax/scopes-shadowing.html

fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}

/* result:
before: 10
inner scope: hello
shadowed in inner scope: true
after: 10
*/
