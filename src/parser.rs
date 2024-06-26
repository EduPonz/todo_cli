pub mod root;

mod add;
mod list;
mod remove;
mod update;

pub trait Parser {
    fn print_help() -> ();
    fn parse(args: Vec<String>) -> Result<(), String>;
}
