use std::error::Error;
use std::fs::{self, File};
use std::io;

// main can have different return types
fn main() -> Result<(), Box<dyn Error>> {
    // unrecoverable error immediatly crash
    // print provided error message
    // panic!("crash and burn");
    a();

    // Use the Result enum for recoverable errors
    // Result has Ok(T) or Err(E)
    // Result is in scope by defalt
    // unwrap simplifies the below examples
    // at success return the file or panic with error
    // use expect to provide a message to statements expected to fail
    // let f = File::open("hello.txt").expect("failed to open hello.txt");

    /*
    let _f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e)
            },
            other_error => {
                panic!("Problem creating the file: {:?}", other_error)
            }
        }
    };

    // alternative method using closures
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });
    */

    let f = File::open("hello.txt")?;

    Ok(())
}

fn _read_username_from_file() -> Result<String, io::Error> {
    // ? similar to unwrap or expect
    // can only be used where result or option is returned
    // if success return file else return early with error
    // use method chaining to simplify

    fs::read_to_string("hello.txt")

    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    
    //let mut f = match f {
    //    Ok(file) => file,
    //    Err(e) => return  Err(e)
    //};


    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e)
    // }
}

fn a() {
    b();
}

fn b() {
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("error 22");
    }
}
