enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    _Alabama,
    Alaska,
    _Arizona,
    _Arkansas,
    _California    
}

pub fn run() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_value = Some(3);
    // if let specifies one pattern and ignores all others
    if let Some(3) = some_value {
        println!("three");
    }
}

fn value_in_cents(c: Coin) -> u8 {
    // match a value based on pattern
    // match requires you handle all possible cases
    // works very well with enums
    // cases longer than one statement require curly braces
    match c {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x:Option<i32>) -> Option<i32> {
    // Using the underscore is a catch all if an arm is not met it falls to _
    // in this instance anything that doesn't meet Some(i) will return None
    match x {
        Some(i) => Some(i + 1),
        _ => None
    }
}
