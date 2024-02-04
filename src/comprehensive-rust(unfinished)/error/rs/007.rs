// error\007.rs
// https://google.github.io/comprehensive-rust/zh-CN/control-flow/blocks.html

fn main() {
    test1();
    test2();
}

fn test1() {
    println!("In test1()");
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4;
                3 - 4
            };
            println!("w: {w}");
            y * w;
            y * w + 1
        };
        println!("z: {z}");
        z - y;
        z - y + 100
    };
    println!("x: {x}");
}

fn test2() {
    println!("\nIn test2()");
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                // ^ expected `()`, found integer.
                // any code following this expression is unreachable
                return 8;
                3 + 4; // unreachable statement
                3 - 4
            };
            println!("w: {w}");
            return y * y * w;
            y * w
        };
        println!("z: {z}");
        return z - y + 100;
        z - y
    };
    println!("x: {x}");
}
/* result:

*/
