mod my_iter;

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn _shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1 = vec![1, 2, 3];

    // iterators are lazy
    // nothing happens until they are used
    let v1_iter = v1.iter();
    let v1_iter2 = v1.iter();

    for value in v1_iter {
        println!("Got: {}", value);
    }

    // iterators are an implemented trait requireing a defined type Item
    // they also require the function next to be implimented

    let total: i32  = v1_iter2.sum();
    print!("Total: {}", total);

    // iterator functions are adapter or consumers
    // adapters take iterators and return iterators
    // consumers take iterators and return other types

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    my_iter::run();
}
