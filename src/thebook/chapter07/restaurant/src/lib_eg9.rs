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

    // error
    // meal.seasonal_fruit = String::from("blueberries");
    /*
        error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
        --> src\lib.rs:23:10
        |
        23 |     meal.seasonal_fruit = String::from("blueberries");
        |          ^^^^^^^^^^^^^^ private field

        For more information about this error, try `rustc --explain E0616`.
    */
}
