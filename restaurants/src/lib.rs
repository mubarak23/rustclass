mod front_of_house {
   pub mod hosting {
       pub fn add_to_waitlist(){}

        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        
        fn serve_order(){}

        fn take_payment(){}
    }
}

fn deliver_order(){}

mod back_of_house {

    // STRUCT VISIBLITY

    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // implement the Breakfast Struct Methods
    impl BreakFast {
        pub fn summer( toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

 
    // ENUM VISIBITY
    pub enum Appetizer {
        Soup,
        Salad
    }



    // The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent
    //  module of back_of_house, which in this case is crate, the root. From there, we look for deliver_order and
    //   find it. Success!

    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){}
}

// using the use keyword to shortcut paths
use crate::front_of_house::hosting::add_to_waitlist;

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
//  By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope

mod customer {
    use crate::front_of_house::hosting::add_to_waitlist;

    pub fn eat_my_restaurant() {
       //  hosting::add_to_waitlist();
        add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {

     // Order a breakfast in the summer with Rye toast
     let mut meal = back_of_house::BreakFast::summer("Rye");
     // if let say i change my mind, i want to each pounded yam
     meal.toast = String::from("Pounded Yam");
     println!("I'd like {} toast please", meal.toast);

     // we cant call a private struct field
     // meal.seasonal_fruit = String::from("blueberries");

     let order1 = back_of_house::Appetizer::Soup;
     let order2 = back_of_house::Appetizer::Salad;
   
     // hosting::add_to_waitlist();

        add_to_waitlist();
     // Absolute Path
   //  crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
   //  front_of_house::hosting::add_to_waitlist();
}