// define module, but get contents im file of same name
mod front_of_house;

// use allows you to bring functions and module into scope
// this is useful for shortening pathes
// self can be used instead of crate to make the path relative
// the idiomatic way to bring functions into scope is to bring the parent
// module into scope
// this shortens pathes, but removes ambiguity
// this way at least the parent must be specified
// for structs and enums use full path uless names would conflict
// adding pub let's us export hosting and its functions
// use also brings in external dependencies
pub use crate::front_of_house::hosting;
pub use self::front_of_house::hosting::add_to_waitlist;
// nested path for different things from same module
// glob operator * brings all public items under module into scope
//use rand::{Rng, CryptoRng, ErrorKind::Transient};
//use std::io::{self, Write};

pub fn eat_at_restaurant() {
    // Absolute function path
    // by default child modules are private
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative function path
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    add_to_waitlist();

    // The breakfast struct can only be made in the summer function
    // this is because Breakfast has a private field
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    //let secret_number = rand::thread_rng().gen_range(1, 101);
}

fn _serve_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        // Super references parent modules
        super::_serve_order();
    }

    fn _cook_order() {}

    // struct fields are private by default
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches")
            }
        }
    }

    // enum variants are public by default
    pub enum Appetizer {
        Soup,
        Salad
    }
}

// In cases of conflict you can assign aliases to a module's types
use std::fmt::Result;
use std::io::Result as IoResult;

fn _function1() -> Result {
    Ok(())
}

fn _function2() -> IoResult<()> {
    Ok(())
}
