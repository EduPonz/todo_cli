use crate::app::csv::CSVManager;
use crate::app::parser::Parser;

pub struct List();

impl Parser for List {
    fn print_help() -> () {
        println!("Usage: {} list [options]", env!("CARGO_PKG_NAME"));
        println!("");
        println!("Options:");
        println!("  -h, --help    Print this help message");
    }

    fn parse(args: Vec<String>) -> Result<(), String> {
        let ret;

        if args.len() > 0 {
            // Check list input
            match args[0].as_str() {
                "-h" | "--help" => {
                    Self::print_help();
                    ret = Ok(());
                }
                _ => {
                    Self::print_help();
                    ret = Err("Unknown argument '".to_owned() + &args[0] + "'");
                }
            }
        } else {
            ret = CSVManager::list();
        }

        ret
    }
}
