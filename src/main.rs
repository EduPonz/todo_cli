use todo::app::App;
use todo::app::Runnable;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match App::run(args) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}
