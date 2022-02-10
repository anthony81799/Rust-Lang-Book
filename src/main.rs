struct Point {
    x: i32,
    y: i32
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    _Write(String),
    ChangeColor(i32, i32, i32)
}

enum H {
    Hello { id: i32 }
}

fn main() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    let x = Some(5);
    let _y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(_y) => println!("Matched, y = {:?}", _y),
        _ => println!("Default case, x = {:?}", x)
    }

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point {x: 0, y: 7};

    match p {
        Point { x, y: 0 } => {
            println!("On the x axis at {}" , x)
        },
        Point { x:0, y } => {
            println!("On the y axis at {}" , y)
        },
        Point { x, y } => {
            println!("On niether axis at ({}, {})" , x, y)
        },
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::_Quit => {
            println!("Quit");
        }
        Message::_Move { x, y } => {
            println!("Move to x: {} y: {}", x, y);
        }
        Message::_Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color: red: {}, blue: {}, green: {}", r, g, b);
        }
    }

    foo(3, 4);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => ()
    }

    let h = H::Hello { id: 5 };

    match h {
        H::Hello {
            id: id_variable @ 3..=7
        } => println!("Found an id in range: {}", id_variable),
        H::Hello { id: 10..=12 } => println!("Found an id in another range"),
        H::Hello { id } => println!("Found some other id: {}", id)
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the parameter: {}", y);
}
