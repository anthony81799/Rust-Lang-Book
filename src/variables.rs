pub fn run() {
    // Variables are immutable, unchangeable,  by default.
    // the mut keyword makes them mutable
    let x = 5;
    println!("The value of x is: {}", x);
    // redeclaring x is called shadowing
    // shadowing preserves immutability whike allowing for converting types
    let x = "six";
    println!("The value of x is: {}", x);

    // const variables can never change
    // mut is not allowed to be used with const to ensure this
    // const variables must be type annotated
    // the convention for constants is to capilize everything and have words 
    // separated by underscores
    // constants can only be set to constant expressions
    // this means they can not be set to function returns or runtime values
    // number literals can be made more readable by using underscores.
    const  SUBSCRIBER_COUNT: u32 = 100_000;
    println!("Subcriber count is: {}", SUBSCRIBER_COUNT);  
}
