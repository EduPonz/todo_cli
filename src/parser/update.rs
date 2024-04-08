use crate::database::csv::CSVManager;
use crate::database::DatabaseManager;
use crate::parser::Parser;

pub struct Updater();

impl Parser for Updater {
    fn print_help() -> () {
        println!(
            "Usage: {} update [-h|--help] <id> [options]",
            env!("CARGO_PKG_NAME")
        );
        println!("");
        println!("Options:");
        println!("  -h, --help                  Print this help message");
        println!("  -d, --description [text]    Update the description");
        println!("                              of the todo item");
        println!("  -s, --status      [status]  Update the status of the todo item");
    }

    fn parse(args: Vec<String>) -> Result<(), String> {
        let mut ret = Ok(());
        let mut id: u64 = 0;

        // Check if there are enough arguments
        if args.len() < 1 {
            Self::print_help();
            // Early return if the user only wants to see the help message
            ret = Err("Not enough arguments".to_string());
        }

        // Get the ID of the item to update
        let mut iter = args.iter();
        let mut arg = iter.next();
        if !ret.is_err() {
            match arg {
                // Case there is an argument to be parsed
                Some(a) => {
                    // First try to parse the argument as an ID
                    id = match a.parse::<u64>() {
                        Ok(i) => i,
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
                                    ret = Err("Invalid ID".to_string());
                                    // Return 0 just to match types. As error is set, this value
                                    // will not have any effect
                                    0
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
        }

        // Getting here without and error means that the user provided a valid ID. First check if
        // there are enough arguments to update the item
        if (!ret.is_err()) && (args.len() < 3) {
            Self::print_help();
            ret = Err("Not enough arguments".to_string());
        }

        // Process the rest of the update arguments
        arg = iter.next();
        while (!ret.is_err()) && (arg != None) {
            match arg.unwrap().as_str() {
                "-h" | "--help" => {
                    Self::print_help();
                    // Early return if the user only wants to see the help message
                    return Ok(());
                }
                "-d" | "--description" => {
                    // Advance the iterator to get the description
                    arg = iter.next();
                    if arg == None {
                        ret = Err("Missing description".to_string());
                        break;
                    }
                    ret = CSVManager::update_description(id, arg.unwrap().to_string());
                    // Advance the iterator to prepare it for the next loop iteration
                    arg = iter.next();
                }
                "-s" | "--status" => {
                    // Advance the iterator to get the status
                    arg = iter.next();
                    if arg == None {
                        ret = Err("Missing status".to_string());
                        break;
                    }
                    ret = CSVManager::update_status(id, arg.unwrap().to_string());
                    // Advance the iterator to prepare it for the next loop iteration
                    arg = iter.next();
                }
                e => {
                    ret = Err(format!("Invalid argument: {}", e));
                }
            }
        }

        if !ret.is_err() {
            println!("Updated todo item with ID {}", id);
        }

        ret
    }
}
