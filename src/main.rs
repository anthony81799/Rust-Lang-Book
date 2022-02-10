mod smart_pointers;
mod interior_mutability;
mod ref_cycle_list;
mod tree;

// Box can be used to create recursive structures
// This is because Box is a fixed size
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::*;

fn main() {
    // Box is a smart pointer that allocates on the heap
    let b = Box::new(5);
    println!("b = {}", b);

    let list = 
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("list = {:?}", list);

    let x = 5;
    let y = Box::new(x);

    // you can dereference Box because it implements the deref trait
    assert_eq!(5, x);
    assert_eq!(5, *y);

    smart_pointers::run();

    // The smart pointer is coerced into the correct type for hello
    // it coerces from & -> &, &mut -> &mut, and &mut -> &
    let m = smart_pointers::MyBox::new(String::from("Rust"));
    hello(&m);

    ref_cycle_list::run();
    tree::run();
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
