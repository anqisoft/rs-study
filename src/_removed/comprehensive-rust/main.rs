// fn main() {
//     println!("Hello, world!");
//     println!("Debugging...");
// }

// 034.rs
// https://google.github.io/comprehensive-rust/zh-CN/enums.html

// Execute: cargo add rand
// See: https://www.reddit.com/r/rust/comments/i5syo2/cant_find_crate_when_compiling/?rdt=49892&onetap_auto=true

extern crate rand;
// import commonly used items from the prelude:
use rand::prelude::*;

// fn generate_random_number() -> i32 {
//     // Implementation based on https://xkcd.com/221/
//     4  // Chosen by fair dice roll. Guaranteed to be random.
// }

// https://stackoverflow.com/questions/53985641/using-a-crate-in-a-cargo-project-errors-with-maybe-a-missing-extern-crate
fn generate_random_number() -> i32 {
    // 100 * rand::thread_rng().gen()
    let mut rng = thread_rng();
    rng.gen_range(1..=100)
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    // let random_number = generate_random_number();
    // if random_number % 2 == 0 {
    //     return CoinFlip::Heads;
    // } else {
    //     return CoinFlip::Tails;
    // }

    if generate_random_number() % 2 == 0 { CoinFlip::Heads } else { CoinFlip::Tails } 
}

fn main() {
    println!("You got: {:?}", flip_coin());
}

/* result:
You got: Heads
*/
