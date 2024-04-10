fn server_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        super::server_order();
        crate::server_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn new(toast: String, seasonal_fruit: String) -> Self {
            Self {
                toast,
                seasonal_fruit,
            }
        }

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
