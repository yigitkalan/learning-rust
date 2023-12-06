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
