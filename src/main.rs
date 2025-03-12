extern crate rust_grep;

use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = rust_grep::Config::new(&args).unwrap_or_else(|err| {
        
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_grep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
