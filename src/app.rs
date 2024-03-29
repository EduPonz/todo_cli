use std::fs::OpenOptions;
use std::io::prelude::*;

mod parser;
use crate::app::parser::Parseable;
use crate::app::parser::Parser;

pub trait Runnable {
    fn run(args: Vec<String>) -> ();
}

pub struct App();

impl Runnable for App {
    fn run(args: Vec<String>) -> () {
        Parser::parse(args);
    }
}

impl App {
    fn add(description: String) -> () {
        println!("Added todo item with description '{}'", description);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("todo.txt")
            .unwrap();

        if let Err(e) = writeln!(file, "{}", description) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}
