use crate::app::parser::Parser;
use crate::app::parser::RootParser;

mod add;
mod parser;

pub trait Runnable {
    fn run(args: Vec<String>) -> ();
}

pub struct App();

impl Runnable for App {
    fn run(args: Vec<String>) -> () {
        RootParser::parse(args);
    }
}
