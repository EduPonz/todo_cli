use std::fs::OpenOptions;
use std::io::prelude::*;

use crate::app::parser::Parser;

pub struct Adder();

impl Parser for Adder {
    fn print_help() -> () {
        println!(
            "Usage: {} add [options] \"<description>\"",
            env!("CARGO_PKG_NAME")
        );
        println!("");
        println!("Options:");
        println!("  -h, --help    Print this help message");
    }

    fn parse(args: Vec<String>) -> Result<(), String> {
        // Check if there are enough arguments
        if args.len() == 0 {
            Self::print_help();
            std::process::exit(1);
        }

        // Check add input
        match args[0].as_str() {
            "-h" | "--help" => Self::print_help(),
            _ => Self::add(args[0].as_str().to_string()),
        }

        Ok(())
    }
}

impl Adder {
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
