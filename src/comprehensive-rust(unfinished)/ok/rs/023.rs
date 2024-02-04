// 023.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/for-expressions.html

fn main() {
    {
        let v = vec![10, 20, 30];

        for x in v {
            println!("x: {x}");
        }

        for i in (0..10).step_by(2) {
            println!("i: {i}");
        }
    }

    println!("\nNext:");
    {
        let mut v = vec![10, 20, 30];
        v[1] = 21;

        for x in v {
            println!("x: {x}");
        }

        for i in (0..10).step_by(2) {
            println!("i: {i}");
        }
    }

    println!("\nThe end:");
    {
        let mut v = vec![10, 20, 30];
        v[1] = 21;

        for x in v.iter_mut() {
            *x += 5;
            println!("x: {x}");
        }

        for i in (0..10).step_by(2) {
            println!("i: {i}");
        }
    }
}

/* result:
x: 10
x: 20
x: 30
i: 0
i: 2
i: 4
i: 6
i: 8

Next:
x: 10
x: 21
x: 30
i: 0
i: 2
i: 4
i: 6
i: 8

The end:
x: 15
x: 26
x: 35
i: 0
i: 2
i: 4
i: 6
i: 8
*/
