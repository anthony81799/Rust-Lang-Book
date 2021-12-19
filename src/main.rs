struct User {
    _email: String,
    username: String,
    _sign_in_count: u64,
    _active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// impl is for implimentation blocks
// this is used to impliment methods and traits used by structs and enums
impl Rectangle {
    // the first parameter for every method is self
    // this can be a reference, mut reference or the actual struct
    // the choice depends on if we want the method to take ownership
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Structs can have many impl blocks
impl Rectangle {
    //associated functions do not get the self parameter
    // When used it is similar to a namespace using the ::
    // to use this function type Rectangle::square(size);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}
// Structs have the advantage of names over tuples needing indecies
fn main() {
    let mut user1 = User {
        _email: String::from("example@mail.com"),
        username: String::from("name"),
        _active: true,
        _sign_in_count: 1
    };
    // Structs use dot nootation
    let _name = user1.username;
    user1.username = String::from("new_name");

    let user2 = build_user(
        String::from("example2@mail.com"),
        String::from("name2"));
    let _user3 = User {
        _email: String::from("example3@mail.com"),
        username: String::from("name3"),
        // this line gives the remaining fields in user 3 the values from user2
        ..user2
    };

    // Structs can also be made without field names
    // these are called tuple structs
    struct _Color(i32, i32, i32);
    struct _Point(i32, i32, i32);
    // useful to name tuples and diffentiate similar tuples by type
    // point and color have the same fields, but are diffentt types

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40
    };

    let _rect3 = Rectangle::square(6);

    // the :? specifies print using the debug trait
    // adding the # puts the fields on new lines
    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        // methods are called using dot notation
        rect.area()
    );

    println!("rect can hold rect2? {}!", rect.can_hold(&rect2));
}

fn build_user(_email: String, username: String) -> User{
    User {
        // field init shorthand simplifies setting the provided fields
        // This works because the parameters have the same name as the fields
        _email,
        username,
        _active: true,
        _sign_in_count: 1
    }
}
