use crate::app::add::Adder;

pub trait Parser {
    fn print_help() -> ();
    fn parse(args: Vec<String>) -> ();
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

    fn parse(args: Vec<String>) -> () {
        // Check if there are enough arguments
        if args.len() < 2 {
            Self::print_help();
            std::process::exit(1);
        }

        // Check the first argument
        match args[1].as_str() {
            "-h" | "--help" => Self::print_help(),
            "-v" | "--version" => Self::print_version(),
            "add" => Adder::parse(args[2..args.len()].to_vec()),
            "list" => println!("list is not implemented yet"),
            "update" => println!("update is not implemented yet"),
            "remove" => println!("remove is not implemented yet"),
            _ => {
                println!("Unknown verb '{}'", args[1]);
                Self::print_help();
                std::process::exit(1);
            }
        }
    }
}
