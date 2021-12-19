pub fn run() {
    let sum = my_function(11, 22);
    println!("The sum is: {}", sum);
}

// Functions are declared with the fn keyword.
// the function signature follows the format specified below
// fn name(param1: type...) -> return type
// there can be more than one parameter
// if a return type is not provided the function returns nothing
fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // rust will implicitly return the last statement in a function
    // in this form it does not require a semicolon
    // values can also be returned using the return keyword and a semicolon
    x + y
}
