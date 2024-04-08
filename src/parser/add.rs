use crate::database::csv::CSVManager;
use crate::database::DatabaseManager;
use crate::parser::Parser;

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
        let mut ret = Ok(());

        // Check if there are enough arguments
        if args.len() < 1 {
            Self::print_help();
            ret = Err("Not enough arguments".to_string());
        }

        if !ret.is_err() {
            // Check add input
            match args[0].as_str() {
                "-h" | "--help" => {
                    Self::print_help();
                    ret = Ok(());
                }
                _ => {
                    ret = CSVManager::add(args[0].as_str().to_string());
                }
            }
        }

        ret
    }
}
