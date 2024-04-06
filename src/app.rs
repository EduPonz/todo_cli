use crate::app::parser::Parser;
use crate::app::parser::RootParser;

mod add;
mod csv;
mod list;
mod parser;

pub trait Runnable {
    fn run(args: Vec<String>) -> Result<(), String>;
}

pub struct App();

impl Runnable for App {
    fn run(args: Vec<String>) -> Result<(), String> {
        RootParser::parse(args)
    }
}
