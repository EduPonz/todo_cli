use todo_cli::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match todo_cli::TodoCli::run(args) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
