// enums group related data
// only one field is active at  time in each instance
// data can be stored in each variant using parentheses
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum _Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl _Message {
    fn _some_function() {
        println!("let's get Rusty!")
    }
}

struct _IpAddr {
    kind: IpAddrKind,
    address: String
}

pub fn run() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _localhost = IpAddrKind::V4(127, 0, 0, 1);

    // Rust has no null values
    // instead they have Option with fields Some(T) and None
    // Option requirs that we handle the case of none when it occurs
    // objects that can be potentially null must be wrapped in Option
    // Option and its fields are included in program scipe by default
    let _some_number = Some(5);
    let _some_string = Some("a string");

    // Type must be annnotated because no value is passed to infer from
    let _absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // since y is an Option you must extract the int value using unwrap
    // unwrap_or extracts the value or returns the provided default if none
    let _sum = x + y.unwrap_or(0);
}
