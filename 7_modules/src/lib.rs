// use std::fmt;
// //we could do pub to make code calling us use io just as if it brought it to scope
// we can use pub keyword before use to make it accessible from others
// pub use std::io;
// use std::io::Result as IOResult;

// use std::array;
// use std::cmp::Ordering;
// //ORRRRRR
// use std::{array, cmp::Ordering};

// use std::io;
// use std::io::Write;
// //ORRRRR
// use std::io::{self, Write};

////GLOB OPERATOR
// use std::collections::*;
// fn fn1() -> fmt::Result{
// }

// fn fn2() -> io::Result<()>{
// }

//WHEN TWO OF THE SAME RESULT struct IS USED WE NEED TO 'USE' THEIR PARENT
//ORRRRR USE as KEYWORD

mod back_of_house;

//relative use
// use front_of_house::hosting;
//absolute use
// use crate::front_of_house::hosting;

// use crate::front_of_house::hosting::add_to_waitlist;

mod front_of_house;
use crate::front_of_house::hosting;

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
}
