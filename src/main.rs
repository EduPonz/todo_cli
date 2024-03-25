use crate::app::App;
use crate::app::Runnable;

mod app;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    App::run(args);
}
