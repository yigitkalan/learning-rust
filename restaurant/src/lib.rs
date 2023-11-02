use std::fmt;
use std::io;
use std::io::Result as IOResult;

// fn fn1() -> fmt::Result{
// }

// fn fn2() -> io::Result<()>{
// }

//WHEN TWO OF THE SAME RESULT IS USED WE NEED TO 'USE' THEIR PARENT
//ORRRRR USE as KEYWORD


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Salad,
        Soup,
    }

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

    fn fix_wrong_order() {
        cook_order();
        //super is like ../ in linux, one up level of back_of_house is root(create)
        super::front_of_house::serving::serve_order();
    }
    fn cook_order() {}
}

//relative use
// use front_of_house::hosting;
//absolute use
use crate::front_of_house::hosting;
use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // //absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    //
    // //relative path
    // front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    //order a breakfast in summer with french toast
    let mut meal = back_of_house::Breakfast::summer("french");
    //change mind about the toast
    meal.toast = String::from("Rye");

    println!("I'd like {} toast please", meal.toast);

    //gives errors
    // meal.seasonal_fruit = String::from("blueberries");
    //SINCE BREAKFAST HAS PRIVATE FIELDS WE NEED A METHOD THAT CREATES INSTANCES LIKE SUMMER

    hosting::add_to_waitlist();
    add_to_waitlist();
}
