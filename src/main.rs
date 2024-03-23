fn print_help() {
    println!("Usage: {} [options] <verb> [verb_options]", env!("CARGO_PKG_NAME"));
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
    println!("Run '{} <verb> --help' for more information on a verb.", env!("CARGO_PKG_NAME"));
}

fn print_version() {
    println!("{} version {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

fn parse_arguments(args: Vec<String>) {
    // Check if there are enough arguments
    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }

    // Check the first argument
    match args[1].as_str() {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        "add" => println!("add"),
        "list" => println!("list"),
        "update" => println!("update"),
        "remove" => println!("remove"),
        _ => {
            println!("Unknown verb '{}'", args[1]);
            print_help();
            std::process::exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    parse_arguments(args);
}
