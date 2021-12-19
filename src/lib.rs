// more test options can be found with cargo test --help or
// cargo test -- --help
// cargo test -- show-out shows output for all tests not just failures
// Run subsets with cargo test testname or cargo test name part
// or cargo test modname::
// ignore tests using #[ignore]
// cargo test -- --ignored will run only ignored tests
// test categories are unit and integragration
// unit tests are small and test modules in isolation
// integration tests are separate from the library
// and test the public interface
// covention states unit tests are in the same file as product code
// integration tests go int the test directory at project root

#[derive(Debug)]
pub struct Rectangle {
    _width: u32,
    _height: u32,
}

impl Rectangle {
    pub fn _can_hold(&self, other: &Rectangle) -> bool {
        self._width > other._width && self._height > other._height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    _value: i32
}

impl Guess {
    pub fn new(_value: i32) -> Guess {
        if _value < 1 || _value > 100 {
            panic!("Guess must be between 1 and 100")
        }

        Guess { _value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            _width: 8,
            _height: 7
        };

        let smaller = Rectangle {
            _width: 5,
            _height: 1
        };

        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            _width: 8,
            _height: 7
        };

        let smaller = Rectangle {
            _width: 5,
            _height: 1
        };

        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        // opposite is assert_ne!
        // types must implement partial_eq and debug traits
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // can add custom failure messages
        assert!(result.contains("Carol"),
        "Greeting did not contain name, value was {}", result);
    }

    #[test]
    // add for code that is supposed to panic
    // can panic on specific messages
    // add (expected = "message") after should_panic
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus three does not equal four"))
        }
    }
}
