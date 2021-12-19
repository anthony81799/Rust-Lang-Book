use std::fmt::{Debug, Display};

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle {
    // summarize_author is required because it has no default
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub trait Summary {
    // if you want every type to impliment summary only provide a signature
    // otherwise provide a default implimentation as seen below
    // the above impl block override the default
    fn summarize_author(&self) -> String;
    
    // you can call trait functions within other trait functions
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as ypu probably already know, people"),
        reply: false,
        retweet: false
    }
}

struct _Pair<T> {
    x: T,
    y:T
}

// availible to all Pair<T>
impl<T> _Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self {x ,y}
    }
}

// Only availible when T impliments Display and PartialOrd
impl<T: Display + PartialOrd> _Pair<T>  {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Traits can also be parameters
// below we specifiy item as a reference to anything that impliments 
// the Summary trait
// The below syntax works, but is just syntactic sugar for the below version
// The uncommon version is called a trait bind 
// you can also specify multiple trats
// it goes (item: &impl Summary + trait...)
//pub fn notify(item: &impl Summary) {
//    println!("Breaking news! {}", item.summarize())
//}


// This works for any type that impliments Summary
// This is more useful because if there are multiple arguments
// and they need to be the same type this can be expressed with generics
// you can also specify multiple trats
// it goes <T: Summary + trait...>
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using multiple traits in a bind can hinder readability
// to help with this use the where clause
fn _some_function<T, U>(_t: &T, _u: &U) -> i32
    where T: Display + Clone, U: Clone + Debug {
        //...
        32
    }

fn main() {
    // traits allow us to define a set of methods that are shared between types
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The sky is Falling!"),
        content: String::from("The sky is not actually falling")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    println!("{}", returns_summarizable().summarize());
}
