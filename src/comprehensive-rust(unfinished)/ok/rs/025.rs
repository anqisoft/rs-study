// 025.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/break-continue.html

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("In test1()");
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}

fn test2() {
    println!("\nIn test2()");
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            if i % 2 == 0 {
                i += 1;
                continue;
            }

            println!("x: {x}, i: {i}");
            i += 1;

            if i >= 12 {
                println!("will break outer");
                break 'outer;
            }
        }
    }
}

/* result:
In test1()
x: 10
x: 10, i: 0
x: 10, i: 1
x: 10, i: 2

In test2()
x: 10
x: 10, i: 1
x: 10, i: 3
x: 10, i: 5
x: 10, i: 7
x: 10, i: 9
x: 20
x: 20, i: 1
x: 20, i: 3
x: 20, i: 5
x: 20, i: 7
x: 20, i: 9
x: 20, i: 11
will break outer
*/
