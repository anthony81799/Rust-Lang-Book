// hash map code
use std::collections::HashMap;

pub fn run() {
    // Store key-value pairs placed by a hashing function
    let blue = String::from("blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // the second insert overwrites the first
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);

    // Doesn't overwrite
    //insert only if an entry for the key doesn't exist
    // if it inserts it gives the value provided in the chained or_insert 
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Yellow")).or_insert(40);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
