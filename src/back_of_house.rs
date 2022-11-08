fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
    // super 指的是 back_of_house 的上一層，也就是 root crate
}

pub fn cook_order() {
    println!("cook_order()");
}

//-------- pub struct 的範例
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Self {
        Breakfast {
            toast: toast.to_owned(),
            seasonal_fruit: "peaches".to_owned(),
        }
    }
}

//-------- pub enum 的範例
pub enum Appetizer {
    Soup,
    Salad,
}
