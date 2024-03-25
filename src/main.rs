mod app;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    app::App::run(args);
}
