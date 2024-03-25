use todo::app::App;
use todo::app::Runnable;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    App::run(args);
}
