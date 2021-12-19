pub fn run() {
    // Control flow
    let number = 5;
    // conditions must be boolean
    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true"); 
    } else {
        println!("condition was false");
    }

    let condition = true;
    let num2 = if condition { 5 } else { 6 };
    println!("{:?}", (number, num2));

    // Basic loop runs until break is called
    // values may also be returned from this loop type
    let mut  counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };

    println!("Result is: {}", result);

    let mut num3 = 3;
    while num3 != 0 {
        println!("{}!", num3);
        num3 -= 1;
    }
    println!("LIFTOFF!");

    let a = [10, 20, 30, 40, 50];
    // For in loop used to loop through collections
    for e in a.iter() {
        println!("the value is: {}", e);
    }

    // for loops also used on ranges
    // ranges are exclusive
    for num in 1..4 {
        println!("{}!", num);
    }
}
