// 049.rs
// https://google.github.io/comprehensive-rust/zh-CN/pattern-matching/match-guards.html

#[rustfmt::skip]
fn main() {
    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y     => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _                    => println!("No correlation..."),
    }

    println!();
    for pair in [(2, 2), (2, -2), (3, 4), (4, 5)] {
        println!("Tell me about {pair:?}");
        match pair {
            (x, y) if x == y     => println!("These are twins"),
            (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
            (x, _) if x % 2 == 1 => println!("The first one is odd"),
            _                    => println!("No correlation..."),
        }
    }
}

/* result:
Tell me about (2, -2)
Antimatter, kaboom!

Tell me about (2, 2)
These are twins
Tell me about (2, -2)
Antimatter, kaboom!
Tell me about (3, 4)
The first one is odd
Tell me about (4, 5)
No correlation...
*/
