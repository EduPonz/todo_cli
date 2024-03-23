fn print_help() {
    println!("Usage: todo [-h|--help] [-v|--version] <verb> [options]");
    println!("");
    println!("Options:");
    println!("  -h, --help    Print this help message");
    println!("  -v, --version Print version information");
    println!("");
    println!("todo verbs:");
    println!("  add           Add a new todo item");
    println!("  list          List all todo items");
    println!("  update        Update a todo item");
    println!("  remove        Remove a todo item");
    println!("");
    println!("Run 'todo <verb> --help' for more information on a verb.");
}

fn main() {
    print_help();
}
