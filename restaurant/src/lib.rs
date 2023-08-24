pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute Path
    // crate::front_of_house::add_to_waitlist()

    // Relative Path
    // front_of_house::add_to_waitlist()
    hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
   
}


// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }
