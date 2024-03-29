use crate::app::parser::Parseable;
use crate::app::App;

pub struct AddParser();

impl Parseable for AddParser {
    fn print_help() -> () {
        println!(
            "Usage: {} add [options] \"<description>\"",
            env!("CARGO_PKG_NAME")
        );
        println!("");
        println!("Options:");
        println!("  -h, --help    Print this help message");
    }

    fn parse(args: Vec<String>) -> () {
        // Check if there are enough arguments
        if args.len() == 0 {
            Self::print_help();
            std::process::exit(1);
        }

        // Check add input
        match args[0].as_str() {
            "-h" | "--help" => Self::print_help(),
            _ => App::add(args[0].as_str().to_string()),
        }
    }
}
