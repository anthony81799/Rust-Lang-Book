// Can use generics with struct and enums
// Can have more than ine generic type
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn _x(&self) -> &T {
        &self.x
    }

    fn _y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    // extracting code duplication to a function
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest is: {}", largest);

    // remove duplication by allowing code to interact with multiple types
    // this is done through generics
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest is: {}", largest);

    let _p1 = Point {x: 5, y: 10};
    let _p2 = Point {x: 5.0, y: 10.0};
    let p3 = Point {x: 5, y: 10.0};

    let p4 = Point {x: "Hello", y: 'c'};
    let p5 = p3.mixup(p4);
    println!("p5.x = {}, p5.y = {}", p5.x, p5.y);
}

// Generics need <T>
// Specify through traits that T must be orderable and copyable
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for elm in list {
        if elm > largest {
            largest = elm;
        }
    }
    largest
}
