mod front_of_house;
mod back_of_house;
mod customer;

pub use crate::front_of_house::hosting;

// 把 front_of_house::hosting::* 和 back_of_house::* 都拉進新的 module 中
pub mod facilities {
    pub use super::back_of_house::*;
    pub use super::front_of_house::hosting::*;
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    //-------- pub struct 的範例
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I like {} for toast.", meal.toast);
    // println!("The fruit is {}.", meal.seasonal_fruit);

    //-------- pub enum 的範例
    let _order1 = back_of_house::Appetizer::Salad;
    let _order2 = back_of_house::Appetizer::Soup;

    // 因為有 use，所以現在 hosting 也在這個 scope 中了
    hosting::add_to_waitlist();
}

fn deliver_order() {
    println!("deliver_order()");
}
