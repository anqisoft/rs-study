// 085.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-2/iterators-and-ownership.html

fn main() {
    test1();
    test2();
    test3();
}

// Ok
fn test1() {
    println!("In test1()");
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }

}

fn test2() {
    println!("\nIn test2()");
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }
    // Repeat again.
    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }
}


fn test3() {
    println!("\nIn test3()");
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        println!("word: {word}");
    }
    // Repeat again.
    for word in &v {
        println!("word: {word}");
    }

    for word in v {
        println!("word: {word}");
    }
    // Repeat again.
    // error[E0382]: use of moved value: `v` --> ..\rs\085.rs:59:17
    // - move occurs because `v` has type `Vec<String>`, which does not implement the `Copy` trait
    // note: `into_iter` takes ownership of the receiver `self`, which moves `v`
    //  - `v` moved due to this implicit call to `.into_iter()` Line 55
    // value used here after move
    // for word in v {
    //     println!("word: {word}");
    // }

    // todo show the type of word!
}

/* result:
In test1()
word: foo
word: bar
word: foo
word: bar

In test2()
word: foo
word: bar
word: foo
word: bar
word: foo
word: bar

In test3()
word: foo
word: bar
word: foo
word: bar
word: foo
word: bar
*/
