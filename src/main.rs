use std::{process, env};
use minigrep2::Config;

fn main() {    
    // parse arguments
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        // print to stderr
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file: {}", config.filename);

    // read from file
    if let Err(e) = minigrep2::run(config) {
        // print to stderr
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}    
