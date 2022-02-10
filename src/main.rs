fn main() {
    #[derive(Debug)]
    enum Language {
        English,
        _Spanish,
        _Russian,
        _Japanese
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello World!"),
        Language::_Spanish => println!("Hola Mundo!"),
        lang => println!("Unsupported language, {:?}!", lang)
    }

    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status)
    } else if is_admin {
        println!("Authorization status: admin")
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
        println!("Authorization status: priviliged")
        } else {
        println!("Authorization status: basic")
        }
    } else {
        println!("Authorization status: guest")
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let _x = 5;

    let (_x, _y, _z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
