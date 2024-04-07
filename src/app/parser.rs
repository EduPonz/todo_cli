use crate::app::add::Adder;
use crate::app::list::List;
use crate::app::update::Updater;

pub trait Parser {
    fn print_help() -> ();
    fn parse(args: Vec<String>) -> Result<(), String>;
}

pub struct RootParser();

impl RootParser {
    fn print_version() {
        println!(
            "{} version {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }
}

impl Parser for RootParser {
    fn print_help() -> () {
        println!(
            "Usage: {} [options] <verb> [verb_options]",
            env!("CARGO_PKG_NAME")
        );
        println!("");
        println!("Options:");
        println!("  -h, --help    Print this help message");
        println!("  -v, --version Print version information");
        println!("");
        println!("{} verbs:", env!("CARGO_PKG_NAME"));
        println!("  add           Add a new todo item");
        println!("  list          List all todo items");
        println!("  update        Update a todo item");
        println!("  remove        Remove a todo item");
        println!("");
        println!(
            "Run '{} <verb> --help' for more information on a verb.",
            env!("CARGO_PKG_NAME")
        );
    }

    fn parse(args: Vec<String>) -> Result<(), String> {
        // Check if there are enough arguments
        let mut ret = Ok(());

        if args.len() < 2 {
            Self::print_help();
            ret = Err("Not enough arguments".to_string());
        }

        if !ret.is_err() {
            // Check the first argument
            match args[1].as_str() {
                "-h" | "--help" => {
                    Self::print_help();
                    ret = Ok(());
                }
                "-v" | "--version" => {
                    Self::print_version();
                    ret = Ok(());
                }
                "add" => {
                    ret = Adder::parse(args[2..args.len()].to_vec());
                }
                "list" => {
                    ret = List::parse(args[2..args.len()].to_vec());
                }
                "update" => {
                    ret = Updater::parse(args[2..args.len()].to_vec());
                }
                "remove" => {
                    println!("remove is not implemented yet");
                    ret = Ok(());
                }
                _ => {
                    Self::print_help();
                    ret = Err("Unknown verb '".to_owned() + &args[1] + "'");
                }
            }
        }

        ret
    }
}
