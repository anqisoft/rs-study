// 018.rs
// https://www.runoob.com/rust/rust-ownership.html

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");

    println!("before call takes_and_gives_back(), s1 is {}, s2 is {}", s1, s2);
    let s3 = takes_and_gives_back(s2.clone());
    println!("after call takes_and_gives_back(), s1 is {}, s2 is {}, s3 is {}", s1, s2, s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string;
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/* result:
before call takes_and_gives_back(), s1 is hello, s2 is hello
after call takes_and_gives_back(), s1 is hello, s2 is hello, s3 is hello
*/
