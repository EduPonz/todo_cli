use crate::database::csv::CSVManager;
use crate::database::DatabaseManager;
use crate::parser::Parser;

pub struct Remover();

impl Parser for Remover {
    fn print_help() -> () {
        println!("Usage: {} remove [options] <id>", env!("CARGO_PKG_NAME"));
        println!("");
        println!("Options:");
        println!("  -h, --help                  Print this help message");
    }

    fn parse(args: Vec<String>) -> Result<(), String> {
        let ret;

        let arg = args.iter().next();
        match arg {
            // Case there is an argument to be parsed
            Some(a) => {
                // First try to parse the argument as an ID
                match a.parse::<u64>() {
                    Ok(id) => {
                        ret = CSVManager::remove(id);
                        println!("Removed todo item with ID {}", id);
                    }
                    Err(_e) => {
                        // If the argument is not an ID, check if it is a help flag
                        match a.as_str() {
                            "-h" | "--help" => {
                                Self::print_help();
                                // Early return if the user only wants to see the help message
                                return Ok(());
                            }
                            // If the argument is not an ID nor a help flag, return an error
                            _ => {
                                Self::print_help();
                                ret = Err("Invalid ID".to_string());
                            }
                        }
                    }
                };
            }
            // Case there is no argument to be parsed
            None => {
                ret = Err("Not enough arguments".to_string());
            }
        }

        ret
    }
}
