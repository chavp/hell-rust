use back_of_house::Breakfast;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub  fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() -> Breakfast {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    
    // use
    hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);
    meal
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod customer {
    use crate::front_of_house::hosting;
    use crate::front_of_house::hosting::seat_at_table;
    use std::collections::HashMap;
    use crate::front_of_house::serving::serve_order as Serve;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        seat_at_table();

        let mut map = HashMap::new();
        map.insert(1, 2);

        Serve();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn run_eat_at_restaurant() {
        let m = eat_at_restaurant();
        assert_eq!(m.toast, "Wheat");
    }
}
