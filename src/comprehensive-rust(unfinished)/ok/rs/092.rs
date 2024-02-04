// 092.rs
// https://google.github.io/comprehensive-rust/zh-CN/traits/deriving-traits.html

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default();
    let p2 = p1.clone();
    println!("Is {:?}\nequal to {:?}?\nThe answer is {}!", &p1, &p2,
             if p1 == p2 { "yes" } else { "no" });
}

/* result:
Is Player { name: "", strength: 0, hit_points: 0 }
equal to Player { name: "", strength: 0, hit_points: 0 }?
The answer is yes!
*/
