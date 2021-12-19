fn main() {
    /***************************Ownership Rules********************************/
    // 1. Each value in Rust has a variable as it's owner
    // 2. There can only be one owner at a time
    // 3. When owner goes out of scope the value is dropped

    // a and b are on the stack
    // underscore prefix indicates unused variables
    // their local vars are also on the stack
    fn _a() {
        let _x = "hello";
        let _y = 22;
        _b();
    }

    fn _b() {
        // The value of x is on the heap because String::from() makes a pointer
        let _x = String::from("world");
    }

    {
        // the new scope is defined by the curly braces
        // when the scope ends _s is automatically deallocated
        // this means manual management with new and delete is unnecessary
        let _s = String::from("hello");
    }

    let x = 5;
    let _y = x; // copy x

    let s1 = String::from("hello");
    let s2 = s1; // Move(not shallow copy)
    // s1 is now invalidated so using after declaring _s2 is an error
    // instead to clone s1 call s1.clone()

    take_ownership(s2);
    // using s2 after the function call would be an error
    // passing parameters givess the function ownership of the parameters

    // move semantics are the default in rust
    // however basic data types like ints are copied by default
    // variables set to the return of a function also take ownership
    // this way the value is not dropped when the function goes out of scope

    let s3 = gives_ownership();
    let s4 = String::from("hello");
    let s5 = takes_and_gives_back(s4);
    println!("s3: {}, s4: {}", s3, s5);

    // references allow us to use a variable without taking ownership
    // This is called borrowing. References are immutable by default.
    let len = calculate_length(&s5);
    println!("The length of '{}' is {}.", s5, len);

    let mut s6 = String::from("hello");
    // can only have 1 &mut to a piece of data in aspecific scope
    // can't mix &mut and & because & expects the data to never change
    change(&mut s6);

    /*******************************Slices*************************************/
    let s = String::from("hello world");
    // this declares a slice
    // get a reference to the string, but only the indexes asked for.
    let _hello = &s[..5]; // grabs from begining
    let _world = &s[6..]; // grabs until end
    // omitting both numbers in the range grtabs the whole string
    let s7 = "hello world";

    let word = first_word(s7);
    println!("{}", word);

    // slices work on many types
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[..2];
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len returns length of string
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
