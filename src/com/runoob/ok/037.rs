// 037.rs
// https://www.runoob.com/rust/rust-project-management.html

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

fn main() {
    eat_at_restaurant()
}

/* result:
I'd like Wheat toast please
*/

/* warnings:
warning: field `seasonal_fruit` is never read
 --> ..\037.rs:7:9
  |
5 |     pub struct Breakfast {
  |                --------- field in this struct
6 |         pub toast: String,
7 |         seasonal_fruit: String,
  |         ^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: 1 warning emitted
*/