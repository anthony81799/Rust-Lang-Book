/*
     * Lifetime Elision Rules
     * 
     * 1. Each parameter that is a reference gets its own lifetime parameter
     * 
     * 2. If there is exactly one input lifetime parameter,
     * that lifetime is assigned to all output lifetime parameters
     * 
     * 3. If there are multiple input lifetime parameters,
     * but one of them is &self or &mut self the lifetime of self is
     * assigned to all output lifetime parameters.
    */

use std::fmt::Display;

// the annotation says the struct cannot outlive part
struct ImportantExcerpt<'a> {
    _part: & 'a str
}

impl<'a> ImportantExcerpt<'a> {
    fn _return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self._part
    }
}

fn main() {
    // Lifetimes help deal with dangling references
    // Lifetimes are based on when variables are in scope

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next()
    .expect("Could not find sentence");

    let _i = ImportantExcerpt {
        _part: first_sentence
    };

    // static means the reference can live as long as the program duration
    // all string literals have a static lifetime
    let _s: &'static str = "I have a static lifetime.";

    longest_with_an_announcement(string1.as_str(),
    string2.as_str(), "Hi");
}

// generic lifetime annotations explain the relationship
// between reference lifetimes
// generic lifetimes start with ' and convention is lower case from a
// this example says the result's lifetime is the same as the smallest
// lifetime of the arguments
// the lifetime of the return must be tied to at least one argument
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str, y: &'a str, ann: T
) -> &'a str where T: Display{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
